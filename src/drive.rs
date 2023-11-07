use crate::auth::GToken;
use color_eyre::eyre::Error;
use color_eyre::Result;
use futures_util::StreamExt;
use indicatif::ProgressBar;
use reqwest::multipart::{Form, Part};
use reqwest::{Body, Client};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::str::FromStr;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio_util::io::ReaderStream;

pub struct GFile {
    pub id: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct FileUploadMetadata<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    parents: Option<Vec<&'a str>>,
    name: &'a str,
    mime_type: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    team_drive_id: Option<&'a str>,
}

#[derive(Deserialize)]
struct FileUploadResponse {
    id: String,
}

#[derive(Deserialize)]
pub struct FileDownloadResponse {
    name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct FileUploadQuery<'a> {
    upload_type: &'a str,
    supports_all_drives: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct FileDownloadQuery<'a> {
    supports_all_drives: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    alt: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    acknowledge_abuse: Option<bool>,
}

const FILE_UPLOAD_URL: &str = "https://www.googleapis.com/upload/drive/v3/files";
const FILE_DOWNLOAD_URL: &str = "https://www.googleapis.com/drive/v3/files/";
const MIME_TYPE: &str = "applicatio/octet-stream";

impl GFile {
    pub async fn download_file_metadata(token: &GToken, id: &str) -> Result<FileDownloadResponse> {
        let url = format!("{FILE_DOWNLOAD_URL}{id}");
        let response = Client::new()
            .get(&url)
            .query(&FileDownloadQuery {
                supports_all_drives: true,
                alt: None,
                acknowledge_abuse: None,
            })
            .bearer_auth(
                &token
                    .token()
                    .ok_or(Error::msg("Authorization token has expires".to_string()))?,
            )
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(response)
    }

    pub async fn download(token: &GToken, id: &str, destination_dir: &Path) -> Result<()> {
        let metadata = Self::download_file_metadata(token, id).await?;

        let mut outfile = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&destination_dir.join(metadata.name))
            .await?;

        let url = format!("{FILE_DOWNLOAD_URL}{id}");
        let response = Client::new()
            .get(&url)
            .query(&FileDownloadQuery {
                supports_all_drives: true,
                alt: Some("media"),
                acknowledge_abuse: Some(true),
            })
            .bearer_auth(
                &token
                    .token()
                    .ok_or(Error::msg("Authorization token has expires".to_string()))?,
            )
            .send()
            .await?
            .error_for_status()?;

        let size = u64::from_str(
            response
                .headers()
                .get("Content-Length")
                .ok_or(Error::msg("No Content-Length header in response"))?
                .to_str()?,
        )?;
        let mut stream = response.bytes_stream();

        let mut already_uploaded = 0;
        let progress_bar = ProgressBar::new(size);

        while let Some(chunk) = stream.next().await {
            if let Ok(chunk) = chunk {
                let progress = u64::min(already_uploaded + (chunk.len() as u64), size);
                already_uploaded = progress;
                progress_bar.set_position(progress);

                if already_uploaded >= size {
                    progress_bar.finish();
                }

                outfile.write(chunk.as_ref()).await?;
            }
        }

        Ok(())
    }

    pub async fn upload(
        token: &GToken,
        name: &str,
        parent_folder_id: Option<&str>,
        team_drive_id: Option<&str>,
        file: &Path,
    ) -> Result<Self> {
        let metadata = serde_json::to_string(&FileUploadMetadata {
            parents: parent_folder_id.map(|id| vec![id]),
            name,
            mime_type: MIME_TYPE,
            team_drive_id,
        })?;

        let file = tokio::fs::File::open(file).await?;
        let total_size = file.metadata().await?.len();
        let mut already_uploaded = 0;

        let progress_bar = ProgressBar::new(total_size);

        let mut reader_stream = ReaderStream::new(file);
        let async_stream = async_stream::stream! {
            while let Some(chunk) = reader_stream.next().await {
                if let Ok(chunk) = &chunk {
                    let new = u64::min(already_uploaded + (chunk.len() as u64), total_size);
                    already_uploaded = new;
                    progress_bar.set_position(new);
                    if already_uploaded >= total_size {
                        progress_bar.finish();
                    }
                }

                yield chunk;
             }
        };

        let body = Body::wrap_stream(async_stream);

        let multipart = Form::new()
            .part(
                "metadata",
                Part::text(metadata).mime_str("application/json; charset=UTF-8")?,
            )
            .part("media", Part::stream(body).mime_str(MIME_TYPE)?);

        let response: FileUploadResponse = Client::new()
            .post(FILE_UPLOAD_URL)
            .query(&FileUploadQuery {
                upload_type: "multipart",
                supports_all_drives: true,
            })
            .bearer_auth(
                &token
                    .token()
                    .ok_or(Error::msg("Authorization token has expires".to_string()))?,
            )
            .multipart(multipart)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(Self { id: response.id })
    }
}

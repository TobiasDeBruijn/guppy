use std::path::Path;
use color_eyre::eyre::Error;
use crate::auth::GToken;
use color_eyre::Result;
use reqwest::{Body, Client};
use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};
use tokio_util::codec::{BytesCodec, FramedRead};

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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct FileUploadQuery<'a> {
    upload_type: &'a str,
    supports_all_drives: bool,
}

const FILE_UPLOAD_URL: &str = "https://www.googleapis.com/upload/drive/v3/files";
const MIME_TYPE: &str = "applicatio/octet-stream";

impl GFile {
    pub async fn upload(token: &GToken, name: &str, parent_folder_id: Option<&str>, team_drive_id: Option<&str>, file: &Path) -> Result<Self> {
        let metadata = serde_json::to_string(&FileUploadMetadata {
            parents: parent_folder_id.map(|id| vec![id]),
            name,
            mime_type: MIME_TYPE,
            team_drive_id,
        })?;

        let file = tokio::fs::File::open(file).await?;
        let stream = FramedRead::new(file, BytesCodec::new());
        let body = Body::wrap_stream(stream);

        let multipart = Form::new()
            .part(
                "metadata",
                Part::text(metadata)
                        .mime_str("application/json; charset=UTF-8")?
            )
            .part(
                "media",
                Part::stream(body)
                        .mime_str(MIME_TYPE)?
            );

        let response: FileUploadResponse = Client::new()
            .post(FILE_UPLOAD_URL)
            .query(&FileUploadQuery {
                upload_type: "multipart",
                supports_all_drives: true,
            })
            .bearer_auth(&token.token()
                .ok_or(Error::msg("Authorization token has expires".to_string()))?
            )
            .multipart(multipart)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(Self {
            id: response.id,
        })
    }
}
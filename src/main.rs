use crate::args::{DownloadCommand, ProgramArgs, SubCommand, UploadCommand};
use crate::auth::{GToken, ServiceAccount};
use crate::drive::GFile;
use clap::Parser;
use color_eyre::eyre::Error;
use color_eyre::Result;
use tokio::io::AsyncReadExt;

mod args;
mod auth;
mod drive;

const APPLICATION_OAUTH_SCOPES: &str = "https://www.googleapis.com/auth/drive";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let args = ProgramArgs::parse();

    let mut pem_file = tokio::fs::File::open(&args.pem).await?;
    let mut pem = String::new();
    pem_file.read_to_string(&mut pem).await?;

    let token = GToken::create(&ServiceAccount {
        private_key_pem: pem,
        email: args.email,
        scopes: APPLICATION_OAUTH_SCOPES.to_string(),
    })
    .await?;

    match args.subcommand {
        SubCommand::Upload(upload_args) => upload(&upload_args, &token).await?,
        SubCommand::Download(download_args) => download(&download_args, &token).await?,
    }

    Ok(())
}

async fn upload(upload: &UploadCommand, token: &GToken) -> Result<()> {
    let file = GFile::upload(
        &token,
        upload
            .source
            .file_name()
            .ok_or(Error::msg("Provided file does not have a name".to_string()))?
            .to_str()
            .ok_or(Error::msg(
                "Provided file name could not be converted to UTF-8".to_string(),
            ))?,
        upload.folder.as_deref(),
        upload.team_drive.as_deref(),
        &upload.source,
    )
    .await?;

    println!("{}", file.id);
    Ok(())
}

async fn download(download: &DownloadCommand, token: &GToken) -> Result<()> {
    GFile::download(token, &download.source, &download.destination_dir).await
}

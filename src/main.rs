use clap::Parser;
use color_eyre::Result;
use color_eyre::eyre::Error;
use tokio::io::AsyncReadExt;
use crate::args::Args;
use crate::auth::{GToken, ServiceAccount};
use crate::drive::GFile;

mod drive;
mod auth;
mod args;

const APPLICATION_OAUTH_SCOPES: &str = "https://www.googleapis.com/auth/drive";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    let mut pem_file = tokio::fs::File::open(&args.pem).await?;
    let mut pem = String::new();
    pem_file.read_to_string(&mut pem).await?;

    let token = GToken::create(&ServiceAccount {
        private_key_pem: pem,
        email: args.email,
        scopes: APPLICATION_OAUTH_SCOPES.to_string(),
    }).await?;

    let file = GFile::upload(
        &token,
        args.file.file_name()
            .ok_or(Error::msg("Provided file does not have a name".to_string()))?
            .to_str()
            .ok_or(Error::msg("Provided file name could not be converted to UTF-8".to_string()))?,
        args.folder.as_deref(),
        args.team_drive.as_deref(),
        &args.file
    ).await?;

    println!("{}", file.id);

    Ok(())
}
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct ProgramArgs {
    #[arg(short, long)]
    pub pem: PathBuf,
    #[arg(short, long)]
    pub email: String,
    #[clap(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    Upload(UploadCommand),
    Download(DownloadCommand),
}

#[derive(Debug, Args)]
pub struct UploadCommand {
    #[arg(long, short)]
    pub source: PathBuf,
    #[arg(long, short)]
    pub folder: Option<String>,
    #[arg(long, short)]
    pub team_drive: Option<String>,

    /// Optionally, supply a Discord webhook to be called when an upload is done
    #[arg(long)]
    pub success_webhook: Option<String>,
}

#[derive(Debug, Args)]
pub struct DownloadCommand {
    #[arg(long, short)]
    pub source: String,
    #[arg(long, short)]
    pub destination_dir: PathBuf,
}

use std::path::PathBuf;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(short, long)]
    pub pem: PathBuf,
    #[arg(short, long)]
    pub email: String,
    #[arg(long, short)]
    pub folder: Option<String>,
    #[arg(long, short)]
    pub team_drive: Option<String>,

    pub file: PathBuf,
}

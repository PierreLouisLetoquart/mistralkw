pub mod generation;
pub mod model;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub model: String,

    #[arg(short, long)]
    pub document: PathBuf,

    #[arg(short, long)]
    pub output: Option<PathBuf>,
}

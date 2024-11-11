use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct CliParams {
    #[arg(short, long)]
    rom: PathBuf,
}

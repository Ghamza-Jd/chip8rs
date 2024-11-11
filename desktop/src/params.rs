use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct CliParams {
    #[arg(short, long)]
    pub rom: PathBuf,
    #[arg(short, long, default_value_t = 15)]
    pub scale: u32,
}

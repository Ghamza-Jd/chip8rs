mod params;

use chip8_core::emu::Emu;
use clap::Parser;
use params::CliParams;

fn main() -> anyhow::Result<()> {
    let params = CliParams::try_parse()?;
    _ = Emu::new();
    Ok(())
}

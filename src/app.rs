// orkestrasi aplikasi
use anyhow::{Result, bail};

use crate::cli::{Cli, Commands};
use crate::jobs::example;

pub fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Run { job } => match job.as_str() {
            "example" => example::run()?,
            _ => bail!("Unknown job: {job}"),
        },
    }

    Ok(())
}

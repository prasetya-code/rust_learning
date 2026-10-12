// definisi perintah
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "rust-automate",
    version,
    about = "A modular automation toolkit"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Menjalankan contoh automation
    Run {
        /// Nama job yang akan dijalankan
        #[arg(long, default_value = "example")]
        job: String,
    },
}

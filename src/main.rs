// entry point
// mengacu pada biner file name di Cargo.toml atau main name package::nama fungsi harus publik

use anyhow::Result;
use clap::Parser;

use rust_learning::app;
use rust_learning::cli::Cli;
use rust_learning::logging;

fn main() -> Result<()> {
    logging::init();

    let cli = Cli::parse();

    app::run(cli)
}

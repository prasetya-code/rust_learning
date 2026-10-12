use clap::Parser;
use rust_automate::cli::{Cli, Commands};

#[test]
fn parses_run_command() {
    let cli = Cli::try_parse_from([
        "rust-automate",
        "run",
        "--job",
        "example",
    ])
    .expect("CLI arguments should be valid");

    match cli.command {
        Commands::Run { job } => {
            assert_eq!(job, "example");
        }
    }
}
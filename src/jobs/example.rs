// implementasi job pertama
use anyhow::Result;
use tracing::info;

pub fn run() -> Result<()> {
    info!("Starting example automation");

    println!("Automation is running!");

    info!("Example automation completed");

    Ok(())
}

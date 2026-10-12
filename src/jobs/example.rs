// implementasi job pertama
use anyhow::Result;
use tracing::info;

pub fn run() -> Result<()> {
    info!("Starting example automation \n");

    println!("Automation is running! \n");

    info!("Example automation completed");

    Ok(())
}

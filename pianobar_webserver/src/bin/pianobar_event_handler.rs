use anyhow::Result;
use log;

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    log::info!("AAAAA");

    Ok(())
}

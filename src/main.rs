use std::env;

use eyre::eyre;

fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    let url = env::args().nth(1).ok_or_else(|| eyre!("No URL provided"))?;

    wbe::load(&url)?;

    Ok(())
}

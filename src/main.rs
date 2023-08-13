use std::env;

fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    let url = if let Some(url) = env::args().nth(1) {
        url
    } else {
        let mut path = std::env::current_dir()?;
        path.push("example.html");
        let path = path.to_str().expect("non-utf8 working dir");
        format!("file://{}", path)
    };

    wbe::load(&url)?;

    Ok(())
}

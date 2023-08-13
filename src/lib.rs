mod request;
mod url;

use ascii::AsAsciiStr;
use eyre::Context;

pub use request::request;
pub use url::URL;

fn show(body: &str) {
    let mut in_angle = false;
    for c in body.chars() {
        if c == '<' {
            in_angle = true;
        } else if c == '>' {
            in_angle = false;
        } else if !in_angle {
            print!("{c}");
        }
    }
}

pub fn load(url: &str) -> eyre::Result<()> {
    let url = url.as_ascii_str().wrap_err("url")?;
    let url = URL::parse(url)?;
    let resp = request(&url).wrap_err("request")?;
    show(&resp.body);
    Ok(())
}

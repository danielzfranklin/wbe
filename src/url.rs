use std::sync::OnceLock;

use ascii::{AsAsciiStr, AsciiStr, AsciiString, IntoAsciiString};
use eyre::{eyre, Context};
use regex::bytes::Regex;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct URL {
    pub scheme: Scheme,
    pub host: AsciiString,
    pub port: Option<u16>,
    pub path: AsciiString,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Scheme {
    HTTP,
    HTTPS,
    File,
}

impl URL {
    pub fn parse(s: &AsciiStr) -> eyre::Result<URL> {
        static RE: OnceLock<Regex> = OnceLock::new();
        let re = RE.get_or_init(|| {
            Regex::new(
                r"(?x)
                ^
                (?P<scheme>.+)
                ://
                (?P<host>[^:/?\#]*)
                (:(?P<port>\d+))?
                (?P<path>/.*)?
                $
                    ",
            )
            .unwrap()
        });

        let caps = re
            .captures(s.as_bytes())
            .ok_or_else(|| eyre!("malformed or unsupported URL: {}", s))?;
        tracing::trace!(?caps, "matched URL regex");

        // We know this is valid ASCII because the input is
        let scheme = match &caps["scheme"] {
            b"http" => Scheme::HTTP,
            b"https" => Scheme::HTTPS,
            b"file" => Scheme::File,
            scheme => {
                return Err(eyre!(
                    "unsupported scheme: {}",
                    scheme.as_ascii_str().unwrap()
                ))
            }
        };
        let host = caps["host"].as_ascii_str().unwrap().to_owned();
        let path = if let Some(path) = caps.name("path") {
            path.as_bytes().as_ascii_str().unwrap().to_owned()
        } else {
            "/".into_ascii_string().unwrap()
        };
        let port = if let Some(port) = caps.name("port") {
            let port = port.as_bytes().as_ascii_str().unwrap();
            let port = port.as_str().parse::<u16>().wrap_err("port")?;
            Some(port)
        } else {
            None
        };

        let url = URL {
            scheme,
            host,
            path,
            port,
        };
        tracing::trace!(?url, "parsed URL");

        Ok(url)
    }
}

use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    net::ToSocketAddrs,
};

use eyre::{eyre, Context};

use socket2::Socket;

use crate::URL;

pub struct Response {
    headers: HashMap<String, String>,
    pub body: String,
}

pub fn request(url: &URL) -> eyre::Result<Response> {
    // TODO: Skipped TLS support

    let port = url.port.unwrap_or(80);
    let addr = (url.host.as_str(), port)
        .to_socket_addrs()?
        .next()
        .expect("parses to one addr");
    tracing::trace!(?addr);

    let s = Socket::new(
        socket2::Domain::IPV4,
        socket2::Type::STREAM,
        Some(socket2::Protocol::TCP),
    )?;

    s.connect(&addr.into()).wrap_err("connect")?;
    tracing::trace!("connected");

    let req = format!(
        "GET {path} HTTP/1.0\r\nHost: {host}\r\n\r\n",
        path = url.path,
        host = url.host.as_str()
    );
    tracing::trace!(?req);
    let req = req.as_bytes();
    if s.send(req).wrap_err("send")? != req.len() {
        return Err(eyre!("failed to send entire request"));
    }
    tracing::trace!("sent request");

    let mut resp = BufReader::new(s);

    let mut statusline = String::new();
    resp.read_line(&mut statusline)
        .wrap_err("read status line")?;
    tracing::trace!(?statusline);
    let mut parts = statusline.splitn(3, ' ');

    let _version = parts
        .next()
        .ok_or_else(|| eyre!("expected status version"))?;
    let status = parts.next().ok_or_else(|| eyre!("expected status code"))?;
    let explanation = parts
        .next()
        .ok_or_else(|| eyre!("expected status explanation"))?;
    tracing::trace!(?_version, ?status, ?explanation);

    if status != "200" {
        return Err(eyre!("status {status}: {explanation}"));
    }

    let mut headers = HashMap::new();
    let mut header_line = String::new();
    loop {
        header_line.clear();
        resp.read_line(&mut header_line)
            .wrap_err("read header line")?;
        tracing::trace!(?header_line);

        if header_line == "\r\n" {
            break;
        }

        let mut parts = header_line.splitn(2, ':');

        let key = parts
            .next()
            .ok_or_else(|| eyre!("expected header key"))?
            .to_ascii_lowercase();

        let value = parts
            .next()
            .ok_or_else(|| eyre!("expected header value"))?
            .trim()
            .to_string();

        tracing::trace!(?key, ?value);
        headers.insert(key, value);
    }
    tracing::trace!("finished reading headers");

    if headers.contains_key("transfer-encoding") {
        return Err(eyre!("transfer-encoding not supported"));
    }
    if headers.contains_key("content-encoding") {
        return Err(eyre!("content-encoding not supported"));
    }

    let mut body = String::new();
    resp.read_to_string(&mut body).wrap_err("read body")?;
    tracing::trace!("read body");

    Ok(Response { headers, body })
}

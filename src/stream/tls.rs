use std::{net::IpAddr, ops::Deref, sync::LazyLock};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};

static RT: LazyLock<tokio::runtime::Runtime> =
    LazyLock::new(|| tokio::runtime::Runtime::new().unwrap());

pub(super) struct Stream {
    inner: tokio_native_tls::TlsStream<tokio::net::TcpStream>,
}

impl Stream {
    pub fn connect(domain: &str, addr: std::net::SocketAddr) -> eyre::Result<Self> {
        RT.block_on(async move {
            let connector = native_tls::TlsConnector::new()?;
            let connector = tokio_native_tls::TlsConnector::from(connector);

            let stream = tokio::net::TcpStream::connect(addr).await?;
            let stream = connector.connect(domain, stream).await?;

            Ok(Self { inner: stream })
        })
    }

    pub fn into_buf_reader(self) -> BufReader {
        BufReader {
            inner: tokio::io::BufReader::new(self.inner),
        }
    }
}

impl std::io::Read for Stream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        RT.block_on(self.inner.read(buf))
    }
}

impl std::io::Write for Stream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        RT.block_on(self.inner.write(buf))
    }

    fn write_all(&mut self, mut buf: &[u8]) -> std::io::Result<()> {
        RT.block_on(self.inner.write_all(&mut buf))
    }

    fn flush(&mut self) -> std::io::Result<()> {
        RT.block_on(self.inner.flush())
    }
}

pub(super) struct BufReader {
    inner: tokio::io::BufReader<tokio_native_tls::TlsStream<tokio::net::TcpStream>>,
}

impl std::io::Read for BufReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        RT.block_on(self.inner.read(buf))
    }
}

impl std::io::BufRead for BufReader {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        RT.block_on(self.inner.fill_buf())
    }

    fn consume(&mut self, amt: usize) {
        self.inner.consume(amt)
    }
}

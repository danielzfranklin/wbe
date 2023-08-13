use std::net::SocketAddr;

mod tls;

pub struct Stream(Inner);

enum Inner {
    Tcp(std::net::TcpStream),
    Tls(tls::Stream),
}

pub struct BufReader(InnerBufReader);

enum InnerBufReader {
    Tcp(std::io::BufReader<std::net::TcpStream>),
    Tls(tls::BufReader),
}

impl Stream {
    pub fn tcp_connect(addr: SocketAddr) -> eyre::Result<Self> {
        let inner = std::net::TcpStream::connect(addr)?;
        Ok(Self(Inner::Tcp(inner)))
    }

    pub fn tls_connect(domain: &str, addr: SocketAddr) -> eyre::Result<Self> {
        let inner = tls::Stream::connect(domain, addr)?;
        Ok(Self(Inner::Tls(inner)))
    }

    pub fn into_buf_reader(self) -> eyre::Result<BufReader> {
        match self.0 {
            Inner::Tcp(inner) => {
                let inner = std::io::BufReader::new(inner);
                Ok(BufReader(InnerBufReader::Tcp(inner)))
            }
            Inner::Tls(inner) => {
                let inner = inner.into_buf_reader();
                Ok(BufReader(InnerBufReader::Tls(inner)))
            }
        }
    }
}

impl std::io::Read for Stream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match &mut self.0 {
            Inner::Tcp(inner) => inner.read(buf),
            Inner::Tls(inner) => inner.read(buf),
        }
    }
}

impl std::io::Write for Stream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match &mut self.0 {
            Inner::Tcp(inner) => inner.write(buf),
            Inner::Tls(inner) => inner.write(buf),
        }
    }
    fn flush(&mut self) -> std::io::Result<()> {
        match &mut self.0 {
            Inner::Tcp(inner) => inner.flush(),
            Inner::Tls(inner) => inner.flush(),
        }
    }
    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        match &mut self.0 {
            Inner::Tcp(inner) => inner.write_all(buf),
            Inner::Tls(inner) => inner.write_all(buf),
        }
    }
}

impl std::io::Read for BufReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match &mut self.0 {
            InnerBufReader::Tcp(inner) => inner.read(buf),
            InnerBufReader::Tls(inner) => inner.read(buf),
        }
    }
}

impl std::io::BufRead for BufReader {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        match &mut self.0 {
            InnerBufReader::Tcp(inner) => inner.fill_buf(),
            InnerBufReader::Tls(inner) => inner.fill_buf(),
        }
    }

    fn consume(&mut self, amt: usize) {
        match &mut self.0 {
            InnerBufReader::Tcp(inner) => inner.consume(amt),
            InnerBufReader::Tls(inner) => inner.consume(amt),
        }
    }
}

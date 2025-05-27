use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use crate::client_error::ClientError;
use tl_common::Result;

pub(crate) struct Socks5Server {
    conn: TcpStream,
}

impl Socks5Server {
    pub fn new(conn: TcpStream) -> Socks5Server {
        Socks5Server { conn: conn }
    }

    pub async fn wait_readable(&mut self) -> Result<()> {
        self.conn.readable().await?;
        Ok(())
    }

    pub fn try_read(&mut self, buf: &mut [u8]) -> Result<usize> {
        Ok(self.conn.try_read(buf)?)
    }

    pub async fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        self.conn.write_all(buf).await?;
        Ok(())
    }

    pub async fn method_select(&mut self) -> Result<()> {
        let mut buf: Vec<u8> = vec![0; 256];

        let b = &mut buf[..2];
        self.conn.read_exact(b).await?;

        let version = b[0];
        let methods_bytes = b[1];

        // check version
        if version != 0x05 {
            return Err(Box::new(ClientError::Socks5VersionInvalid));
        }

        // discard methods
        let b = &mut buf[..methods_bytes.into()];
        self.conn.read_exact(b).await?;

        // answer server accepted method
        self.conn.write_all(&[0x05, 0x00]).await?;

        Ok(())
    }

    pub async fn receive_dst_addr(&mut self) -> Result<String> {
        let mut buf: Vec<u8> = vec![0; 256];

        let b = &mut buf[..4];
        self.conn.read_exact(b).await?;

        let version = b[0];
        let cmd = b[1];
        let addr_type = b[3];

        // check version
        if version != 0x05 {
            return Err(Box::new(ClientError::Socks5VersionInvalid));
        }
        // only support connect command
        if cmd != 0x01 {
            return Err(Box::new(ClientError::Socks5CmdNotSupported));
        }

        if addr_type == 0x01 {
            // ipv4
            let b = &mut buf[..6];
            self.conn.read_exact(b).await?;

            let port: u16 = ((b[4] as u16) << 8) + b[5] as u16;
            let addr = format!("{}.{}.{}.{}:{}", b[0], b[1], b[2], b[3], port);

            return Ok(addr);
        } else if addr_type == 0x03 {
            // domain
            let b = &mut buf[..1];
            self.conn.read_exact(b).await?;
            let domain_length = b[0] as usize;

            let b = &mut buf[..(domain_length + 2)];
            self.conn.read_exact(b).await?;
            let domain = std::str::from_utf8(&b[..domain_length])?;
            let port: u16 =
                ((b[domain_length] as u16) << 8) + b[domain_length + 1] as u16;
            let addr = format!("{}:{}", domain, port);

            return Ok(addr);
        } else {
            return Err(Box::new(ClientError::Socks5AddrTypeNotSupported));
        }
    }

    pub async fn notify_connect_success(&mut self) -> Result<()> {
        self.conn
            .write_all(&[
                0x05, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ])
            .await?;

        Ok(())
    }
}

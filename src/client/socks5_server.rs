use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use crate::common::Result;
use crate::client_error::ClientError;

pub(crate) struct Socks5Server {
    conn: TcpStream,
}

impl Socks5Server {
    pub fn new(conn: TcpStream) -> Socks5Server {
        Socks5Server {
            conn: conn,
        }
    }

    pub async fn method_select(&mut self) -> Result<()> {
        let mut b: Vec<u8> = vec![0; 2];
        self.conn.read_exact(&mut b).await?;

        let version = b[0];
        let methods_bytes = b[1];

        // check version
        if version != 0x05 {
            return Err(Box::new(ClientError::Socks5VersionInvalid));
        }

        // discard methods
        let mut b: Vec<u8> = vec![0; methods_bytes.into()];
        self.conn.read_exact(&mut b).await?;

        // answer server accepted method
        self.conn.write_all(&[0x05, 0x00]).await?;

        Ok(())
    }

    pub async fn receive_dst_addr(&mut self) -> Result<String> {
        let mut b: Vec<u8> = vec![0; 4];
        self.conn.read_exact(&mut b).await?;

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
            let mut b: Vec<u8> = vec![0; 6];
            self.conn.read_exact(&mut b).await?;

            let port: u16 = ((b[4] as u16) << 8) + b[5] as u16;
            let addr = format!("{}.{}.{}.{}:{}", b[0], b[1], b[2], b[3], port);

            return Ok(addr);
        }

        Ok(String::new())
    }
}

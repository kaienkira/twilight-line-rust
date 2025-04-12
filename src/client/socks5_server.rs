use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

use crate::common::Result;

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
        let mut b: [u8; 2] = [0; 2];
        self.conn.read_exact(&mut b).await?;
        Ok(())
    }
}

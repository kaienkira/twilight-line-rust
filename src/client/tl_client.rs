use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use crate::common::Result;

pub(crate) struct TlClient {
    conn: TcpStream,
    sec_key: &'static str,
    fake_request: &'static [u8],
    fake_response: &'static [u8],
    comm_key: Vec<u8>,
}

impl TlClient {
    pub fn new(
        conn: TcpStream,
        sec_key: &'static str,
        fake_request: &'static [u8],
        fake_response: &'static [u8])
        -> TlClient {
        TlClient {
            conn: conn,
            sec_key: sec_key,
            fake_request: fake_request,
            fake_response: fake_response,
            comm_key: Vec::new(),
        }
    }

    fn reset_cipher(&mut self, key: &[u8]) {
    }

    pub async fn connect(&mut self, dst_addr: &str) -> Result<()> {
        self.conn.write_all(self.fake_request).await?;

        Ok(())
    }

}

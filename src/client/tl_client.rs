use cfb_mode::cipher::KeyIvInit;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use tl_common::Result;

type Aes256CfbEncoder = cfb_mode::BufEncryptor<aes::Aes256>;
type Aes256CfbDecoder = cfb_mode::BufDecryptor<aes::Aes256>;

pub(crate) struct TlClient {
    conn: TcpStream,
    sec_key: &'static str,
    fake_request: &'static [u8],
    fake_response: &'static [u8],
    comm_key: Vec<u8>,
    encoder: Aes256CfbEncoder,
    decoder: Aes256CfbDecoder,
}

impl TlClient {
    pub fn new(
        conn: TcpStream,
        sec_key: &'static str,
        fake_request: &'static [u8],
        fake_response: &'static [u8])
        -> TlClient {
        let (encoder, decoder) = Self::create_cipher(sec_key.as_bytes());

        TlClient {
            conn: conn,
            sec_key: sec_key,
            fake_request: fake_request,
            fake_response: fake_response,
            comm_key: Vec::new(),
            encoder: encoder,
            decoder: decoder,
        }
    }

    fn create_cipher(key: &[u8]) -> (Aes256CfbEncoder, Aes256CfbDecoder) {
        let aes_key: Vec<u8> = tl_common::util::sha256_sum(key);
        let iv: Vec<u8> = vec![0; 16];
        let encoder = Aes256CfbEncoder::new(
            aes_key.as_slice().into(), iv.as_slice().into());
        let decoder = Aes256CfbDecoder::new(
            aes_key.as_slice().into(), iv.as_slice().into());

        (encoder, decoder)
    }

    pub async fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let n = self.conn.read(buf).await?;
        self.decoder.decrypt(buf[..n]);

        Ok(n)
    }

    pub async fn connect(&mut self, dst_addr: &str) -> Result<()> {
        self.conn.write_all(self.fake_request).await?;

        Ok(())
    }

}

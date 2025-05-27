use bytes::BufMut;
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
        fake_response: &'static [u8],
    ) -> TlClient {
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
            aes_key.as_slice().into(),
            iv.as_slice().into(),
        );
        let decoder = Aes256CfbDecoder::new(
            aes_key.as_slice().into(),
            iv.as_slice().into(),
        );

        (encoder, decoder)
    }

    pub async fn wait_readable(&mut self) -> Result<()> {
        self.conn.readable().await?;
        Ok(())
    }

    pub fn try_read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self.conn.try_read(buf) {
            Ok(n) => {
                self.decoder.decrypt(&mut buf[..n]);
                return Ok(n);
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    pub async fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        self.conn.read_exact(buf).await?;
        self.decoder.decrypt(buf);

        Ok(())
    }

    pub async fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        let mut buf_encode = buf.to_vec();
        self.encoder.encrypt(buf_encode.as_mut_slice());
        self.conn.write_all(buf_encode.as_slice()).await?;

        Ok(())
    }

    pub async fn connect(&mut self, dst_addr: &str) -> Result<()> {
        // write fake request
        self.conn.write_all(self.fake_request).await?;

        // write request addr
        {
            let mut buf: Vec<u8> = Vec::with_capacity(2048);
            let sign: Vec<u8> = tl_common::util::sha256_sum(
                format!("{}{}", dst_addr, self.sec_key).as_bytes(),
            );
            buf.put_u16(dst_addr.len() as u16);
            buf.put(dst_addr.as_bytes());
            buf.put(sign.as_slice());
            self.write_all(buf.as_slice()).await?;
        }

        // read fake response
        {
            let mut buf: Vec<u8> = vec![0; self.fake_response.len()];
            self.conn.read_exact(buf.as_mut_slice()).await?;
        }

        // create communication key
        {
            let mut buf: Vec<u8> = vec![0; 256];

            let b = &mut buf[..1];
            self.read_exact(b).await?;
            let comm_key_len = b[0] as usize;

            let b = &mut buf[..comm_key_len];
            self.read_exact(b).await?;
            self.comm_key = b.to_vec();
        }

        // reset cipher
        let (encoder, decoder) = Self::create_cipher(self.comm_key.as_slice());
        self.encoder = encoder;
        self.decoder = decoder;

        Ok(())
    }
}

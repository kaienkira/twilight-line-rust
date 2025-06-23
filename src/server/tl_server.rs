use bytes::BufMut;
use cfb_mode::cipher::KeyIvInit;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use crate::server_error::ServerError;
use tl_common::Result;

type Aes256CfbEncoder = cfb_mode::BufEncryptor<aes::Aes256>;
type Aes256CfbDecoder = cfb_mode::BufDecryptor<aes::Aes256>;

pub(crate) struct TlServer {
    conn: TcpStream,
    sec_key: &'static str,
    fake_request: &'static [u8],
    fake_response: &'static [u8],
    comm_key: Vec<u8>,
    encoder: Aes256CfbEncoder,
    decoder: Aes256CfbDecoder,
}

impl TlServer {
    pub fn new(
        conn: TcpStream,
        sec_key: &'static str,
        fake_request: &'static [u8],
        fake_response: &'static [u8],
    ) -> TlServer {
        let (encoder, decoder) = Self::create_cipher(sec_key.as_bytes());

        TlServer {
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

    pub async fn accept(&mut self) -> Result<TcpStream> {
        // read fake request
        {
            let mut buf: Vec<u8> = vec![0; self.fake_request.len()];
            let mut l: usize = 0;
            loop {
                let n = self.conn.read(&mut buf[l..]).await?;
                if n == 0 {
                    return Err(Box::new(ServerError::TlFakeRequestInvalid));
                }
                if buf[l..l + n] != self.fake_request[l..l + n] {
                    return Err(Box::new(ServerError::TlFakeRequestInvalid));
                }
                l += n;
                if l >= self.fake_request.len() {
                    break;
                }
            }
        }

        let mut buf: Vec<u8> = vec![0; 512];

        // read request addr
        let b = &mut buf[..2];
        self.read_exact(b).await?;
        let addr_len: usize = (((b[0] as u16) << 8) + b[1] as u16).into();
        if addr_len > 260 {
            return Err(Box::new(ServerError::TlRequestAddrInvalid));
        }

        let b = &mut buf[..addr_len];
        self.read_exact(b).await?;
        let addr = String::from_utf8(b.to_vec())?;
        let sign: Vec<u8> = tl_common::util::sha256_sum(
            format!("{}{}", addr, self.sec_key).as_bytes(),
        );

        // check addr sign
        let b = &mut buf[..32];
        self.read_exact(b).await?;
        if b != sign.as_slice() {
            return Err(Box::new(ServerError::TlRequestAddrInvalid));
        }

        // connect to request addr
        let conn: TcpStream = TcpStream::connect(addr).await?;

        // create communication key
        let comm_key_len = 32 + rand::random_range(0..128 - 32);
        self.comm_key = vec![0; comm_key_len];
        for i in 0..comm_key_len {
            self.comm_key[i] = rand::random_range(0..=255);
        }

        // write fake response
        self.conn.write_all(self.fake_response).await?;

        // write communication key
        {
            let mut buf: Vec<u8> = Vec::with_capacity(256);
            buf.put_u8(comm_key_len as u8);
            buf.put(self.comm_key.as_slice());
            self.write_all(buf.as_slice()).await?;
        }

        // reset cipher
        let (encoder, decoder) = Self::create_cipher(self.comm_key.as_slice());
        self.encoder = encoder;
        self.decoder = decoder;

        Ok(conn)
    }
}

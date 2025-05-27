use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

use crate::Config;
use crate::tl_server::TlServer;
use tl_common::Result;

pub(crate) async fn handle_proxy(config: &'static Config) -> Result<()> {
    let listener = TcpListener::bind(&config.local_addr).await?;

    loop {
        match listener.accept().await {
            Ok((conn, addr)) => {
                tokio::spawn(proxy(conn, config));
            }
            Err(e) => {
                eprintln!("TcpListener::accept() failed: {}", e);
            }
        }
    }
}

async fn proxy(client_conn: TcpStream, config: &'static Config) -> Result<()> {
    let mut s = TlServer::new(
        client_conn,
        &config.sec_key,
        config.fake_request.as_bytes(),
        config.fake_response.as_bytes(),
    );

    let mut c = s.accept().await?;

    let mut copy_buf: Vec<u8> = vec![0; 32 * 1024];
    loop {
        tokio::select! {
            _ = c.readable() => {
                let ret = copy_data_c2s(
                    &mut c, &mut s, copy_buf.as_mut_slice()).await?;
                if ret == false {
                    break;
                }
            }
            _ = s.wait_readable() => {
                let ret = copy_data_s2c(
                    &mut s, &mut c, copy_buf.as_mut_slice()).await?;
                if ret == false {
                    break;
                }
            }
        };
    }

    Ok(())
}

async fn copy_data_c2s(
    c: &mut TcpStream,
    s: &mut TlServer,
    buf: &mut [u8],
) -> Result<bool> {
    loop {
        match c.try_read(buf) {
            Ok(n) => {
                if n == 0 {
                    return Ok(false);
                }
                s.write_all(&buf[..n]).await?;
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::WouldBlock {
                    return Ok(true);
                } else {
                    return Err(e.into());
                }
            }
        }
    }
}

async fn copy_data_s2c(
    s: &mut TlServer,
    c: &mut TcpStream,
    buf: &mut [u8],
) -> Result<bool> {
    loop {
        match s.try_read(buf) {
            Ok(n) => {
                if n == 0 {
                    return Ok(false);
                }
                c.write_all(&buf[..n]).await?;
            }
            Err(e) => {
                if let Some(io_error) = e.downcast_ref::<std::io::Error>() {
                    if io_error.kind() == std::io::ErrorKind::WouldBlock {
                        return Ok(true);
                    }
                } else {
                    return Err(e.into());
                }
            }
        }
    }
}

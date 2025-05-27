use core::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

use crate::Config;
use crate::socks5_server::Socks5Server;
use crate::tl_client::TlClient;
use tl_common::Result;

pub(crate) async fn handle_proxy(config: &'static Config) -> Result<()> {
    let listener = TcpListener::bind(&config.local_addr).await?;

    loop {
        match listener.accept().await {
            Ok((conn, addr)) => {
                tokio::spawn(proxy(conn, addr, config));
            }
            Err(e) => {
                eprintln!("TcpListener::accept() failed: {}", e);
            }
        }
    }
}

async fn proxy(
    client_conn: TcpStream,
    client_addr: SocketAddr,
    config: &'static Config,
) -> Result<()> {
    let mut s = Socks5Server::new(client_conn);
    s.method_select().await?;
    let dst_addr = s.receive_dst_addr().await?;

    println!("proxy_request: [{}] => [{}]", client_addr, dst_addr);

    let server_conn: TcpStream;
    match TcpStream::connect(&config.server_addr).await {
        Ok(v) => server_conn = v,
        Err(e) => {
            eprintln!("connect tl-server failed: {}", e);
            return Err(e.into());
        }
    }

    let mut c = TlClient::new(
        server_conn,
        &config.sec_key,
        config.fake_request.as_bytes(),
        config.fake_response.as_bytes(),
    );
    c.connect(&dst_addr).await?;

    s.notify_connect_success().await?;

    let mut copy_buf: Vec<u8> = vec![0; 32 * 1024];
    loop {
        tokio::select! {
            _ = c.wait_readable() => {
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
    c: &mut TlClient,
    s: &mut Socks5Server,
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

async fn copy_data_s2c(
    s: &mut Socks5Server,
    c: &mut TlClient,
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

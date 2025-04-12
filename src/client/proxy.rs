use core::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

use crate::Config;
use crate::common::Result;
use crate::socks5_server::Socks5Server;

pub(crate) async fn handle_proxy(config: &'static Config)
    -> Result<()> {
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
    config: &Config)
    -> Result<()> {
    let mut s = Socks5Server::new(client_conn);
    s.method_select().await?;

    Ok(())
}

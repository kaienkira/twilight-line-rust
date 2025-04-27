use core::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

use crate::Config;
use crate::common::Result;
use crate::socks5_server::Socks5Server;
use crate::tl_client::TlClient;

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
    config: &'static Config)
    -> Result<()> {
    let mut s = Socks5Server::new(client_conn);
    s.method_select().await?;
    let dst_addr = s.receive_dst_addr().await?;

    println!("proxy_request: [{}] => [{}]", client_addr, dst_addr);

    let server_conn: TcpStream;
    match TcpStream::connect(&config.server_addr).await {
        Ok(v) => server_conn = v,
        Err(e) => {
            eprintln!("connect tl-server failed: {}", e);
            return Err(Box::new(e));
        }
    }

    let c = TlClient::new(server_conn,
        &config.sec_key,
        config.fake_request.as_bytes(),
        config.fake_response.as_bytes());

    s.notify_connect_success().await?;

    Ok(())
}

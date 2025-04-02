use crate::Config;
use core::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

pub(crate) async fn handle_proxy(config: &'static Config)
    -> std::io::Result<()>
{
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
    -> std::io::Result<()>
{
    Ok(())
}

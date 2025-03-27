use crate::Config;
use core::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

pub(crate) async fn handle_proxy(config: Config) -> std::io::Result<()> {
    let listener = TcpListener::bind(config.local_addr).await?;

    loop {
        match listener.accept().await {
            Ok((conn, addr)) => proxy(conn, addr),
            Err(e) => {
                eprintln!("TcpListener::accept() failed: {}", e);
            }
        }
    }
}

fn proxy(client_conn: TcpStream, client_addr: SocketAddr)
{
}

use std::net::TcpStream;

pub(crate) struct TlClient {
    conn: TcpStream,
    sec_key: String,
}

impl TlClient {
}

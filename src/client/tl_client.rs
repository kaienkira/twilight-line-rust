use tokio::net::TcpStream;

pub(crate) struct TlClient {
    conn: TcpStream,
    sec_key: String,
    fake_request_bytes: Vec<u8>,
    fake_response_bytes: Vec<u8>,
    comm_key: Vec<u8>,
}

impl TlClient {
}

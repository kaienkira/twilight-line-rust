use tokio::net::TcpStream;

pub(crate) struct TlClient<'a> {
    conn: TcpStream,
    sec_key: &'a str,
    fake_request_bytes: &'a [u8],
    fake_response_bytes: &'a [u8],
    comm_key: Vec<u8>,
}

impl<'a> TlClient<'a> {
}

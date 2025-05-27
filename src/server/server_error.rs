use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Copy, Debug)]
pub(crate) enum ServerError {
    TlFakeRequestInvalid,
    TlRequestAddrInvalid,
}

impl ServerError {
    pub fn as_str(&self) -> &'static str {
        use ServerError::*;
        match *self {
            TlFakeRequestInvalid => "tlproxy fake request invalid",
            TlRequestAddrInvalid => "tlproxy request addr invalid",
        }
    }
}

impl Error for ServerError {}

impl Display for ServerError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(self.as_str())
    }
}

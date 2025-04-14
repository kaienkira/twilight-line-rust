use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Copy, Debug)]
pub(crate) enum ClientError {
    Socks5VersionInvalid,
    Socks5CmdNotSupported,
}

impl ClientError {
    pub fn as_str(&self) -> &'static str
    {
        use ClientError::*;
        match *self {
            Socks5VersionInvalid => "socks5 version invalid",
            Socks5CmdNotSupported => "socks5 cmd not supported",
        }
    }
}

impl Error for ClientError {
}

impl Display for ClientError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(self.as_str())
    }
}

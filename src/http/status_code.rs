use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Copy, Clone, Debug)]

pub enum StatusCode {
    OK = 200,
    BADREQUEST = 400,
    NOTFOUND = 404
}

// #[derive(Copy, Clone, Debug)]

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::OK => "OK",
            Self::BADREQUEST => "BADREQUEST",
            Self::NOTFOUND => "NOTFOUND"
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}

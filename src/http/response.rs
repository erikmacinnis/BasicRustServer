use tokio::io::{AsyncWriteExt, Result as AsyncResult}; 
use std::pin::Pin;
use super::StatusCode;

pub struct Response {
    pub  status_code: StatusCode,
    body: Option<String>
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self {
            status_code,
            body
        }
    }
    #[warn(unused_must_use)]
    pub async fn send(&self, mut stream: Pin<&mut impl AsyncWriteExt>) -> Result<(), std::io::Error> {
        let body = match &self.body {
            Some(b) => b,
            None => ""
        };

        let message = format!("HTTP/1.1 {} {}\r\n\r\n{}", 
        self.status_code,
        self.status_code.reason_phrase(), 
        body);

        stream.write_all(
            message.as_bytes()
        ).await

    }
}
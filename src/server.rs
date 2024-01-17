// use std::net::TcpListener;

use tokio::io::{AsyncReadExt};
use tokio::net::{TcpListener, TcpStream};
use std::pin::Pin;
use std::sync::Arc;
use std::thread::JoinHandle;
use crate::website_handler::WebsiteHandler;
use std::boxed::Box;

use super::http::{Response, Request, ParseError, StatusCode};

pub struct Server { 
    addr: String,
}

pub trait Handler {
    fn good_request_handler(&mut self, request: &Request) -> Response;

    fn bad_request_handler(&mut self, error: &ParseError) -> Response;
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    #[tokio::main]

    pub async fn run(self, handler: WebsiteHandler) {

        println!("Listening on {}", &self.addr);
        let listener = TcpListener::bind(self.addr).await.unwrap();
        
        // let handler = Arc::new(handler);

        loop {
            // accepted request
            match listener.accept().await {
                Ok((mut stream, _)) => {
                    let mut handler = handler.clone();
                    let _future = tokio::spawn( async move {
                        handle_client(stream, handler).await
                    });
                }

                Err(e) => println!("Request was not accepted {}", e)
            }
        }
    }
}

// this iwll return impl std::future::Future<Output = ()>
async fn handle_client(mut stream: TcpStream, handler: WebsiteHandler) 
    {
        let mut buffer = [0;1024];

        let mut handler_mut = Box::pin(handler);

        // read stream into buffer
        match stream.read(&mut buffer[..]).await {
            // check if we can read the buffer
            Ok(_) => {
                // if yes we print the stream and create a response object
                println!("Recieved request from {}.", String::from_utf8_lossy(&buffer));
                let response = match Request::try_from(&buffer as &[u8]) {
                    Ok(request) => handler_mut.good_request_handler(&request),
                    Err(e) => handler_mut.bad_request_handler(&e)
                };

                if response.status_code as u8 == StatusCode::BADREQUEST as u8 {
                    println!("This was a bad request");
                }
                if let Err(e) = response.send(Pin::new(&mut stream)).await {
                    println!("Failed to send response {}", e);
                };
            }
            // if not then we print this issue
            Err(e) => {
                println!("Issues reading from the buffer {}", e)
        }
    }
}

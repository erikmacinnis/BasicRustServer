use crate::server::Handler;
use std::fs;
use super::http::{Response, Request, StatusCode, Method};

#[derive( Clone)]
pub struct WebsiteHandler{
    public_path: String
}


impl WebsiteHandler {
    pub fn new<'a>(public_path: String) -> Self {
        Self {
            public_path
        }
    }

    pub fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!{"{}/{}", self.public_path, file_path};

        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                }else {
                    println!("Dir Travers Attack: {}", file_path);
                    None
                }
            }
            Err(_) => None
        }
    }
}

impl Handler for WebsiteHandler {

    fn good_request_handler(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {

                "/" => Response::new(StatusCode::OK, Some("hello".to_string())),
                "/home" => Response::new(StatusCode::OK, Some("You are home".to_string())),

                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::OK, Some(contents)),
                    None => Response::new(StatusCode::NOTFOUND, None),
                }
            }
            _ => Response::new(StatusCode::NOTFOUND, None)
        }
    }

    fn bad_request_handler(&mut self, error: &crate::http::ParseError) -> Response {
        Response::new(StatusCode::BADREQUEST, Some("<h1>Bad Request Test</h1>".to_string()))
    }
}
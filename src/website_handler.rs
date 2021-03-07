use super::server::Handler;
use crate::http::{Request, Response, StatusCode};

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, _request: &Request) -> Response {
        Response::new(StatusCode::Ok, Some("<h1>TEST</h1>".to_string()))
    }
}

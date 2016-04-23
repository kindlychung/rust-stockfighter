extern crate hyper;

use std::io::Read;
use hyper::Client;
use hyper::header::Connection;

pub struct Request {
    pub requires_auth: bool,
    pub request_url: String,
}

impl Request {
    pub fn send_request(&self) -> String {
        let client = Client::new();

        let mut res = client.get(&self.request_url)
                            .header(Connection::close())
                            .send()
                            .unwrap();

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        body
    }
}

use crate::request::Request;

use std::{collections::HashMap, net::{TcpListener, TcpStream}, io::{Write}};
use log;

type Handler = fn (&Request) -> String;

pub struct Server {
  routes: HashMap<String, Handler>,
  not_found_handler: Handler
}

impl Server {
  pub fn new(not_found_handler: Handler) -> Server {
    Server {
      routes: HashMap::new(),
      not_found_handler
    }
  }

  pub fn route(&mut self, path: String, handler: Handler) {
    self.routes.insert(path, handler);
  }

  fn handle_stream(&self, mut stream: &TcpStream) {
    log::debug!("Accepted Connection");
    let request = Request::new(&stream);
    if request.aborted {
      log::debug!("Aborted Request");
      stream.flush().unwrap();
      return
    }
    log::info!("Request {} {}", request.method, request.path);
    let handler = self.routes.get(&request.path).map(|f| *f).unwrap_or(self.not_found_handler);
    let response = handler(&request);
    stream.write_all(&response.as_bytes()).expect("write to be a ok");
    stream.flush().expect("expect to be able to flush stream after write");
  }

  pub fn listen(&mut self, address: &str) {
    let listener = TcpListener::bind(address).unwrap();
    log::debug!("Listening at: {}", address);
    for stream in listener.incoming() {
      match stream {
        Ok(stream) => {
          self.handle_stream(&stream)
        }
        Err(_e) => {
          log::debug!("Connection Failed")
        }
      }
    }
    log::debug!("Closed");
  }
}
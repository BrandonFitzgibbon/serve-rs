use std::{net::TcpStream, io::Read, io::Error, collections::HashMap, time::Duration};

fn read_to_char(mut stream: &TcpStream, check_char: char) -> Result<String, Error> {
  let mut buf = [0; 1];
  let mut val = String::new();
  loop {
    stream.read(&mut buf)?;
    match buf {
      [0] => break,
      [c] if c as char == check_char => break,
      [c] => {
        val.push(c as char);   
      }
    }
  }
  Ok(val)
}

type ParseResult = (String, String, HashMap<String, String>);

fn parse_method_and_path(status_string: &String) -> Option<(String, String)> {
  let mut iter = status_string.split_ascii_whitespace();
  let method = iter.next()?;
  let path = iter.next()?;
  Some((String::from(method), String::from(path)))
}

fn parse_http_stream(stream: &TcpStream) -> Result<ParseResult, Error> {
  stream.set_read_timeout(Some(Duration::new(3, 0)))?;
  let status_string = read_to_char(&stream, '\n')?;
  let mut headers: HashMap<String, String> = HashMap::new();
  let mut method = String::from("");
  let mut path = String::from("");
  if let Some((parsed_method, parsed_path)) = parse_method_and_path(&status_string) {
    method = parsed_method;
    path = parsed_path;
  };
  loop {
    let header = read_to_char(&stream, '\n')?;
    if header.eq_ignore_ascii_case("\r") {
      break
    }
    let mut siter = header.split(": ");
    let key = siter.next().unwrap();
    let value = siter.next().unwrap();
    headers.insert(key.to_string(), value.to_string());
  }
  Ok((method, path, headers))
}

pub struct Request {
  pub method: String,
  pub path: String,
  pub headers: HashMap<String, String>,
  pub aborted: bool
}

impl Request {
  pub fn new(stream: &TcpStream) -> Request {
    let result = parse_http_stream(stream);
    match result {
      Ok((method, path, headers)) => Request { 
        method, 
        path,
        headers,
        aborted: false
      },
      Err(_e) => Request {
        method: "".to_string(),
        path: "".to_string(),
        headers: HashMap::new(),
        aborted: true
      }
    }
  }
}
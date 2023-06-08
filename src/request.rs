use std::{net::TcpStream, io::{Read, ErrorKind}, io::Error, collections::HashMap, time::Duration};

#[cfg(test)]
mod test {
    use super::read_to_char;

  #[test]
  fn it_reads_to_char() {
    let status = String::from("GET / HTTP/1.1\r\n");
    let mut stream = status.as_bytes();
    let method = read_to_char(&mut stream, ' ', None).unwrap();
    let uri = read_to_char(&mut stream, ' ', Some(8000)).unwrap();
    assert_eq!(method, String::from("GET"));
    assert_eq!(uri, String::from("/"));
  }

  #[test]
  #[should_panic]
  fn it_panics_when_exceeding_max_octets() {
    let s = String::from("Hello, world");
    let mut stream = s.as_bytes();
    read_to_char(&mut stream, ' ', Some(1)).unwrap();
  }
}

fn read_to_char(stream: &mut impl Read, check_char: char, max_octets: Option<usize>) -> Result<String, Error> {
  let mut buf = [0; 1];
  let mut val = String::new();
  let mut bytes_read: usize = 0;
  loop {
    bytes_read = bytes_read + stream.read(&mut buf)?;
    println!("{bytes_read}");
    match buf {
      [0] => break,
      [c] if c as char == check_char => break,
      [c] => {
        val.push(c as char);   
      }
    }
    if max_octets.is_some() {
      let max = max_octets.unwrap();
      if max < bytes_read {
        return Err(Error::from(ErrorKind::InvalidData));
      }
    }
  }
  Ok(val)
}

type ParseResult = (String, String, HashMap<String, String>);

fn parse_method_and_path(status_string: &String) -> Option<(&str, &str)> {
  let mut iter = status_string.split_ascii_whitespace();
  let method = iter.next()?;
  let path = iter.next()?;
  Some((method, path))
}

fn parse_http_stream(stream: &mut TcpStream) -> Result<ParseResult, Error> {
  stream.set_read_timeout(Some(Duration::new(3, 0)))?;
  let status_string = read_to_char(stream, '\n', Some(8000))?;
  let method;
  let path;
  if let Some((parsed_method, parsed_path)) = parse_method_and_path(&status_string) {
    method = parsed_method;
    path = parsed_path;
  } else {
    return Err(Error::from(ErrorKind::InvalidData))
  }
  let mut headers: HashMap<String, String> = HashMap::new();
  loop {
    let header = read_to_char(stream, '\n', None)?;
    if header.eq_ignore_ascii_case("\r") {
      break
    }
    let mut split_header = header.split(": ");
    let key;
    let value;
    if let Some(split_key) = split_header.next() {
      key = split_key;
    } else {
      return Err(Error::from(ErrorKind::InvalidData))
    }
    if let Some(split_value) = split_header.next() {
      value = split_value;
    } else {
      return Err(Error::from(ErrorKind::InvalidData))
    }
    headers.insert(key.to_string(), value.to_string());
  }
  Ok((method.to_string(), path.to_string(), headers))
}

pub struct Request {
  pub method: String,
  pub path: String,
  pub headers: HashMap<String, String>,
  pub aborted: bool
}

impl Request {
  pub fn new(stream: &mut TcpStream) -> Request {
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
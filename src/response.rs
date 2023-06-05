use serde::{Serialize, Deserialize};
use serde_json;
use std::collections::HashMap;

fn get_status_string(status_code: u16) -> &'static str {
  return match status_code {
    200 => "HTTP/1.1 200 OK",
    404 => "HTTP/1.1 404 Not Found",
    _ => "HTTP/1.1 404 Not Found"
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T: Serialize> {
    headers: HashMap<String, String>,
    status: u16,
    data: T,
}

impl<T: Serialize> Response<T> {
  pub fn new(data: T, status: u16) -> Response<T> {
    return Response {
      data,
      headers: HashMap::new(),
      status
    }
  }

  pub fn stringify(&mut self) -> String {
    let contents = serde_json::to_string(&self.data).unwrap();
    let length = contents.len();
    self.headers.insert(String::from("Content-Length"), length.to_string());
    self.headers.insert(String::from("Content-Type"), String::from("application/json"));
    let headers = self.headers.iter().map(|(key, val)| format!("{}: {}", key, val)).collect::<Vec<String>>().join("\r\n");
    let status = get_status_string(self.status);
    return format!("{status}\r\n{headers}\r\n\r\n{contents}\r\n")
  }
}
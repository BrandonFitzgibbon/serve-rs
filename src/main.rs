mod response;
mod server;
mod request;
use crate::response::Response;
use crate::server::Server;
use request::Request;
use serde::{Serialize, Deserialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
  x: u16,
  y: u16
}

#[derive(Serialize, Deserialize, Debug)]
struct DataResponse<T> {
  message: String,
  data: T
}

fn root_get() -> String {
  return response::Response::new(Point { x: 10, y: 20 }, 200).stringify();
}

fn handle_root(request: &Request) -> String {
  return match request.method.as_str() {
    "GET" => root_get(),
    _ => handle_404(request)
  }
}

fn handle_404(_request: &Request) -> String {
  return Response::new(DataResponse { message: String::from("Not Found"), data: ()}, 404).stringify();
}

fn init_logger() -> Result<(), fern::InitError> {
  fern::Dispatch::new()
    .format(|out, message, record| {
      out.finish(format_args!(
        "[{} {} {}] {}",
        humantime::format_rfc3339_seconds(SystemTime::now()),
        record.level(),
        record.target(),
        message
      ))
    })
    .chain(std::io::stdout())
    .apply()?;
  Ok(())
}

fn main() -> std::io::Result<()> {
  init_logger().unwrap();
  let mut server = Server::new(handle_404);
  server.route("/".to_string(), handle_root);
  server.listen("127.0.0.1:4001");
  Ok(())
}

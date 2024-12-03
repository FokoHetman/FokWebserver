use std::{
  fs, net::{TcpListener, TcpStream},
  io::{Read,Write,self},str
};
mod utils;
use utils::threading::ThreadPool;
use utils::web;

struct Response {
  body: String,
  headers: Vec<(String, String)>,
  code: i32,
}
impl Response {
  fn build(&self) -> String {
    let mut result = format!("HTTP/1.1 {} {}\r\n", self.code, self.get_status());
    for i in self.headers.clone() {
      result += &(i.0 + ": " + &i.1 + "\r\n");
    }
    result += "\r\n\r\n";
    result += &self.body;
    result
  }
  fn get_status(&self) -> &str {
    match self.code {
      100 => "CONTINUE",
      200 => "OK",
      404 => "NOT FOUND",
      _ => "UNKNOWN",
    }
  }
  fn new() -> Self {
    Self {body: String::new(), headers: vec![], code: 200}
  }
}



fn estabilish_listener(ip: &str) {
  let listener = TcpListener::bind(ip).unwrap();
  println!("Listening on http://{ip}");
  let pool = ThreadPool::new(8);

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    pool.execute(|| {let _ = handle_connection(stream);});
  }
}

fn handle_connection(mut stream: TcpStream) {
  let mut buffer = [0; 1024];
  stream.read(&mut buffer).unwrap();
  let req = str::from_utf8(&buffer).unwrap();
  let contents = req.split("\r\n\r\n").collect::<Vec<&str>>()[1].replace("\0","");
  println!("request: {}", req);
  let dir = req.split(" ").collect::<Vec<&str>>()[1];


  let mut response = Response::new();

  match dir {
    "/" | "/home" | "/homepage" | "/index" | "/index.html" => {
      response.body = web::render_template("templates/index.html", vec![]);
      response.code = 200;
    },
    _ => {
      println!("{}", dir);
      response.body = web::render_template("templates/404.html", vec![]);
      response.code = 404;
    },
  }
  let _ = stream.write(response.build().as_bytes());
}


fn main() {
  estabilish_listener("0.0.0.0:2137");
}

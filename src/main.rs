use std::{
  fs, net::{TcpListener, TcpStream},
  io::{Read,Write,self},str
};
mod utils;
use utils::threading::ThreadPool;
use utils::web;
pub mod lib;
use lib::json;

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
      100 => "CONTINUE",  // idk what that is lol
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



fn load_lang(lang: &str) -> Vec<(Box<web::Fructa>, Box<web::Fructa>)> {
  let mut result = vec![];

  let lang_json = json::parse_json(fs::read_to_string("translations/".to_string()+&lang+".json").unwrap());
  match lang_json.body {
    json::JsonElem::Dict(d) => {
      for i in d {
        result.push(
          (match i.0 {
            json::JsonElem::String(s) => {
              Box::new(web::Fructa::Str(s))
            },
            _ => panic!("?")
          },
          match i.1 {
            json::JsonElem::String(s) => {
              Box::new(web::Fructa::Str(s))
            },
            _ => panic!("?")
          }
          ));
      }
    }
    _ => panic!("No lang file found: {}", lang)
  }

  result
}


fn handle_connection(mut stream: TcpStream) {
  let mut buffer = [0; 1024];
  stream.read(&mut buffer).unwrap();
  let req = str::from_utf8(&buffer).unwrap();
  let contents = req.split("\r\n\r\n").collect::<Vec<&str>>()[1].replace("\0","");
  println!("request: {}", req);
  let dir = req.split(" ").collect::<Vec<&str>>()[1];



  let languages = 
      ("translations".to_string(), web::Fructa::Dictario(vec![
        (Box::new(web::Fructa::Str("en_us".to_string())), Box::new(web::Fructa::Dictario(load_lang("en_us")))),
      ]));


  let mut response = Response::new();

  if dir.starts_with("/static/") && !dir.contains("..") {
    let _ = stream.write(&fs::read(dir[1..].to_string()).unwrap());
  } else {

    match dir {
      "/" | "/home" | "/homepage" | "/index" | "/index.html" => {
        response.body = web::render_template("templates/index.html", vec![
            ("username".to_string(), web::Fructa::Str("Foko".to_string())),
            ("lang".to_string(), web::Fructa::Str("en_us".to_string())),
            languages
        ]);
        response.code = 200;
      },
      "/test" => {
        response.body = web::render_template("templates/test.html", vec![
            ("lang".to_string(), web::Fructa::Str("en_us".to_string())),
            languages
        ]);
        response.code = 200;
      },
      "/favicon.ico" => {
        let _ = stream.write(&fs::read(dir[1..].to_string()).unwrap());
        return;
      },
      _ => {
        println!("{}", dir);
        response.body = web::render_template("templates/404.html", vec![]);
        response.code = 404;
      },
    }
    let _ = stream.write(response.build().as_bytes());
  }
}


fn main() {
  estabilish_listener("0.0.0.0:2137");
}

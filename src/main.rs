use std::{
  fs, net::{TcpListener, TcpStream},
  io::{Read,Write,self},str,
  sync::{Mutex,Arc},
  process::{Command, Stdio},
};
mod utils;
use utils::threading::ThreadPool;
use utils::{web,language::{self,Fructa}};
pub mod lib;
use lib::json;

//use sqlite;


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



fn estabilish_listener(ip: &str, controller: Arc<Mutex<Controller>>) {
  let listener = TcpListener::bind(ip).unwrap();
  println!("Listening on http://{ip}");
  let pool = ThreadPool::new(8);

  for stream in listener.incoming() {
    let stream = stream.unwrap();
    let clone = Arc::clone(&controller);
    pool.execute(|| {let _ = handle_connection(stream, clone);});
  }
}



fn load_lang(lang: &str) -> Vec<(Box<Fructa>, Box<Fructa>)> {
  let mut result = vec![];

  let lang_json = json::parse_json(fs::read_to_string("translations/".to_string()+&lang+".json").unwrap());
  match lang_json.body {
    json::JsonElem::Dict(d) => {
      for i in d {
        result.push(
          (match i.0 {
            json::JsonElem::String(s) => {
              Box::new(Fructa::Str(s))
            },
            _ => panic!("?")
          },
          match i.1 {
            json::JsonElem::String(s) => {
              Box::new(Fructa::Str(s))
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


fn handle_connection(mut stream: TcpStream, controller: Arc<Mutex<Controller>>) {
  let mut buffer = [0; 1024];
  stream.read(&mut buffer).unwrap();
  let req = str::from_utf8(&buffer).unwrap();
  let contents = req.split("\r\n\r\n").collect::<Vec<&str>>()[1].replace("\0","");
  let o_headers = req.split("\r\n\r\n").collect::<Vec<&str>>()[0].split("\r\n").collect::<Vec<&str>>()[1..].to_vec();
  let mut headers: Vec<(&str, &str)> = vec![];
  for i in o_headers {
    let s = i.split(": ").collect::<Vec<&str>>();
    headers.push((s[0], s[1]));
  }
  println!("headers: {:#?}", headers);
  println!("request: {:#?}", req);
  let mut dir = req.split(" ").collect::<Vec<&str>>()[1];
  
  let mut params: Vec<(&str, &str)> = vec![];
  if dir.contains("?") {
    let split = &dir.split("?").collect::<Vec<&str>>();
    dir = split[0];
    let split = &split[1..];
    for i in split {
      let split = i.split("=").collect::<Vec<&str>>();
      params.push((split[0], split[1]));
    }
  }
  println!("{}", dir);

  let mut cookies: Vec<(&str, &str)> = vec![];
  for i in headers {
    if i.0 == "Cookie" {
      let s = i.1.split("=").collect::<Vec<&str>>();
      cookies.push((s[0].trim(), s[1].trim()));
    }
  }
  println!("cookies: {:#?}", cookies);


  let langs = vec!["en_uk", "pl_pl"];

  let languages = 
      ("translations".to_string(), 
        Fructa::Dictario(
           langs.clone().into_iter().map(|x| (Box::new(Fructa::Str(x.to_string())), Box::new(Fructa::Dictario(load_lang(x))))).into_iter().collect::<Vec<(Box<Fructa>, Box<Fructa>)>>()
        ));

      /*vec![
        (Box::new(web::Fructa::Str("en_uk".to_string())), Box::new(web::Fructa::Dictario(load_lang("en_uk")))),
        (Box::new(web::Fructa::Str("pl_pl".to_string())), Box::new(web::Fructa::Dictario(load_lang("pl_pl")))),
      ]));*/

  /*help
  let lock = controller.lock().unwrap();
  *lock.db_conn.prepare(statement)
   */


  let mut response = Response::new();

  let mut lang = "en_uk".to_string();
  for i in cookies {
    if i.0=="lang" {
      lang = i.1.to_string();
    }
  }

  if (dir.starts_with("/static/") || dir.starts_with("/dynamic/")) && !dir.contains("..") {
    let _ = stream.write(&fs::read(dir[1..].to_string()).unwrap());
  } else {

    match dir {
      "/" | "/home" | "/homepage" | "/index" | "/index.html" => {
        response.body = web::render_template("templates/index.html", vec![
            ("username".to_string(), Fructa::Str("Foko".to_string())),
            ("lang".to_string(), Fructa::Str(lang)),
            languages
        ]);
        response.code = 200;
      },
      "/projects" => {
        let mut render_project = false;
        for i in params {
          match i.0 {
            "project" => {
              render_project = true;
              response.body = web::render_gitrepo(&("/home/git/FokoHetman/".to_owned() + i.1), vec![
                ("lang".to_string(), Fructa::Str(lang.clone())),
                languages.clone()
              ]);
            },
            _ => {}
          }
        }
        if !render_project {
          response.body = web::render_gituser("/home/git/FokoHetman/", vec![
            ("username".to_string(), Fructa::Str("Foko".to_string())),
            ("lang".to_string(), Fructa::Str(lang)),
            languages
          ]);
        }
      },
      "/change_lang"  => {
        let mut lang = String::from("en_uk");
        let mut redirect = String::from("/");
        for i in params {
          match i.0 {
            "lang" => {
              lang = langs[i.1.to_string().parse::<usize>().unwrap()].to_string();
            },
            "redirect" => {
              redirect = i.1.to_string();
            },
            _ => {}
          }
        }
        response.body = format!("<script> window.location.replace(\"{}\"); </script>", redirect);
        response.headers.push(("Set-Cookie".to_string(), format!("lang={}", lang)));
      },
      "/test" => {
        response.body = web::render_template("templates/test.html", vec![
            ("lang".to_string(), Fructa::Str(lang)),
            languages
        ]);
        response.code = 200;
      },
      "/favicon.ico" => {
        let _ = stream.write(&fs::read(dir[1..].to_string()).unwrap());
        return;
      },
      "/terminal" => {
        let mut run = None;
        for i in params {
          match i.0 {
            "command" => {
              run = Some(i.1);
            },
            _ => {}
          }
        }
        match run {
          Some(i) => {
            response.body = "{\"help\": \"me\"}".to_string();
            response.code = 200;
          },
          None => {
            response.body = web::render_template("templates/projects/emulators/terminal.html", vec![
              ("username".to_string(), Fructa::Str("Foko".to_string())),
              ("lang".to_string(), Fructa::Str(lang)),
              languages
            ]);
            response.code = 200;
          }
        }
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


struct  Controller {
//  db_conn: sqlite::Connection,
}


fn start_terminal_session() {
  let mut child_shell = Command::new("nix-shell")
      .stdin(Stdio::piped())
      .stdout(Stdio::piped())
      .spawn().unwrap();
  
}



fn main() {
  let mut controller = Controller {
    //db_conn: sqlite::open("databases/main.db").unwrap(),
  };
  let mut controller = Arc::new(Mutex::new(controller));
  estabilish_listener("0.0.0.0:2137", controller);
}

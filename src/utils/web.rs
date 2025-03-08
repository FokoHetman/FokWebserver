use std::{fs, path::Path, process::Command};
use crate::utils::language::{self,Fructa};


static UNKNOWN_IMAGE_PATH: &str = "assets/whar.png";

pub fn render_gituser(path: &str, args: Vec<(String, Fructa)>) -> String {
  let mut repositories: Vec<Box<Fructa>> = vec![];
  for i in fs::read_dir(path).unwrap() {
    let root = i.unwrap().path().to_str().unwrap().to_owned();
    let desc_file = fs::read_to_string(root.clone() + "/description").unwrap().to_owned();
    let splt = desc_file.split(";").collect::<Vec<&str>>();
    let img_path;
    let pathe = root.clone() + "/banner.png";
    if Path::new(&pathe).exists() {
      img_path = "dynamic/".to_owned() + root.split("/").collect::<Vec<&str>>().last().unwrap() + "_banner.png";
      Command::new("cp").arg(pathe).arg(img_path.clone()).output().unwrap().stdout;
    } else {
      img_path = UNKNOWN_IMAGE_PATH.to_string();
    }
    

    repositories.push(Box::new(
      Fructa::Dictario(vec![
        (Box::new(Fructa::Str(String::from("name"))), Box::new(Fructa::Str(splt[0].to_string()))),
        (Box::new(Fructa::Str(String::from("desc"))), Box::new(Fructa::Str(splt[1].to_string()))),
        (Box::new(Fructa::Str(String::from("img_path"))), Box::new(Fructa::Str(  img_path  ))),
      ]
      )
    ));
  };
  let mut args = args;
  args.push(("repos".to_string(), Fructa::Inventarii(repositories)));
  render_template("templates/git.html", args)
}











pub fn render_template(path: &str, args: Vec<(String, Fructa)>) -> String {
  let template = fs::read_to_string(path).unwrap();
  let tokens = tokenize_template(template.clone());
  return parse(tokens, args);
}

#[derive(Debug,PartialEq,Clone)]
pub enum TemplateToken {
  CodeBlock(String),
  Irrelevant(String),
}

pub fn tokenize_template(template: String) -> Vec<TemplateToken>{
  let mut result: Vec<TemplateToken> = vec![];

  let mut chars = template.chars().collect::<Vec<char>>();

  while chars.len() > 1 {
    let mut buffer = String::new();
    while chars.len()>1 && (chars[0] != '{' || chars[1] != '{') {
      buffer += &chars[0].to_string();
      chars.remove(0);
    }
    result.push(TemplateToken::Irrelevant(buffer));
    for i in 0..2 {
      if chars.len()>0 {
        chars.remove(0);
      }
    }
    let mut buffer = String::new();
    while chars.len()>1 && (chars[0] != '}' || chars[1] != '}') {
      buffer += &chars[0].to_string();
      chars.remove(0);
    }
    for i in 0..2 {
      if chars.len()>0 {
        chars.remove(0);
      }
    }

    result.push(TemplateToken::CodeBlock(buffer));
  }
  result.push(TemplateToken::Irrelevant(chars.into_iter().collect::<String>()));


  result
}

pub fn parse(tokens: Vec<TemplateToken>, args: Vec<(String, Fructa)>) -> String {
  let mut result = String::new();
  println!("{:#?}", tokens);
  let mut parser = language::Parser {tokens: vec![]};
  let mut env =  language::Env {data: args};
  for i in tokens {
    result += &match i {
      TemplateToken::Irrelevant(s) => s,
      TemplateToken::CodeBlock(s) => template_lang(s, &mut parser.clone(), &mut env),
    }
  }
  result
}

pub fn template_lang(string: String, parser: &mut language::Parser, env: &mut language::Env) -> String {
  let tokens = language::tokenize_lang(string);
  println!("{:#?}", tokens);
  
  language::parse_lang(tokens, parser, env)
}



use std::fs;
use crate::utils::language::{self,Fructa};



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



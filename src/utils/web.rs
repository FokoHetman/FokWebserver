use std::fs;



pub fn render_template(path: &str, args: Vec<(String, String)>) -> String {
  let template = fs::read_to_string(path).unwrap();
  let tokens = tokenize_template(template.clone());
  
  return parse(tokens, args);
}

#[derive(Debug,PartialEq,Clone)]
enum TemplateToken {
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

pub fn parse(tokens: Vec<TemplateToken>, args: Vec<(String, String)>) -> String {
  let mut result = String::new();
  println!("{:#?}", tokens);
  for i in tokens {
    result += &match i {
      TemplateToken::Irrelevant(s) => s,
      TemplateToken::CodeBlock(s) => template_lang(s, args.clone()),
    }
  }
  result
}

pub fn template_lang(string: String, args: Vec<(String, String)>) -> String {
  let tokens = tokenize_lang(string);
  println!("{:#?}", tokens);
  
  parse_lang(tokens)
}

#[derive(Debug,PartialEq,Clone)]
pub enum Token {
  Int(i32),
  Float(f64),
  Str(String),
  Identifier(String),
  Oparen,
  Cparen,
  OCparen,
  CCparen,
  Operator(Operator),
}


#[derive(Debug,PartialEq,Clone)]
pub enum Operator {
  Addition,
  Substraction,
  Multiplication,
  Division,
  Exponentiation,
  Range,//..
}

#[derive(Debug,PartialEq,Clone)]
pub enum Node {
  BinaryOperation(Box<Node>, Box<Node>, Operator),
  Integer(i32),
  Str(String),
  List(Vec<Box<Node>>),
}


pub fn parse_lang(tokens: Vec<Token>) -> String {
  let mut tokens = tokens.clone();
  let mut result = String::new();

  let mut nodes: Vec<Node> = vec![];

  while tokens.len()>0 {
    nodes.push(parse_primary(tokens[0].clone()));
  }
  result
}

pub fn parse_primary(token: Token) -> Node {
  match token {
    _ => panic!("hi")
  }
}





pub fn tokenize_lang(string: String) -> Vec<Token> {
  let mut chars = string.chars().collect::<Vec<char>>();

  let mut result: Vec<Token> = vec![];
  while chars.len()>0 {
    match chars[0] {
      '(' => {result.push(Token::Oparen)},
      ')' => {result.push(Token::Cparen)},
      '{' => {result.push(Token::OCparen)},
      '}' => {result.push(Token::CCparen)},
      '+' => {result.push(Token::Operator(Operator::Addition))},
      '-' => {result.push(Token::Operator(Operator::Substraction))},
      '*' => {result.push(Token::Operator(Operator::Multiplication))},
      '/' => {result.push(Token::Operator(Operator::Division))},
      '\"' => {
        let mut buffer = String::new();
        while chars[0]!='"' {
          buffer+=&chars[0].to_string();
        }
        result.push(Token::Str(buffer));
      },
      _ => {
        if chars[0].is_numeric() {
          let mut buffer = String::new();
          while chars.len()>0 && chars[0].is_numeric() {
            buffer += &chars[0].to_string();
            chars.remove(0);
          }
          if chars[0]=='.' {
            buffer += ".";
            chars.remove(0);
            while chars.len()>0 && chars[0].is_numeric() {
              buffer += &chars[0].to_string();
              chars.remove(0);
            }
            result.push(Token::Float(buffer.parse::<f64>().unwrap()));
          } else {
            result.push(Token::Int(buffer.parse::<i32>().unwrap()));
          }
          continue
        } else {
          if chars[0].is_alphabetic() {
            let mut buffer = String::new();
            while chars.len()>0 && chars[0].is_alphabetic() {
              buffer += &chars[0].to_string();
              chars.remove(0);
            }
            result.push(Token::Identifier(buffer));
            continue
          }
        }
      },
    }
    chars.remove(0);
  }
  result

}

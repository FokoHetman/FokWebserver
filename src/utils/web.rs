use std::fs;



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
  let mut parser = Parser {tokens: vec![]};
  let mut env =  Env{data: args};
  for i in tokens {
    result += &match i {
      TemplateToken::Irrelevant(s) => s,
      TemplateToken::CodeBlock(s) => template_lang(s, &mut parser.clone(), &mut env),
    }
  }
  result
}

pub fn template_lang(string: String, parser: &mut Parser, env: &mut Env) -> String {
  let tokens = tokenize_lang(string);
  println!("{:#?}", tokens);
  
  parse_lang(tokens, parser, env)
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
  OSparen,
  CSparen,
  Operator(Operator),
}


#[derive(Debug,PartialEq,Clone)]
pub enum Operator {
  Addition,
  Substraction,
  Multiplication,
  Division,
  Exponentiation,
  Indexation,
  Range,//..
}

#[derive(Debug,PartialEq,Clone)]
pub enum Node {
  BinaryOperation(Box<Node>, Box<Node>, Operator),
  Integer(i32),
  Float(f64),
  Str(String),
  List(Vec<Box<Node>>),

  Identifier(String),

  Indexation(Box<Node>, Box<Node>),
}

#[derive(Debug,Clone,PartialEq)]
pub enum Fructa {
  Numerum(f64),
  Str(String),
  Inventarii(Vec<Box<Fructa>>),
  Dictario(Vec<(Box<Fructa>, Box<Fructa>)>),
  Nullus
}

impl Fructa {
  fn display(&self) -> String {
    match self {
      Fructa::Str(s) => s.to_string(),
      Fructa::Numerum(n) => n.to_string(),
      Fructa::Inventarii(v) => {
        let mut result = String::new();
        for i in v {
          result += &i.display();
        }
        result
      },
      _ => String::new()
    }

  }
}

pub fn parse_lang(tokens: Vec<Token>, parser: &mut Parser, env: &mut Env) -> String {
  let tokens = tokens.clone();

  let mut nodes: Vec<Node> = vec![];
  parser.tokens = tokens.clone();

  while parser.tokens.len()>0 {
    let panics = std::panic::catch_unwind(|| {let mut parser = parser.clone(); let val = parser.rparse(); (parser.tokens.clone(), val)});
    if panics.is_ok()  {
      let uw = panics.unwrap();
      nodes.push(uw.1);
      parser.tokens = uw.0;
      //println!("!!!: {:#?}", parser.tokens)
    } else {
      return "<code>CodeBlock Error</code>".to_string();
    }

  }
  println!("{:#?}", nodes);

  let mut last_fruit = Fructa::Nullus;
  for node in nodes {
    last_fruit = evaluate(node, env);
  }
  
  

  last_fruit.display()
}

fn mul_str(str: String, n: i32) -> String {
  let mut res = String::new();
  for _i in 0..n {
    res+=&str.clone();
  }
  res
}

pub struct Env {
  data: Vec<(String, Fructa)>,
}
impl Env {
  fn get(&self, id: String) -> Fructa {
    for i in &self.data {
      if i.0==id {
        return i.1.clone();
      }
    }
    return Fructa::Nullus
  }
}


pub fn evaluate(node: Node, env: &mut Env) -> Fructa {
  match node {
    Node::Str(string) => Fructa::Str(string.replace("\n", "<br>")),
    Node::Integer(i) => Fructa::Numerum(i as f64),
    Node::Float(f) => Fructa::Numerum(f),
    Node::Identifier(id) => env.get(id),
    Node::Indexation(l, r) => {
      let d = evaluate(*l, env);
      println!("{:#?}", d);
      match d {
        Fructa::Dictario(d) => {
          let rstr = match *r {
            Node::Str(s) => env.get(s),
            Node::Identifier(s) => Fructa::Str(s),
            _ => panic!("?")
          };
          match rstr {
            Fructa::Str(s) => {
              println!("{}", s);
              for i in d.clone() {
                match *i.0 {
                  Fructa::Str(s2) => {
                    println!("{}", s2);
                    if s==s2 {
                      return *i.1;
                    }
                  }
                  _ => panic!("?")
                }
              }
              println!("{:#?}", d);
              panic!("not found")
            },
            _ => panic!("?")
          }

        }
        _ => panic!("not a dict")
      }
    },
    Node::BinaryOperation(l, r, o) => {
      match o {
        /*Operator::Indexation => {
          
        },*/
        Operator::Addition => {
          match evaluate(*l, env) {
            Fructa::Numerum(i) => {

              match evaluate(*r, env) {
                Fructa::Numerum(i2) => Fructa::Numerum(i2  +  i),
                Fructa::Str(s) => Fructa::Str(i.to_string() + &s),
                _ => panic!("not supported operation")
              }
            },
            Fructa::Str(s) => {
              match evaluate(*r, env) {
                Fructa::Numerum(i2) => Fructa::Str(s + &i2.to_string()),
                Fructa::Str(s2) => Fructa::Str(s + &s2),
                _ => panic!("not supported operation")
              }
            },

            _ => panic!("not supported operation")
          }
        },
        Operator::Substraction => {
          match evaluate(*l, env) {
            Fructa::Numerum(i) => {

              match evaluate(*r, env) {
                Fructa::Numerum(i2) => Fructa::Numerum(i  -  i2),
                _ => panic!("not supported operation")
              }
            },
            _ => panic!("not supported operation")
          }
        },
        Operator::Multiplication => {
          match evaluate(*l, env) {
            Fructa::Numerum(i) => {

              match evaluate(*r, env) {
                Fructa::Numerum(i2) => Fructa::Numerum(i2  *  i),
                Fructa::Str(s) => Fructa::Str(mul_str(s, i as i32)),
                _ => panic!("not supported operation")
              }
            },
            Fructa::Str(s) => {
              match evaluate(*r, env) {
                Fructa::Numerum(i2) => Fructa::Str(s + &i2.to_string()),
                _ => panic!("not supported operation")
              }
            },

            _ => panic!("not supported operation")
          }
        },
        Operator::Division => {
          match evaluate(*l, env) {
            Fructa::Numerum(i) => {

              match evaluate(*r, env) {
                Fructa::Numerum(i2) => Fructa::Numerum(i  /  i2),
                _ => panic!("not supported operation")
              }
            },
            _ => panic!("not supported operation")
          }
        },
        _ => panic!("no impl")
      }
    },
    _ => panic!("unknown node")
  }
}

#[derive(Clone)]
pub struct Parser {
  tokens: Vec<Token>
}
impl Parser {
  pub fn eat(&mut self) -> Token {
    let value = self.tokens[0].clone();
    self.tokens.remove(0);
    value
  }
  pub fn eat_expect(&mut self, expected: Token) -> Token {
    let value = self.tokens[0].clone();
    if value != expected {
      panic!("Unmatched token: {:#?}, expected: {:#?}", value, expected);
    }
    self.tokens.remove(0);
    value
  }
  pub fn rparse(&mut self) -> Node {
    let left = self.parse_addition();
    left
  }

  pub fn parse_addition(&mut self) -> Node {
    let mut left = self.parse_multiplication();
    
    while self.tokens.len()>0 && [Token::Operator(Operator::Addition), Token::Operator(Operator::Substraction)].contains(&self.tokens[0]) {
      let operator = match self.eat() {
        Token::Operator(o) => o,
        _ => panic!("no")
      };
      let rparse = self.parse_multiplication();
      left = Node::BinaryOperation(Box::new(left), Box::new(rparse), operator);
    }
    left
  }
  pub fn parse_multiplication(&mut self) -> Node {
    let mut left = self.parse_indexation();
    while self.tokens.len()>0 && [Token::Operator(Operator::Multiplication), Token::Operator(Operator::Division)].contains(&self.tokens[0]) {
      let operator = match self.eat() {
        Token::Operator(o) => o,
        _ => panic!("no")
      };
      let rparse = self.parse_indexation();
      left = Node::BinaryOperation(Box::new(left), Box::new(rparse), operator);
    }
    left

  }
  pub fn parse_indexation(&mut self) -> Node {
    let mut left = self.parse_primary();
    while self.tokens.len()>0 && [Token::Operator(Operator::Indexation)].contains(&self.tokens[0]) {
      let _ = self.eat();
      left = Node::Indexation(Box::new(left), Box::new(self.parse_primary()));
    }
    left
  }

  pub fn parse_primary(&mut self) -> Node {
    let value = self.eat(); // same as token
    //println!("{:#?}::{:#?}", value, self.tokens);
    match value {
      Token::Oparen => {
        
        let value = self.rparse();
        //panic!("{:#?}::{:#?}", value, self.tokens);
        self.eat_expect(Token::Cparen);
        value
      },
      Token::Int(i) => Node::Integer(i),
      Token::Float(f) => Node::Float(f),
      Token::Identifier(id) => Node::Identifier(id),
      Token::Str(s) => Node::Str(s),
      _ => panic!("Invalid primary token: {:#?}", value),
    }
  }
}




pub fn tokenize_lang(string: String) -> Vec<Token> {
  let mut chars = string.chars().collect::<Vec<char>>();

  let mut result: Vec<Token> = vec![];
  while chars.len()>0 {
    println!("{}", chars[0]);
    match chars[0] {
      '(' => {result.push(Token::Oparen)},
      ')' => {result.push(Token::Cparen)},
      '{' => {result.push(Token::OCparen)},
      '}' => {result.push(Token::CCparen)},
      '[' => {result.push(Token::OSparen)},
      ']' => {result.push(Token::CSparen)},
      '+' => {result.push(Token::Operator(Operator::Addition))},
      '-' => {result.push(Token::Operator(Operator::Substraction))},
      '*' => {result.push(Token::Operator(Operator::Multiplication))},
      '/' => {result.push(Token::Operator(Operator::Division))},
      '.' => {result.push(Token::Operator(Operator::Indexation))},
      '\"' => {
        let mut buffer = String::new();
        chars.remove(0);
        while chars[0]!='"' {
          if chars[0]=='\\' && chars[1]=='"' {
            chars.remove(0);
          }
          buffer+=&chars[0].to_string();
          chars.remove(0);
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
          if chars.len()>0 && chars[0]=='.' {
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
          if chars[0].is_alphabetic() || ['_'].contains(&chars[0]) {
            let mut buffer = String::new();
            while chars.len()>0 && (chars[0].is_alphabetic() || ['_'].contains(&chars[0])) {
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

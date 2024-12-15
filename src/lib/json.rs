#[derive(Debug,PartialEq)]
pub enum JsonToken {
  OpenCursive,      // {
  CloseCursive,     // }
  OpenList,         // p
  CloseList,        // ]

  String(String),   // "h"
  Number(i64),      // 123
  Bool(bool),       // true/false
  Null,             // null
  Separator,        // ,
  Colon,            // :
  EOF,
}
#[derive(Debug, Clone, PartialEq)]
pub enum JsonElem {
  List(Vec<JsonElem>),
  Dict(Vec<(JsonElem, JsonElem)>),
  String(String),
  Number(i64),
  Bool(bool),
  Null,
  Ignore,
}
#[derive(Debug)]
pub struct Json {
  pub body: JsonElem,
}

impl JsonElem {
  pub fn get(&self, key: &str) -> JsonElem {
    let key = JsonElem::String(key.to_string());
    match &self {
      JsonElem::Dict(d) => {
        let mut result = JsonElem::Ignore;
        for i in d {
          if i.0 == key {
            result = i.1.clone();
          }
        }
        result
      }
      _ => panic!("not a dict")
    }
  }
  pub fn has(&self, key: &str) -> bool {
    let key = JsonElem::String(key.to_string());
    match &self {
      JsonElem::Dict(d) => {
        for i in d {
          if i.0 == key {
            return true;
          }
        }
      }
      _ => {return false;},
    }
    false
  }
  pub fn get_list(&self) -> Vec<JsonElem> {
    match &self {
      JsonElem::List(l) => l.to_vec(),
      _ => panic!("not a list")
    }
  }
  pub fn display(&self) -> String {
    match &self {
      JsonElem::String(s) => s.to_string(),
      JsonElem::Number(i) => i.to_string(),
      JsonElem::Ignore => String::new(),
      JsonElem::Null => String::from("null"),
      _ => String::from("lazy")
    }
  }
  pub fn get_i64(&self) -> i64 {
    match &self {
      JsonElem::Number(i) => *i,
      _ => panic!("not a num")
    }
  }
  pub fn get_str(&self) -> String {
    match &self {
      JsonElem::String(s) => s.to_string(),
      _ => panic!("not str")
    }
  }
  //panic!("non-exhausive")
}


pub fn parse_json(json: String) -> Json {
  //println!("{}", json);
  let mut tokens: Vec<JsonToken> = vec![];
  let mut tokenstr = json.chars().collect::<Vec<char>>();
  let mut identifiable = false;
  while tokenstr.len() > 0 {
    //println!("prog: {:#?}", tokens);
    identifiable = false;
    match tokenstr[0] {
      '{' => {tokens.push(JsonToken::OpenCursive);},
      '}' => {tokens.push(JsonToken::CloseCursive);},
      '[' => {tokens.push(JsonToken::OpenList);},
      ']' => {tokens.push(JsonToken::CloseList);},
      '"' => {
        let mut completed = String::new();
        tokenstr.remove(0);
        let mut ltoken = ' ';
        while tokenstr.len()>0 && !(tokenstr[0] == '"' && ltoken!='\\') {
          completed+=&tokenstr[0].to_string();
          ltoken = tokenstr[0];
          tokenstr.remove(0);
        }
        //tokenstr.remove(0);
        tokens.push(JsonToken::String(completed));
      },
      ',' => {tokens.push(JsonToken::Separator);},
      ':' => {tokens.push(JsonToken::Colon);},
      _ => {
        identifiable = true;
        if tokenstr[0].is_numeric() {
          let mut buf = String::new();
          while tokenstr[0].is_numeric() {
            buf+=&tokenstr[0].to_string();
            tokenstr.remove(0);
          }
          tokens.push(JsonToken::Number(buf.parse::<i64>().unwrap()));
        } else {
          
          let mut identifier = String::new();
          while tokenstr.len()>0 {
            if !"{}[]\",:1234567890 ".chars().collect::<Vec<char>>().contains(&tokenstr[0]) {
              identifier += &tokenstr[0].to_string();
              tokenstr.remove(0);
            } else if tokenstr[0] == ' ' {
              tokenstr.remove(0);
            } else { 
              break
            }
          }
          match &identifier as &str {
            "true" => {
              tokens.push(JsonToken::Bool(true));
            },
            "false" => {
              tokens.push(JsonToken::Bool(false));
            },
            "null" => {
              tokens.push(JsonToken::Null);
            },
            _ => {}//println!("{}", &identifier)
          }
          //println!("interesting never parsed token: {}", tokenstr[0]);
        }

      }
    };
    if tokenstr.len()>0 && !identifiable {
      tokenstr.remove(0);
    }
  }
  tokens.push(JsonToken::EOF);
  //println!("{:#?}", tokens);
  //panic!("h");
  let mut result = JsonElem::String("error".to_string());
  while tokens[0] != JsonToken::EOF {
    result = parse_token(&mut tokens);
    tokens.remove(0);
    //println!("RESULT: {:#?}", result);
  };
  Json{body: result}
}
fn parse_token(tokens: &mut Vec<JsonToken>) -> JsonElem {
  //println!("PARSING: {:#?}; because tokens: {:#?}", tokens[0], tokens);
  match &tokens[0] {
    JsonToken::OpenCursive => {
      let mut data: Vec<(JsonElem, JsonElem)> = vec![];
      tokens.remove(0);     // remove {
      while tokens[0] != JsonToken::CloseCursive {

        let val = parse_token(tokens);
        //println!("removing (val): {:#?}", tokens[0]);

        tokens.remove(0);       // rmeove val

        //println!("removing (colon): {:#?}", tokens[0]);

        tokens.remove(0);       // remove :
        let val2 = parse_token(tokens);
        data.push((val, val2));
        tokens.remove(0);       //remove val2
        if tokens[0] == JsonToken::Separator {
          //println!("removing (sep): {:#?}", tokens[0]);
          tokens.remove(0);       // remove ,
        }
      }

      JsonElem::Dict(data)
    },
    JsonToken::OpenList => {
      let mut list: Vec<JsonElem> = vec![];

      tokens.remove(0);     // remove [

      while tokens[0] != JsonToken::CloseList {

        list.push(parse_token(tokens));
        tokens.remove(0);
        if tokens[0] == JsonToken::Separator {
          tokens.remove(0); // get  rid of Separator
        }
      }
      JsonElem::List(list)
    },
    JsonToken::String(s) => JsonElem::String(s.to_string()),
    JsonToken::Number(i) => JsonElem::Number(*i),
    JsonToken::Bool(b) => JsonElem::Bool(*b),
    JsonToken::Null => JsonElem::Null,
    _ => /*JsonElem::Ignore*/panic!("???: {:#?}, {:#?}", tokens[0], tokens)
  }
}

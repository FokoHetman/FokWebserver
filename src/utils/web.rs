use std::fs;



pub fn render_template(path: &str) -> String {
  let template = fs::read_to_string(path).unwrap();

  return template
}

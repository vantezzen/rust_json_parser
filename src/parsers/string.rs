use crate::element::Element;

pub fn parse_string(chars: &mut Vec<char>) -> Element {
  let mut contents = String::from("");
  let mut is_escaped = false;
  loop {
      if let Some(character) = chars.pop() {
          if character == '"' && is_escaped == false {
              // End of string
              break;
          } else if character == '\\' && is_escaped == false {
              // Escape character
              is_escaped = true;
          } else {
              // Character in string
              is_escaped = false;
              contents.push_str(&character.to_string());
          }

      } else {
          panic!("Unexpected end");
      }
  }

  Element::String(contents)
}
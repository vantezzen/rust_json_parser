use crate::element::Element;

pub fn parse_number(chars: &mut Vec<char>, character: char) -> Element {
  let mut contents = String::from(character.to_string());
  loop {
      if let Some(character) = chars.pop() {
          if character.is_ascii_digit() || character == '.' {
              contents.push_str(&character.to_string());
          } else {
              // End of number
              chars.push(character);
              break;
          }
      } else {
          break;
      }
  }

  Element::Number(contents)
}
use crate::parsers::array::parse_array;
use crate::parsers::object::parse_object;
use crate::parsers::string::parse_string;
use crate::parsers::number::parse_number;
use crate::parsers::constants::handle_suspected_constant;
use crate::element::Element;

pub fn parse_generic_character(chars: &mut Vec<char>, character: char) -> Option<Element> {
  if character == '"' {
      let element = parse_string(chars);
      return Some(element);
  } else if character == '{' {
      let element = parse_object(chars);
      return Some(element);
  } else if character == '[' {
      let element = parse_array(chars);
      return Some(element);
  } else if character.is_ascii_digit() || character == '-' {
      let element = parse_number(chars, character);
      return Some(element);
  } else if character == 't' {
      let element = handle_suspected_constant(chars, character, String::from("true"), Element::True);
      return Some(element);
  } else if character == 'f' {
      let element = handle_suspected_constant(chars, character, String::from("false"), Element::False);
      return Some(element);
  } else if character == 'n' {
      let element = handle_suspected_constant(chars, character, String::from("null"), Element::Null);
      return Some(element);
  }
  None
}
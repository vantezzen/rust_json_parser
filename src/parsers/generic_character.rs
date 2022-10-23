use crate::parsers::array::parse_array;
use crate::parsers::object::parse_object;
use crate::parsers::string::parse_string;
use crate::parsers::number::parse_number;
use crate::parsers::constants::handle_suspected_constant;
use crate::element::Element;

pub fn parse_generic_character(chars: &mut Vec<char>, character: char) -> Option<Element> {
  return match character {
    '"' => Some(parse_string(chars)),
    '{' => Some(parse_object(chars)),
    '[' => Some(parse_array(chars)),
    't' => Some(handle_suspected_constant(chars, character, String::from("true"), Element::True)),
    'f' => Some(handle_suspected_constant(chars, character, String::from("false"), Element::False)),
    'n' => Some(handle_suspected_constant(chars, character, String::from("null"), Element::Null)),
    '-' | '0'..='9' => Some(parse_number(chars, character)),
    _ => None
  }
}
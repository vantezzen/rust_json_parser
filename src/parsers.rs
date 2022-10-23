mod array;
mod constants;
mod generic_character;
mod number;
mod object;
mod string;

use crate::element::Element;
use generic_character::parse_generic_character;

pub fn parse_string(string: String) -> Result<Element, String> {
    let mut chars: Vec<_> = string.chars().collect();
    return parse_characters(&mut chars);
}

pub fn parse_characters(chars: &mut Vec<char>) -> Result<Element, String> {
    // Chars need to be reversed so we can use ".pop()"
    chars.reverse();

    let total_character_length = chars.len();

    let mut element: Result<Element, String> = Ok(Element::Empty);
    if let Some(character) = chars.pop() {
        if let Some(generic_element) = parse_generic_character(chars, character) {
            element = generic_element;
        }
    }

    if let Err(message) = element {
        let char_index = total_character_length - chars.len();
        element = Err(format!("{} (at char {})", message, char_index));
    }

    element
}

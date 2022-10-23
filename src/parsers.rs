mod array;
mod constants;
mod generic_character;
mod number;
mod object;
mod string;

use crate::element::Element;
use crate::errors::add_error_context;
use generic_character::parse_generic_character;

pub fn parse_string(string: String, add_full_error_context: bool) -> Result<Element, String> {
    let mut chars: Vec<_> = string.chars().collect();
    return parse_characters(&mut chars, add_full_error_context);
}

pub fn parse_characters(chars: &mut Vec<char>, add_full_error_context: bool) -> Result<Element, String> {
    // Used for creating better error messages
    let all_chars = chars.clone();

    // Chars need to be reversed so we can use ".pop()"
    chars.reverse();

    let mut element: Result<Element, String> = Ok(Element::Empty);
    if let Some(character) = chars.pop() {
        if let Some(generic_element) = parse_generic_character(chars, character) {
            element = generic_element;
        }
    }

    if let Err(message) = element {
        element = Err(add_error_context(chars, all_chars, message, add_full_error_context));
    }

    element
}

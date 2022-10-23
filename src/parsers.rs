mod array;
mod constants;
mod generic_character;
mod number;
mod object;
mod string;

use crate::element::Element;
use generic_character::parse_generic_character;

pub fn parse_characters(chars: &mut Vec<char>) -> Element {
    let mut element = Element::Empty;
    loop {
        if let Some(character) = chars.pop() {
            if let Some(generic_element) = parse_generic_character(chars, character) {
                element = generic_element;
                break;
            }
        } else {
            break;
        }
    }
    element
}

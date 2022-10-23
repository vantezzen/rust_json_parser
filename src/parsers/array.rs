use crate::element::Element;
use crate::parsers::generic_character::parse_generic_character;

pub fn parse_array(chars: &mut Vec<char>) -> Element {
    let mut array_contents: Vec<Element> = Vec::new();

    loop {
        let character = chars.pop().expect("Unexpected end of object");

        match character {
            // New element
            ',' => {}

            // End of array
            ']' => {
                break;
            }

            // Character that doesn't control the object structure
            generic_character => {
                let generic_element_option = parse_generic_character(chars, generic_character);

                if let Some(generic_element) = generic_element_option {
                    array_contents.push(generic_element);
                }
            }
        }
    }

    Element::Array(array_contents)
}

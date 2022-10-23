use crate::element::Element;
use crate::parsers::generic_character::parse_generic_character;

pub fn parse_array(chars: &mut Vec<char>) -> Result<Element, String> {
    let mut array_contents: Vec<Element> = Vec::new();
    let mut has_element_in_slot = false;

    loop {
        let character = chars.pop().expect("Unexpected end of object");

        match character {
            // New element
            ',' => {
                if !has_element_in_slot {
                    return Err("Duplicate comma in array declaration".to_string());
                }
                has_element_in_slot = false;
            }

            // End of array
            ']' => {
                break;
            }

            // Character that doesn't control the object structure
            generic_character => {
                let generic_element_option = parse_generic_character(chars, generic_character);

                if let Some(generic_element) = generic_element_option {
                    match generic_element {
                        Ok(generic_element) => {
                            if has_element_in_slot {
                                return Err("Missing comma in array".to_string());
                            }

                            array_contents.push(generic_element);
                            has_element_in_slot = true;
                        },
                        Err(message) => {
                            return Err(message);
                        }
                    }
                }
            }
        }
    }

    Ok(Element::Array(array_contents))
}

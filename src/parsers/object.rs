use crate::element::Element;
use crate::parsers::generic_character::parse_generic_character;
use std::collections::HashMap;

#[derive(PartialEq)]
enum ObjectParseMode {
    Key,
    Value,
}

pub fn parse_object(chars: &mut Vec<char>) -> Result<Element, String> {
    let mut object_map: HashMap<String, Element> = HashMap::new();
    let mut mode = ObjectParseMode::Key;
    let mut key: Option<String> = None;

    loop {
        let character = chars.pop().expect("Unexpected end of object");

        match character {
            // Switch from key to value
            ':' => {
                if mode == ObjectParseMode::Value {
                    return Err("Unexpected colon in value".to_string());
                } else if key == None {
                    return Err("Colon in object but no key declared".to_string());
                } else {
                    mode = ObjectParseMode::Value;
                }
            }
            // Switch from value to another key
            ',' => {
                if mode == ObjectParseMode::Key {
                    return Err("Unexpected comma in object".to_string());
                } else if key != None {
                    return Err("Unexpected comma before value declaration".to_string());
                } else {
                    mode = ObjectParseMode::Key;
                }
            }

            // End of object
            '}' => {
                break;
            }

            // Character that doesn't control the object structure
            generic_character => {
                let generic_element_option = parse_generic_character(chars, generic_character);

                if let Some(generic_element) = generic_element_option {
                    
                    match generic_element {
                        Ok(generic_element) => {
                            if mode == ObjectParseMode::Key {
                                // Found key
                                if let Element::String(key_string) = generic_element {
                                    key = Some(key_string);
                                } else {
                                    return Err("Invalid object key type".to_string());
                                }
                            } else {
                                // Found value for the given key
                                object_map.insert(key.expect("No key for object value"), generic_element);
                                key = None;
                            }
                        },
                        Err(message) => {
                            return Err(message)
                        }
                    }
                }
            }
        }
    }

    Ok(Element::Object(object_map))
}

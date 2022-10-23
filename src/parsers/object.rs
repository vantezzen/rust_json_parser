use crate::parsers::generic_character::parse_generic_character;
use std::collections::HashMap;
use crate::element::Element;

#[derive(PartialEq)]
enum ObjectParseMode {
    Key,
    Value,
}

pub fn parse_object(chars: &mut Vec<char>) -> Element {
    let mut object_map: HashMap<String, Element> = HashMap::new();
    let mut mode = ObjectParseMode::Key;
    let mut key: Option<String> = None;

    loop {
        let character = chars.pop().expect("Unexpected end of object");

        match character {
            // Switch from key to value
            ':' => {
                if mode == ObjectParseMode::Value {
                    panic!("Unexpected colon in value");
                } else if key == None {
                    panic!("Colon in object but no key declared");
                } else {
                    mode = ObjectParseMode::Value;
                }
            },
            // Switch from value to another key
            ',' => {
                if mode == ObjectParseMode::Key {
                    panic!("Unexpected comma in object");
                } else if key != None {
                    panic!("Unexpected comma before value declaration");
                } else {
                    mode = ObjectParseMode::Key;
                }
            },

            // End of object
            '}' => {
                break;
            },

            // Character that doesn't control the object structure
            generic_character => {
                let generic_element_option = parse_generic_character(chars, generic_character);

                if let Some(generic_element) = generic_element_option {
            
                    if mode == ObjectParseMode::Key {
                        // Found key
                        if let Element::String(key_string) = generic_element {
                            key = Some(key_string);
                        } else {
                            panic!("Invalid object key type");
                        }
                    } else {
                        // Found value for the given key
                        object_map.insert(key.expect("No key for object value"), generic_element);
                        key = None;
                    }
                }
            }
        }
    }

    Element::Object(object_map)
}
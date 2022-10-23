use crate::element::Element;
use std::collections::HashMap;

pub fn parse_item(chars: &mut Vec<char>) -> Element {
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

fn parse_generic_character(chars: &mut Vec<char>, character: char) -> Option<Element> {
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
    }
    None
}

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

pub fn parse_array(chars: &mut Vec<char>) -> Element {
    let mut array_contents: Vec<Element> = Vec::new();

    loop {
        let character = chars.pop().expect("Unexpected end of object");

        match character {
            // New element
            ',' => {
                
            },

            // End of array
            ']' => {
                break;
            },

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

pub fn parse_string(chars: &mut Vec<char>) -> Element {
    let mut contents = String::from("");
    let mut is_escaped = false;
    loop {
        if let Some(character) = chars.pop() {
            if character == '"' && is_escaped == false {
                // End of string
                break;
            } else if character == '\\' && is_escaped == false {
                // Escape character
                is_escaped = true;
            } else {
                // Character in string
                is_escaped = false;
                contents.push_str(&character.to_string());
            }

        } else {
            panic!("Unexpected end");
        }
    }

    Element::String(contents)
}

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

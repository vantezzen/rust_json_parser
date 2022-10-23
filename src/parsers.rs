use crate::element::Element;
use std::collections::HashMap;

pub fn parse_item(chars: &mut Vec<char>) -> Element {
    let mut element = Element::Empty;
    loop {
        if let Some(character) = chars.pop() {
            if character == '"' {
                element = parse_string(chars);
                break;
            } else if character == '{' {
                element = parse_object(chars);
                break;
            }
        } else {
            break;
        }
    }
    element
}

#[derive(PartialEq)]
enum ObjectParseMode {
    Key,
    Value,
}

pub fn parse_object(chars: &mut Vec<char>) -> Element {
    let mut key = String::from("");

    let mut object_map: HashMap<String, Element> = HashMap::new();
    let mut mode = ObjectParseMode::Key;

    loop {
        if let Some(character) = chars.pop() {
            if character == '"' {
                let contents = parse_string(chars);

                if mode == ObjectParseMode::Key {
                    if let Element::String(key_string) = contents {
                        key = key_string;
                    } else {
                        panic!("Invalid object key type");
                    }
                } else {
                    let key_string = key.clone();
                    object_map.insert(key_string, contents);
                }
            } else if character == '{' {
                if mode == ObjectParseMode::Value {
                    let contents = parse_object(chars);
                    let key_string = key.clone();
                    object_map.insert(key_string, contents);
                } else {
                    panic!("Invalid Object start in object key position");
                }
                break;
            } else if character == ':' {
                if mode == ObjectParseMode::Key {
                    mode = ObjectParseMode::Value;
                } else {
                    panic!("Unexpected colon in object");
                }
            } else if character == ',' {
                if mode == ObjectParseMode::Value {
                    mode = ObjectParseMode::Key;
                } else {
                    panic!("Unexpected comma in object");
                }
            } else if character == '}' {
                break;
            }
        } else {
            panic!("Unexpected end");
        }
    }

    Element::Object(object_map)
}

pub fn parse_string(chars: &mut Vec<char>) -> Element {
    let mut contents = String::from("");
    loop {
        if let Some(character) = chars.pop() {
            if character == '"' {
                break;
            }
            contents.push_str(&character.to_string());
        } else {
            panic!("Unexpected end");
        }
    }

    Element::String(contents)
}

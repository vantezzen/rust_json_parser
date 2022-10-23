use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Element {
    String(String),
    Object(HashMap<String, Element>),
    Array(Vec<Element>),
    Number(String), // String is used as numbers may be any format or size
    True,
    False,
    Null,
    Empty,
}

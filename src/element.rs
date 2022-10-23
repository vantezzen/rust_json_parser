use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Element {
    String(String),
    Object(HashMap<String, Element>),
    Empty,
}

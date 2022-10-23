use crate::element::Element;

pub fn handle_suspected_constant(
    chars: &mut Vec<char>,
    character: char,
    suspected_constant: String,
    suspected_element: Element,
) -> Result<Element, String> {
    let mut contents = String::from(character.to_string());
    loop {
        if let Some(character) = chars.pop() {
            contents.push_str(&character.to_string());

            if contents != suspected_constant[0..contents.len()] {
                return Err(format!("Unknown constant '{}', did you mean '{}'?", contents, suspected_constant));
            }
            if contents.len() == suspected_constant.len() {
                break;
            }
        } else {
            break;
        }
    }

    Ok(suspected_element)
}

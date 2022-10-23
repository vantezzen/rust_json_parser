pub fn add_error_context(chars: &Vec<char>, all_chars: Vec<char>, error_message: String, add_full_error_context: bool) -> String {
  let total_character_length = all_chars.len();
  let char_index = total_character_length - chars.len();

  let mut full_context = String::from("");
  if add_full_error_context {
    full_context = get_json_context(&String::from_iter(all_chars), char_index);
  }

  return format!("{} (at char {})\n\n\n{}", error_message, char_index, full_context);
}

fn get_json_context(json: &String, position: usize) -> String {
  let error_line_number = get_line_number_of_char_position(&json, position);

  let lines: Vec<_> = json.lines().collect();
  let mut final_context = String::from("");
  let start_line = if error_line_number > 3 { error_line_number - 3 } else { 0 };

  for (line_number, line_contents) in lines.iter().enumerate() {
    if line_number >= start_line && line_number <= error_line_number + 2 {
      final_context.push_str(line_contents);
      if line_number == error_line_number - 1 {
        final_context.push_str("   <---- Error");
      }
      final_context.push('\n');
    }
  }

  final_context
}

fn get_line_number_of_char_position(text: &String, position: usize) -> usize {
  let mut line_number = 0;

  for (index, character) in text.chars().enumerate() {
    if character == '\n' {
      line_number += 1;
    }
    if index == position {
      break;
    }
  }

  line_number
}
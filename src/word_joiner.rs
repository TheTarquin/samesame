extern crate rand;
use self::rand::Rng;

use std::string::String;

pub fn map(input: String) -> String {
  let mut output = String::new();
  let mut input_chars = input.chars().peekable();
  while input_chars.peek() != None {
      let next = input_chars.next().unwrap();
      output.push(next);
      //TODO: should we also insert legacy ZWNBS (\xFEFF)?
      if rand::thread_rng().gen_range(0,10) == 0 {
          output.push('\u{2060}');
      }
  }
  return output;
}

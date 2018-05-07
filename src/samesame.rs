use std::io;
mod english_confusables;
//mod confusables;

fn main() {
  let mut input = String::new();
  let mut output = String::new();

//  confusables::initialize();

  match io::stdin().read_line(&mut input) {
    Ok(_n) => {
      let mut input_chars = input.chars().peekable();
      while input_chars.peek() != None {
        output.push(input_chars.next().unwrap());
        output.push('\u{0307}');
      }
      print!("{}", output);
    }
    Err(error) => println!("error: {}", error),

  }
}

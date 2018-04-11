use std::io;

fn main() {
  let mut input = String::new();
  let mut output = String::new();

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

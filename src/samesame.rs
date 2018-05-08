use std::io;
mod english_confusables;
//mod confusables;

fn main() {
  let mut input = String::new();
  let mut output = String::new();

//  confusables::initialize();

  match io::stdin().read_line(&mut input) {
    Ok(_n) => {
        output = english_confusables::map(input); 
        print!("{}", output);
    }
    Err(error) => println!("error: {}", error),

  }
}

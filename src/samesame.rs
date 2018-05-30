use std::io;
use std::env;

extern crate getopts;

mod english_confusables;

fn main() {
  let mut input = String::new();
  let mut output = String::new();

  let args: Vec<String> = env::args().collect();
  println!("{:?}", args);

  match io::stdin().read_line(&mut input) {
    Ok(_n) => {
        output = english_confusables::map(input); 
        print!("{}", output);
    }
    Err(error) => println!("error: {}", error),

  }
}

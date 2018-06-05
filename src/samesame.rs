use std::io;
use std::env;
use std::fs::File;
use std::io::prelude::*;

extern crate getopts;
use getopts::Options;

mod english_confusables;
mod discrete_english_confusables;
mod word_joiner;

fn print_usage(program: &str, opts: Options) {
    let use_text = format!("Usage: {} -i IN_FILE [options]\n       \
                           {} [options] [TEXT]", program, program);
    print!("{}", opts.usage(&use_text));
}

fn main() {
  let mut input = String::new();
  let mut output = String::new();

  let args: Vec<String> = env::args().collect();
  println!("{:?}", args);

  let mut opts = Options::new();
  opts.optflag("d", "discrete", "use discrete mode, avoiding obvious homographs");
  opts.optflag("z", "wordjoiners", "randomly insert zero-width word joiners into output");
  opts.optopt("i", "", "set input file name", "IN_FILE");
  opts.optopt("o", "", "set output file name", "OUT_FILE");
  opts.optflag("v", "verbose", "use verbose mode");
  opts.optflag("h", "help", "print help menu");

  let opt_matches = match opts.parse(&args[1..]) {
    Ok(m) => { m }
    Err(f) => { panic!(f.to_string()) }
  };

  if opt_matches.opt_present("h") {
    print_usage(&args[0].clone(), opts);
    return;
  }

  if opt_matches.opt_present("i") {
      let in_file = opt_matches.opt_str("i");
      let file = File::open(in_file.unwrap());
      //TODO: match on result and do error handling
      file.unwrap().read_to_string(&mut input);
  } else if !opt_matches.free.is_empty() {
      input = opt_matches.free[0].clone();
      input.push('\n');
  } else {
      match io::stdin().read_line(&mut input) {
          Ok(_n) => {
          }
          Err(f) => { panic!(f.to_string()) }
      };
  }

  if opt_matches.opt_present("d") {
    output = discrete_english_confusables::map(input);
    if opt_matches.opt_present("v") {
        println!("using discrete english map");
    }
  } else {
    output = english_confusables::map(input); 
    if opt_matches.opt_present("v") {
        println!("using regular english map");
    }
  }

  //additionally randomly insert ZWNBS/Word joiners
  //TODO: this is messy mapping output to output.
  //      Figure out a way to build list of maps to apply
  if opt_matches.opt_present("z") {
      output = word_joiner::map(output);
  }

  if opt_matches.opt_present("o") {
      let out_file = opt_matches.opt_str("o");
      let file = File::create(out_file.unwrap());
      //TODO: match on result and do error handling
      file.unwrap().write_all(output.as_bytes());
    } else {
      print!("{}", output);
    }
}

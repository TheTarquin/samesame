use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub fn initialize() {

    let mut source_to_confusables = HashMap::new();

    let mut file_contents = String::new();
    let mut f = File::open("confusables.txt").expect("file not found");
    f.read_to_string(&mut file_contents);
    let pairs = file_contents.split("\n");

    for (_pair_num, pair) in pairs.enumerate() {
      let mut pair_iter = pair.split(",");
      let source = pair_iter.next();
      let confusable = pair_iter.next();
      source_to_confusables.insert(source, confusable);
    }
}

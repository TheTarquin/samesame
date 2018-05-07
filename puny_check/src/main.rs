extern crate url;
extern crate regex;
extern crate csv;
use regex::Regex;
use url::idna::punycode::decode_to_string;
use std::io::{BufReader,BufRead};
use std::fs::File;

fn main() {

    ///Build Confusable Hash map.  Key is unicode char that is confused, Value is finding.
    let mut confusable = HashMap::new();
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
       confusable.insert(&result[0], &result[1]); 
    }
    
    // Our puny file is going to be something like xn----chxc.com.  strip the leading xn-- and .com.  The regex does that
    let punyRegex = Regex::new(r"xn--(.*)\.[A-Za-z]+\.").unwrap()
    let punyFile = File::open("../../puny2").unwrap();
    for line in BufReader::new(punyFile).lines() {
       for cap in punyRegex.captures_iter(&line.unwrap()) {
          let mut puny: Option<String> = decode_to_string(&cap[1]);
          let mut punyVal = puny.as_ref().map(|x| &**x).unwrap_or("unwrap fail");
          for c in punyVal.chars() {
            print!("{}, ", c.escape_unicode());
// Convert char c to unicode value.
// Lookup unicode in lookup table.
// convert unicode to printable chars 
          }
       }
    }
}

extern crate rand;
use self::rand::Rng;

use std::string::String;
use std::collections::HashMap;

pub fn map(input: String) -> String {
        let mut confusables = HashMap::new();
    
    confusables.insert(' ', vec!['\u{00A0}','\u{1680}','\u{2000}','\u{2001}','\u{2002}','\u{2003}','\u{2004}','\u{2005}','\u{2006}','\u{2007}','\u{2008}','\u{2009}','\u{200A}','\u{202F}','\u{205F}','\u{3000}']);

    let mut output = String::new();
    let mut input_chars = input.chars().peekable();
    //TODO: do we want rate of swapping to be configurable?
    while input_chars.peek() != None {
        let next = input_chars.next().unwrap();
        let next_confusables = confusables.get(&next);
        match next_confusables {
            Some(next_confusables) => {
                let next_out = rand::thread_rng().choose(next_confusables);
                output.push(*next_out.unwrap());
            }    
            None => {
                output.push(next);
                continue;
            }
        }
    }
    return output;
}

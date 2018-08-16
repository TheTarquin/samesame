extern crate rand;
use self::rand::Rng;

use std::string::String;
use std::collections::HashMap;

pub fn map(input: String) -> String {
        let mut confusables = HashMap::new();
    
    confusables.insert('A', vec!['\u{0391}','\u{0410}','\u{FF21}']);

    confusables.insert('B', vec!['\u{0392}','\u{FF22}']);

    confusables.insert('C', vec!['\u{1D5A2}','\u{1D672}','\u{0421}','\u{0187}','\u{FF23}']);

    confusables.insert('D', vec!['\u{1D673}','\u{13A0}','\u{15DE}','\u{FF24}']);

    confusables.insert('E', vec!['\u{0395}','\u{FF25}']);

    confusables.insert('F', vec!['\u{FF26}']);

    confusables.insert('G', vec!['\u{FF27}']);

    confusables.insert('H', vec!['\u{0397}','\u{041D}','\u{FF28}']);

    confusables.insert('I', vec!['\u{FF29}']);

    confusables.insert('J', vec!['\u{FF2A}']);

    confusables.insert('K', vec!['\u{FF2B}']);

    confusables.insert('L', vec!['\u{FF2C}']);

    confusables.insert('M', vec!['\u{FF2D}']);

    confusables.insert('N', vec!['\u{FF2E}']);

    confusables.insert('O', vec!['\u{FF2F}']);

    confusables.insert('P', vec!['\u{FF30}']);

    confusables.insert('Q', vec!['\u{FF31}']);

    confusables.insert('R', vec!['\u{FF32}']);

    confusables.insert('S', vec!['\u{FF33}']);

    confusables.insert('T', vec!['\u{FF34}']);

    confusables.insert('U', vec!['\u{FF35}']);

    confusables.insert('V', vec!['\u{FF36}']);

    confusables.insert('W', vec!['\u{FF37}']);

    confusables.insert('X', vec!['\u{FF38}']);

    confusables.insert('Y', vec!['\u{FF39}']);

    confusables.insert('Z', vec!['\u{0396}','\u{FF3A}']);

    confusables.insert('a', vec!['\u{FF41}']);

    confusables.insert('b', vec!['\u{FF42}']);

    confusables.insert('c', vec!['\u{FF43}']);

    confusables.insert('d', vec!['\u{FF44}']);

    confusables.insert('e', vec!['\u{1D41E}','\u{0435}','\u{FF45}']);

    confusables.insert('f', vec!['\u{1E9D}','\u{0192}','\u{FF46}']);

    confusables.insert('g', vec!['\u{FF47}']);

    confusables.insert('h', vec!['\u{FF48}']);

    confusables.insert('i', vec!['\u{FF49}']);

    confusables.insert('j', vec!['\u{FF4A}']);

    confusables.insert('k', vec!['\u{FF4B}']);

    confusables.insert('l', vec!['\u{2223}','\u{0661}','\u{06F1}','\u{FF4C}']);

    confusables.insert('m', vec!['\u{FF4D}']);

    confusables.insert('n', vec!['\u{FF4E}']);

    confusables.insert('o', vec!['\u{0966}','\u{0BE6}','\u{0D66}','\u{0E50}','\u{1040}','\u{FF4F}']);

    confusables.insert('p', vec!['\u{FF50}']);

    confusables.insert('q', vec!['\u{FF51}']);

    confusables.insert('r', vec!['\u{FF52}']);

    confusables.insert('s', vec!['\u{1D5CC}','\u{1D600}','\u{1D634}','\u{1D69C}','\u{0455}','\u{0282}','\u{FF53}']);

    confusables.insert('t', vec!['\u{FF54}']);

    confusables.insert('u', vec!['\u{FF55}']);

    confusables.insert('v', vec!['\u{FF56}']);

    confusables.insert('w', vec!['\u{FF57}']);

    confusables.insert('x', vec!['\u{FF58}']);

    confusables.insert('y', vec!['\u{FF59}']);

    confusables.insert('z', vec!['\u{FF5A}']);


    let mut output = String::new();
    let mut input_chars = input.chars().peekable();
    while input_chars.peek() != None {
        let next = input_chars.next().unwrap();
        if rand::thread_rng().gen_range(0,10) > 0 {
            output.push(next);
            continue;
        }

        let next_confusables = confusables.get(&next);
        //TODO: do I want to make homograph density configurable? Do science to figure out the
        //      right threashold?
        match next_confusables {
            Some(next_confusables) => {
                let next_out = rand::thread_rng().choose(next_confusables);
                println!("mapping char {} to unicode point {}", next, next_out.unwrap().escape_unicode());
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

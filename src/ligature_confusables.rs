extern crate rand;
use self::rand::Rng;

use std::string::String;
use std::collections::HashMap;

pub fn map(input: String) -> String {
    // included ligatures for equivalent two character ascii

    let mut confusables = HashMap::new();

    confusables.insert(String::from("AA"), vec!['\u{A732}']);

    confusables.insert(String::from("AE"), vec!['\u{00C6}']);

    confusables.insert(String::from("AO"), vec!['\u{A734}']);

    confusables.insert(String::from("AU"), vec!['\u{A736}']);

    confusables.insert(String::from("AV"), vec!['\u{A738}']);

    confusables.insert(String::from("AY"), vec!['\u{A73C}']);

    confusables.insert(String::from("IL"), vec!['\u{1EFA}']);

    confusables.insert(String::from("OE"), vec!['\u{0152}']);

    confusables.insert(String::from("OO"), vec!['\u{A74E}']);

    confusables.insert(String::from("TZ"), vec!['\u{A728}']);

    confusables.insert(String::from("VV"), vec!['\u{0057}']);

    confusables.insert(String::from("VY"), vec!['\u{A760}']);

    confusables.insert(String::from("Hv"), vec!['\u{01F6}']);

    confusables.insert(String::from("aa"), vec!['\u{A733}']);

    confusables.insert(String::from("ae"), vec!['\u{00E6}']);

    confusables.insert(String::from("ao"), vec!['\u{A735}']);

    confusables.insert(String::from("au"), vec!['\u{A737}']);

    confusables.insert(String::from("av"), vec!['\u{A739}']);

    confusables.insert(String::from("ay"), vec!['\u{A73D}']);

    confusables.insert(String::from("et"), vec!['\u{1F670}']);

    confusables.insert(String::from("ff"), vec!['\u{FB00}']);

    confusables.insert(String::from("fi"), vec!['\u{FB01}']);

    confusables.insert(String::from("fl"), vec!['\u{FB02}']);

    confusables.insert(String::from("hv"), vec!['\u{0195}']);

    confusables.insert(String::from("oe"), vec!['\u{0153}']);

    confusables.insert(String::from("oo"), vec!['\u{A74F}']);

    confusables.insert(String::from("st"), vec!['\u{FB06}']);

    confusables.insert(String::from("tz"), vec!['\u{A729}']);

    confusables.insert(String::from("ue"), vec!['\u{1D6B}']);

    confusables.insert(String::from("uo"), vec!['\u{AB63}']);

    confusables.insert(String::from("vv"), vec!['\u{0077}']);

    confusables.insert(String::from("vy"), vec!['\u{A761}']);

    let mut output = String::new();

    let mut input_position = 0;

    while input_position < input.chars().count() - 1 {
        let next: String = input.chars().skip(input_position).take(2).collect();

        let next_confusables = confusables.get(&next);
        match next_confusables {
            Some(next_confusables) => {
                let next_out = rand::thread_rng().choose(next_confusables);
                println!("mapping chars {} to unicode point {}", next, next_out.unwrap().escape_unicode());
                output.push(*next_out.unwrap());
                input_position+=2;
            }
            None => {
                output.push(next.chars().nth(0).unwrap());
                input_position+=1;
                continue;
            }
        }
    }
    return output;  
}

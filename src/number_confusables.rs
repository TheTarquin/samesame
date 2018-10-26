extern crate rand;
use self::rand::Rng;

use std::collections::HashMap;
use std::string::String;

pub fn map(input: String) -> String {
    let mut confusables = HashMap::new();

    confusables.insert(
        '0',
        vec![ '\u{7C0}', '\u{1D6F0}', '\u{118E0}', '\u{1D5AE}', '\u{3007}', '\u{10292}', '\u{1D7EC}', '\u{1D4AA}', '\u{1D72A}', 'O', '\u{12D0}', '\u{1D4DE}', '\u{1D79E}', '\u{1D764}', '\u{2D54}', '\u{555}', '\u{1D7E2}', '\u{1D5E2}', '\u{1D7D8}', '\u{1D616}', '\u{2C9E}', '\u{41E}', '\u{39F}', '\u{B20}', '\u{1D7CE}', '\u{1D40E}', '\u{9E6}', '\u{B66}', '\u{1D512}', '\u{1D546}', '\u{1D64A}', '\u{10516}', '\u{FF2F}', '\u{114D0}', '\u{102AB}', '\u{1D67E}', '\u{A4F3}', '\u{118B5}', '\u{1D442}', '\u{10404}', '\u{1D7F6}', '\u{1D476}', '\u{1D6B6}', '\u{104C2}', '\u{1D57A}' ],
    );

    confusables.insert(
        '1',
        vec![ '\u{1D6B0}', '\u{1D62D}', '\u{406}', '\u{1D5A8}', '\u{1D425}', '\u{FE8D}', '\u{FE8E}', '\u{1D529}', '\u{2110}', '\u{2111}', '\u{1028A}', '\u{2C92}', '\u{10309}', '\u{2113}', '\u{1D724}', '\u{196}', '\u{1D798}', '\u{399}', '\u{1D695}', '\u{1D7CF}', '\u{2223}', '\u{627}', '\u{FF29}', '\u{1D5C5}', '\u{1D540}', '\u{1D644}', '\u{1D4C1}', '\u{10320}', '\u{1D43C}', '\u{1EE00}', '\u{1EE80}', '\u{5C0}', '\u{1D470}', '\u{1C0}', '\u{4C0}', '\u{16C1}', '\u{1D7ED}', '\u{1D574}', 'I', '\u{7CA}', '\u{FF4C}', '\u{1D6EA}', '\u{2D4F}', '\u{1D75E}', '\u{1D55D}', '\u{1D7E3}', '\u{5D5}', '\u{1E8C7}', '\u{1D661}', '\u{1D4D8}', '\u{1D5DC}', '\u{1D7D9}', '\u{1D459}', '\u{5DF}', '\u{2160}', '\u{1D610}', '\u{661}', '\u{1D48D}', '\u{1D591}', '\u{FFE8}', '\u{1D408}', 'l', '\u{6F1}', '\u{A4F2}', '\u{16F28}', '\u{1D678}', '\u{1D7F7}', '\u{1D4F5}', '|', '\u{217C}', '\u{23FD}', '\u{1D5F9}' ],
    );

    confusables.insert(
        '2',
        vec![ '\u{1D7D0}', '\u{1D7EE}', '\u{1D7E4}', '\u{A644}', '\u{1A7}', '\u{1D7F8}', '\u{3E8}', '\u{A75A}', '\u{1D7DA}', '\u{14BF}', '\u{A6EF}' ],
    );

    confusables.insert(
        '3',
        vec![ '\u{4E0}', '\u{1D7EF}', '\u{1D7D1}', '\u{1D206}', '\u{A76A}', '\u{1D7E5}', '\u{A7AB}', '\u{2CCC}', '\u{1B7}', '\u{417}', '\u{16F3B}', '\u{21C}', '\u{1D7DB}', '\u{1D7F9}', '\u{118CA}' ],
    );

    confusables.insert(
        '4',
        vec![ '\u{1D7F0}', '\u{118AF}', '\u{1D7D2}', '\u{1D7E6}', '\u{1D7DC}', '\u{1D7FA}', '\u{13CE}' ],
    );

    confusables.insert(
        '5',
        vec![ '\u{1D7DD}', '\u{1D7D3}', '\u{1D7F1}', '\u{1D7E7}', '\u{1BC}', '\u{1D7FB}', '\u{118BB}' ],
    );

    confusables.insert(
        '6',
        vec![ '\u{431}', '\u{1D7DE}', '\u{2CD2}', '\u{1D7D4}', '\u{118D5}', '\u{1D7F2}', '\u{1D7E8}', '\u{1D7FC}', '\u{13EE}' ],
    );

    confusables.insert(
        '7',
        vec![ '\u{1D7DF}', '\u{1D7FD}', '\u{1D212}', '\u{1D7F3}', '\u{104D2}', '\u{1D7D5}', '\u{118C6}', '\u{1D7E9}' ],
    );

    confusables.insert(
        '8',
        vec![ '\u{1031A}', '\u{222}', '\u{B03}', '\u{223}', '\u{1D7F4}', '\u{9EA}', '\u{A6A}', '\u{1E8CB}', '\u{1D7EA}', '\u{1D7E0}', '\u{1D7FE}', '\u{1D7D6}' ],
    );

    confusables.insert(
        '9',
        vec![ '\u{A67}', '\u{B68}', '\u{2CCA}', '\u{118D6}', '\u{9ED}', '\u{D6D}', '\u{1D7EB}', '\u{A76E}', '\u{1D7FF}', '\u{1D7E1}', '\u{1D7D7}', '\u{1D7F5}', '\u{118CC}', '\u{118AC}' ],
    );

    let mut output = String::new();
    let mut input_chars = input.chars().peekable();
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

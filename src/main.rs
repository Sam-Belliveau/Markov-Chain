use std::io::{BufRead, BufReader};
use std::fs::File;
mod char_set;
mod char_prob;

const LETTERS: usize = 5;

fn main() {
    // Read Dictionary
    let mut f = BufReader::new(File::open("words.txt").expect("open failed"));

    let mut dict = String::new();

    for line_raw in f.lines() {
        dict += line_raw.expect("Error Reading Line Lol!").to_lowercase().trim();
        dict += " ";
    }

    println!("Finished Reading!");

    // Get Probabilities
    let array_size: usize = 1 << (LETTERS * char_set::length_bits());
    
    let mut probs = vec![char_prob::CharProb::new(); array_size];

    let mut buffer = String::from(" ");

    for l in dict.chars() {
        let id = char_set::get_str_id(&buffer);
        probs.get_mut(id).expect("id's are fucked a").add(l);

        while buffer.len() >= LETTERS {
            buffer.remove(0);
        }

        buffer += &l.to_string();
    }

    println!("Finished Probs!");

    // Generate text
    buffer = String::from(" ");

    for _i in 0..1500 {
        let id = char_set::get_str_id(&buffer);
        let next = probs.get(id).expect("id's are fucked b").get_char();

        print!("{}", next);

        while buffer.len() >= LETTERS {
            buffer.remove(0);
        }

        buffer += &next.to_string();
    }
}

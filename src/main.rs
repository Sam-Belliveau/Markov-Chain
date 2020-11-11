#![allow(dead_code)]

use std::io::{BufRead, BufReader};
use std::fs::File;
use clap::{Arg, App};
mod char_set;
mod char_prob;

// Functions used to check if command line arguments are valid
fn valid_chain_size(v: String) -> Result<(), String> {
    match v.parse::<usize>() {
        Ok(n) => {
            if n <= 0 { Err(String::from("Must be >= 1!")) }
            else if n >= 6 { Err(String::from("Values >= 6 require more than 130GB of ram!"))}
            else { Ok(()) }
        },
        Err(_) => Err(String::from("Is not a recognizable positive integer!"))
    }
}

fn valid_output_length(v: String) -> Result<(), String> {
    match v.parse::<usize>() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Is not a recognizable positive integer!"))
    }
}


// Main Function
fn main() {
    // Taking in Arguments
    let matches = App::new("Sam's Markov Chain Generator")
        .version("0.1.0")
        .author("Sam Belliveau <sam.belliveau@gmail.com>")
        .about("Generate text based on the character probabilities from a given dictionary.")
        .arg(Arg::with_name("dictionary")
                 .short("d")
                 .long("dictionary")
                 .takes_value(true)
                 .required(true)
                 .help("Dictionary from which character probabilities are built from"))
        .arg(Arg::with_name("chain size")
                 .short("c")
                 .long("chain-size")
                 .takes_value(true)
                 .default_value("2")
                 .validator(valid_chain_size)
                 .help("How many past characters to consider when building probabilites"))
        .arg(Arg::with_name("output length")
                 .short("o")
                 .long("output-length")
                 .takes_value(true)
                 .default_value("1000")
                 .validator(valid_output_length)
                 .help("The length of the output Markov Chain"))
        .get_matches();


    // Read Arguments from Command Line
    let dict_name     = matches.value_of("dictionary").unwrap();
    let chain_size    = matches.value_of("chain size").unwrap().parse::<usize>().unwrap();
    let output_length = matches.value_of("output length").unwrap().parse::<usize>().unwrap();

    // Read Dictionary
    let dictionary = BufReader::new(File::open(dict_name).expect("Error Reading Supplied File!"));

    // Read through the dictionary and generate probabilities for each character
    let array_size: usize = 1 << (chain_size * char_set::length_bits());
    let mut probabilities = vec![char_prob::CharProb::new(); array_size];

    // Buffer of the last n characters to build probabilities of
    let mut buffer = String::from(" ");

    for (i, line) in dictionary.lines().enumerate() {
        match line {
            Ok(line) => {
                for l in line.chars() {
                    let id = char_set::get_str_id(&buffer);
                    probabilities.get_mut(id).expect("Internal issue with character ID's, contact the developer!").add(l);
            
                    while buffer.len() >= chain_size {
                        buffer.remove(0);
                    }
            
                    buffer += &l.to_string();
                }
            },

            Err(e) => {
                eprintln!("Error Reading Line {}! ({})", i, e);
            }
        }
    }

    // Generate text from previously generated probabilites
    buffer = String::from(" ");

    for _i in 0..output_length {
        let id = char_set::get_str_id(&buffer);
        let next = probabilities.get(id).expect("Internal issue with character ID's, contact the developer!").get_char();

        print!("{}", next);

        while buffer.len() >= chain_size {
            buffer.remove(0);
        }

        buffer += &next.to_string();
    }

    println!("");
}

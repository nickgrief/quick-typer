use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};

use console::Term;
use rand::Rng;

fn main() {
    let path = "./assets/dict.txt";
    let mut words = vec![];
    read_lines(path, &mut words).unwrap();

    let mut dict: HashMap<String, Vec<String>> = HashMap::new();
    sort_words(&words, &mut dict);

    let mut term = Term::stdout();
    loop {
        let char = term.read_char().unwrap();
        let vector_opt = dict.get(&char.to_string());
        let vector;
        let word;
        match vector_opt {
            Some(vec) => {
                vector = vec;
                word = vector[rand::thread_rng().gen_range(0..vector.len())].clone();
            }
            None => word = char.to_string().clone(),
        };
        term.write_fmt(format_args!("{} ", &word)).unwrap();
    }
}

fn read_lines(path: &str, output: &mut Vec<String>) -> Result<(), Error> {
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        output.push(line?);
    }

    Ok(())
}

fn sort_words(words: &Vec<String>, dict: &mut HashMap<String, Vec<String>>) {
    for word in words {
        match word.chars().next() {
            Some(char) => {
                let first_letter = char.to_string();
                if let std::collections::hash_map::Entry::Vacant(e) =
                    dict.entry(first_letter.clone())
                {
                    let vector = vec![word.to_string()];
                    e.insert(vector);
                } else {
                    let value = dict.get_mut(&first_letter);
                    match value {
                        Some(vector) => {
                            vector.push(word.to_string());
                        }
                        None => (),
                    }
                }
            }
            None => (),
        }
    }
}

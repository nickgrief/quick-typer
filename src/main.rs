use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() {
    let path = "./assets/dict.txt";
    let mut words = vec![];
    read_lines(path, &mut words).unwrap();
    println!("{:?}", words);
}

fn read_lines(path: &str, output: &mut Vec<String>) -> Result<(), Error> {
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        output.push(line?);
    }

    Ok(())
}

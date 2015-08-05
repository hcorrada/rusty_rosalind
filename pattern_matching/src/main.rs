extern crate itertools;
extern crate rosalind_lib;

use itertools::Itertools;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use rosalind_lib::matching::naive;

pub fn read_input(filename: &str) -> (String, String) {
    // open file and get lines iterator
    let fhandle = File::open(filename)
                    .ok()
                    .expect("Could not open file");
    let reader  = BufReader::new(fhandle);
    let mut lines = reader.lines();

    // read the pattern line
    let pattern: String = match lines.next() {
        Some(v) => v.ok()
                    .expect("Could not read pattern input line")
                    .clone(),
        None => panic!("No input pattern line found"),
    };

    // read the genome line
    let genome: String = match lines.next() {
        Some(v) => v.ok()
                    .expect("Could not read genome input line")
                    .clone(),
        None => panic!("No input genome line found"),
    };

    // return input
    (pattern, genome)
}

fn main() {
    let filename = env::args().nth(1)
        .expect("Need filename as argument");
    let (pattern, genome) = read_input(&filename);
    let res = naive(&pattern, &genome);
    let out = res.iter().join(" ");
    println!("{}", out);
}

#[cfg(test)]
mod test {
    #[test]
    fn read_input() {
        let (pattern, genome) = super::read_input("test.txt");
        assert_eq!(pattern, "ATAT".to_string());
        assert_eq!(genome, "GATATATGCATATACTT".to_string());
    }
}

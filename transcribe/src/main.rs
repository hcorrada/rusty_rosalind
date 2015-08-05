extern crate rosalind_lib;

use std::env;

use std::fs::File;
use std::io::Read;
use rosalind_lib::dna_utils::transcribe;

/// Read problem input from file
///
pub fn read_input(filename: &str) -> String {
    let mut fhandle = File::open(filename)
        .ok()
        .expect("Could not open file");

    let mut dna = String::new();
    fhandle.read_to_string(&mut dna)
        .ok()
        .expect("Unable to read");

    dna.trim().to_string()
}

fn main() {
    let filename = env::args().nth(1)
        .expect("Need one argument");

    let string = read_input(&filename);
    let res = transcribe(&string);
    println!("{}", res);
}

#[cfg(test)]
mod test {
    #[test]
    fn read_input() {
        let filename = "test.txt";
        let dna = "GATGGAACTTGACTACGTAAATT";
        let res = super::read_input(filename);
        assert_eq!(res, dna);
    }
}

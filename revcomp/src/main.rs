extern crate rosalind_lib;

use std::env;
use rosalind_lib::dna_utils::revcomp;
use std::fs::File;
use std::io::Read;

/// read string from file
///
pub fn read_input(filename: &str) -> String {
    let mut fhandle = File::open(filename)
        .ok()
        .expect("Could not open file");

    let mut string = String::new();
    fhandle.read_to_string(&mut string)
        .ok()
        .expect("Could not read file");
    string.trim().to_string()
}

fn main() {
    let filename = env::args().nth(1)
        .expect("Need one input");
    let res = revcomp(&filename);
    println!("{}", res);
}

#[cfg(test)]
mod test {
    #[test]
    fn read_input() {
        let filename = "test.txt";
        let res = super::read_input(&filename);
        assert_eq!(res, "AAAACCCGGT");
    }
}

extern crate rosalind_lib;
extern crate itertools;

use std::env;
use std::fs::File;
use std::io::Read;
use itertools::Itertools;
use rosalind_lib::dna_utils::count_nucleotides;

/// Read input file
///
/// # Examples
///
/// ```
/// use count_nucleotides::read_input;
///
/// let filename = "test.txt";
/// let string = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";
/// let res = read_input(filename);
///
/// assert_eq!(res, string);
/// ```
fn read_input(filename: &str) -> String {
    // open file
    let mut fhandle = File::open(filename)
        .ok()
        .expect("Unable to open file");

    // allocate string to read input to
    let mut string = String::new();
    fhandle.read_to_string(&mut string)
        .ok()
        .expect("Unable to read");

    // remove whitespace ('trim' returns a slice so need)
    // convert back to a String
    string.trim().to_string()
}



fn main() {
    let filename = env::args().nth(1)
        .expect("Need one argument");
    let string = read_input(&filename);
    let res = count_nucleotides(&string);
    let res = res.iter().join(" ");
    println!("{}", &res);
}

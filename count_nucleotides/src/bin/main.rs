extern crate rosalind_lib;

use std::env;
use std::fs::File;
use std::io::Read;
use itertools::Itertools;

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
pub fn read_input(filename: &str) -> String {
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
    let string = count_nucleotides::read_input(&filename);
    let res = count_nucleotides::count_nucleotides(&string);
    let res = count_nucleotides::make_output(&res);
    println!("{}", &res);
}

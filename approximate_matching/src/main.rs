extern crate itertools;
extern crate rosalind_lib;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use itertools::Itertools;
use rosalind_lib::kmers::find_matches;

/// read input
///
/// # Examples
///
/// ```
/// use approximate_matching::read_input;
///
/// let res = read_input("test.txt");
/// assert_eq!(res.0, "ATTCTGGA".to_string());
/// assert_eq!(res.1, "CGCCCGAATCCAGAACGCATTCCCATATTTCGGGACCACTGGCCTCCACGGTACGGACGTCAATCAAATGCCTAGCGGCTTGTGGTTTCTCCTACGCTCC".to_string());
/// assert_eq!(res.2, 3);
/// ```
pub fn read_input(filename: &str) -> (String, String, usize) {
    let fhandle = File::open(filename)
        .ok()
        .expect("Couldn't open file");

    let mut lines = BufReader::new(fhandle).lines();

    let pattern = if let Some(Ok(x)) = lines.next() {
        x
    } else {
        panic!("Could not read pattern");
    };

    let text = if let Some(Ok(x)) = lines.next() {
        x
    } else {
        panic!("Could not read text");
    };

    let dstr = if let Some(Ok(x)) = lines.next() {
        x
    } else {
        panic!("Could not read d");
    };

    let d = if let Ok(x) = dstr.parse() { x
    } else {
        panic!("Could not parse d");
    };

    (pattern, text, d)
}

fn main() {
    let filename = env::args().nth(1)
        .expect("Need input filename as argument");

    let (pattern, text, d) = read_input(&filename);
    let res = find_matches(&pattern, &text, d);
    let out = res.iter().join(" ");
    println!("{}", out);
}

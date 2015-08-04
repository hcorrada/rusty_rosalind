extern crate itertools;

use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use itertools::Itertools;

/// Make output string
///
/// # Examples
///
/// ```
/// use count_nucleotides::make_output;
///
/// let counts = vec![20, 12, 17, 21];
/// let res = make_output(&counts);
/// assert_eq!(res, "20 12 17 21");
/// ```
pub fn make_output(counts: &Vec<i32>) -> String {
    // join the vector into a string separated
    // by spaces
    counts.iter().join(" ")
}

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

/// count nucleotides in a string
///
/// # Examples
///
/// ```
/// use count_nucleotides::count_nucleotides;
///
/// let string = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";
/// let res = count_nucleotides(string);
/// assert_eq!(res, vec![20, 12, 17, 21])
/// ```
pub fn count_nucleotides(string: &str) -> Vec<i32> {
    // use a BTreeMap to order the keys of the hash so that
    // keys are ordered (ACGT)
    let mut map = BTreeMap::new();

    // add 1 to counter hash entry for each character
    // in string
    for c in string.chars() {
        *map.entry(c).or_insert(0) += 1;
    }

    // return vector of nucleotide counts
    map.values().cloned().collect()
}

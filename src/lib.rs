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
    let res = counts.iter().join(" ");
    return res;
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
    let mut fhandle = File::open(filename)
        .ok()
        .expect("Unable to open file");

    let mut string = String::new();
    fhandle.read_to_string(&mut string)
        .ok()
        .expect("Unable to read");
    let string = string.trim().to_string();
    return string;
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
    let mut map = BTreeMap::new();

    for c in string.chars() {
        *map.entry(c).or_insert(0) += 1;
    }

    let values: Vec<i32> = map.values().cloned().collect();
    return values;
}

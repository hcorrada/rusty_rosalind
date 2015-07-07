extern crate itertools;

use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::Read;
use itertools::Itertools;

#[test]
fn small_test() {
    let string = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";
    let res = count_nucleotides(string);
    assert_eq!(res, vec![20, 12, 17, 21])
}

#[test]
fn read_file() {
    let filename = "test.txt";
    let string = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";
    let res = read_input(filename);

    assert_eq!(res, string);
}

#[test]
fn test_printout() {
    let counts = vec![20, 12, 17, 21];
    let res = make_output(counts);
    assert_eq!(res, "20 12 17 21");
}

fn make_output(counts: Vec<i32>) -> String {
    let res = counts.iter().join(" ");
    return res;
}

fn read_input(filename: &str) -> String {
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

fn count_nucleotides(string: &str) -> Vec<i32> {
    let mut map = BTreeMap::new();

    for c in string.chars() {
        *map.entry(c).or_insert(0) += 1;
    }

    let values: Vec<i32> = map.values().cloned().collect();
    return values;
}

fn main() {
    let filename = env::args().nth(1)
        .expect("Need one argument");
    let string = read_input(&filename);
    let res = count_nucleotides(&string);
    let res = make_output(res);
    println!("{}", &res);
}

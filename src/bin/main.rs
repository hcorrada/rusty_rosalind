extern crate itertools;
extern crate frequent_words_mismatch;

use std::env;

use itertools::Itertools;
use frequent_words_mismatch::read_input;
use frequent_words_mismatch::find_matches;

fn main() {
    let filename = env::args().nth(1)
        .expect("Need input filename as argument");

    let (text, k, d) = read_input(&filename);
    let res = find_matches(&text, &text, d);
    let out = res.iter().join(" ");
    println!("{}", out);
}

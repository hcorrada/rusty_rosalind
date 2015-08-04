extern crate itertools;
extern crate approximate_matching;

use std::env;

use itertools::Itertools;
use approximate_matching::read_input;
use approximate_matching::find_matches;

fn main() {
    let filename = env::args().nth(1)
        .expect("Need input filename as argument");

    let (pattern, text, d) = read_input(&filename);
    let res = find_matches(&pattern, &text, d);
    let out = res.iter().join(" ");
    println!("{}", out);
}

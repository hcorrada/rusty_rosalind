extern crate itertools;
extern crate pattern_matching;

use itertools::Itertools;
use std::env;
use pattern_matching::read_input;
use pattern_matching::match_pattern;

fn main() {
    let filename = env::args().nth(1)
        .expect("Need filename as argument");
    let (pattern, genome) = read_input(&filename);
    let res = match_pattern(&pattern, &genome);
    let out = res.iter().join(" ");
    println!("{}", out);
}

extern crate frequent_words;
extern crate itertools;

use frequent_words::*;
use std::env;
use itertools::Itertools;

fn main() {
    let filename = env::args().nth(1)
        .expect("Need an argument");

    let (dna, k) = read_input(&filename);
    let kmer_counts = count_kmers(&dna, k);
    let frequent_kmers: Vec<String> = find_frequent_kmers(kmer_counts);
    let res = frequent_kmers.iter().join(" ");
    println!("{}", res);
}

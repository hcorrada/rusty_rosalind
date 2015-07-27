extern crate itertools;
extern crate frequent_words_mismatch;

use std::env;

use itertools::Itertools;
use frequent_words_mismatch::read_input;
use frequent_words_mismatch::count_mismatch_kmers;
use frequent_words_mismatch::find_frequent_kmers;

fn main() {
    let filename = env::args().nth(1)
        .expect("Need input filename as argument");

    let (text, k, d) = read_input(&filename);
    let kmer_counts = count_mismatch_kmers(&text, k, d);
    let frequent_kmers: Vec<String> = find_frequent_kmers(&kmer_counts);
    let res = frequent_kmers.iter().join(" ");
    println!("{}", res);
}

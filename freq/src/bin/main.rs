extern crate itertools;
extern crate frequent_words_mismatch_revcomp;
extern crate rosalind_lib;

use std::env;

use itertools::Itertools;
use frequent_words_mismatch_revcomp::read_input;
use rosalind_lib::count_mismatch_kmers;
use rosalind_lib::find_frequent_kmers;
use rosalind_lib::revcomp;

fn main() {
    let filename = env::args().nth(1)
        .expect("Need input filename as argument");

    let (text, k, d) = read_input(&filename);
    let mut kmer_counts = count_mismatch_kmers(&text, k, d);
    let revcomp_text = revcomp(&text);
    let revcomp_kmer_counts = count_mismatch_kmers(&revcomp_text,k, d);

    for (kmer, count) in revcomp_kmer_counts {
        let counter = kmer_counts.entry(kmer).or_insert(0 as i32);
        *counter += count;
    }

    let frequent_kmers: Vec<String> = find_frequent_kmers(&kmer_counts);
    let res = frequent_kmers.iter().join(" ");
    println!("{}", res);
}

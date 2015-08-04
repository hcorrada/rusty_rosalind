extern crate clump_finding;
extern crate itertools;

use std::env;
use clump_finding::*;
use itertools::Itertools;

fn main() {
    let filename = env::args().nth(1)
        .expect("Need an argument");

    let (genome, k, l, t) = read_input(&filename);
    let kmer_locations = locate_kmers(&genome, k);
    let clumps = find_clumps(kmer_locations, l, t, k);
    let res = clumps.iter().join(" ");
    println!("{}", res);
}

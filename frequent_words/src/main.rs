extern crate rosalind_lib;
extern crate itertools;

use rosalind_lib::kmers::count_kmers;

use std::env;
use itertools::Itertools;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;


pub fn read_input(filename: &str) -> (String, usize) {
    let fhandle = File::open(filename)
        .ok()
        .expect("Could not open file");
    let reader = BufReader::new(fhandle);
    let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    let dna: String = lines[0].clone();
    let k: usize = lines[1].parse().unwrap();
    (dna, k)
}

fn main() {
    let filename = env::args().nth(1)
        .expect("Need an argument");

    let (dna, k) = read_input(&filename);
    let kmer_counts = count_kmers(&dna, k);
    let frequent_kmers: Vec<String> = kmer_counts.find_frequent_kmers();
    let res = frequent_kmers.iter().join(" ");
    println!("{}", res);
}

#[cfg(test)]
mod test {
    #[test]
    fn read_input() {
        let (dna, k) = super::read_input("test.txt");
        assert_eq!(dna, "ACGTTGCATGTCGCATGATGCATGAGAGCT");
        assert_eq!(k, 4);
    }
}

extern crate itertools;
extern crate rosalind_lib;

use std::env;

use itertools::Itertools;
use rosalind_lib::kmers::count_mismatch_kmers;
use rosalind_lib::dna_utils::revcomp;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;


pub fn read_input(filename: &str) -> (String, usize, usize) {
    let fhandle = File::open(filename)
        .ok()
        .expect("Couldn't open file");

    let mut lines = BufReader::new(fhandle).lines();

    let text = if let Some(Ok(x)) = lines.next() { x } else { panic!("Could not read text"); };
    let kdstr = if let Some(Ok(x)) = lines.next() { x } else { panic!("Could not read d"); };
    let mut iter = kdstr.split_whitespace();
    let kstr = if let Some(x) = iter.next() { x } else { panic!("Could not read k"); };
    let k = if let Ok(x) = kstr.parse() { x } else { panic!("Couldn't parse k"); };
    let dstr = if let Some(x) = iter.next() { x } else { panic!("Could not read d"); };
    let d = if let Ok(x) = dstr.parse() { x } else { panic!("Could not parse d"); };
    (text, k, d)
}

fn main() {
    let filename = env::args().nth(1)
        .expect("Need input filename as argument");

    let (text, k, d) = read_input(&filename);
    let mut kmer_counts = count_mismatch_kmers(&text, k, d);
    let revcomp_text = revcomp(&text);
    let revcomp_kmer_counts = count_mismatch_kmers(&revcomp_text, k, d);

    for (kmer, count) in revcomp_kmer_counts.to_hashmap() {
        kmer_counts.insert_u8(kmer, *count);
    }

    let frequent_kmers: Vec<String> = kmer_counts.find_frequent_kmers();
    let res = frequent_kmers.iter().join(" ");
    println!("{}", res);
}

#[cfg(test)]
mod test {
    #[test]
    fn read_input() {
        let res = super::read_input("test.txt");
        assert_eq!(res.0, "ACGTTGCATGTCGCATGATGCATGAGAGCT".to_string());
        assert_eq!(res.1, 4);
        assert_eq!(res.2, 1);
    }
}

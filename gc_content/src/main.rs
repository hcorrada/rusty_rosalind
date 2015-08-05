extern crate rosalind_lib;

use std::env;
use rosalind_lib::io::parse_fast_file;
use rosalind_lib::dna_utils::gc_content;

pub fn main() {
    let filename = env::args().nth(1)
        .expect("Need one argument");

    let records = parse_fasta_file(&filename);
    let mut maxgc = -1.0 as f32;
    let mut maxid = "";

    for (id, dna) in &records {
        let curgc = gc_content(&dna);
        if curgc > maxgc {
            maxgc = curgc;
            maxid = id;
        }
    }
    println!("{}", maxid);
    println!("{}", maxgc);
}

extern crate rosalind_lib;
extern crate itertools;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;
use rosalind_lib::kmers::kmer_composition;
use itertools::Itertools;

fn read_input(filename: &str) -> (usize, String) {
    let fhandle = File::open(filename)
            .ok()
            .expect("Could not open input file");

    let mut lines = BufReader::new(fhandle).lines();

    let kstr = if let Some(Ok(x)) = lines.next() { x } else { panic!("Could not read k input line") };
    let k = if let Ok(x) = kstr.parse() { x } else { panic!("Could not parse k") };
    let text = if let Some(Ok(x)) = lines.next() { x } else { panic!("Could not read text input line")};
    (k, text)
}

fn main() {
    let filename = env::args().nth(1)
        .expect("Need input filename as argument");
    let (k, text) = read_input(&filename);
    let res = kmer_composition(&text, k).into_iter()
                .map(|x| String::from_utf8(x).unwrap())
                .join("\n");
    println!("{}", res);
}

#[cfg(test)]
mod test {
    #[test]
    fn read_input() {
        let res = super::read_input("test.txt");
        assert_eq!(res.0, 5);
        assert_eq!(res.1, "CAATCCAAC");
    }
}

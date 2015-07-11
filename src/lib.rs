#[macro_use] extern crate scan_fmt;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

/// locate kmers
///
/// # Examples
///
/// ```
/// use clump_finding::locate_kmers;
///
/// let genome = "CGACACGACATTGCGACATA";
/// let res = locate_kmers(genome, 5);
/// assert!(res.contains_key("CGACA"));
/// let locations = res.get("CGACA").unwrap();
/// assert_eq!(*locations, vec![0, 5, 13]);
pub fn locate_kmers(genome: &str, k: usize) -> HashMap<String, Vec<usize>> {
    let mut kmer_locations = HashMap::new();
    let n = genome.len();

    for location in 0..n-k+1 {
        let kmer = &genome[location..location+k];
        let locations = kmer_locations.entry(kmer.to_string()).or_insert(Vec::new());
        locations.push(location);
    }
    kmer_locations
}

/// read input
///
/// # Examples
///
/// ```
/// use clump_finding::read_input;
///
/// let (genome, k, l, t) = read_input("test.txt");
/// assert_eq!(genome, "CGGACTCGACAGATGTGAAGAAATGTGAAGACTGAGTGAAGAGAAGAGGAAACACGACACGACATTGCGACATAATGTACGAATGTAATGTGCCTATGGC".to_string());
/// assert_eq!(k, 5);
/// assert_eq!(l, 75);
/// assert_eq!(t, 4);
pub fn read_input(filename: &str) -> (String, usize, usize, usize){
    let fhandle = File::open(filename)
        .ok()
        .expect("Could not read file");

    let reader = BufReader::new(fhandle);
    let mut lines = reader.lines();

    // read the genome line
    let genome = match lines.next() {
        Some(v) => v.ok().expect("Could not read genome line").clone(),
        None => panic!("No input found"),
    };

    // read the parameter line
    let parms = match lines.next() {
        Some(v) => v.ok().expect("Could not read parameter line"),
        None => panic!("No parameter input line found"),
    };

    // parse the parameter line
    let (k, l, t) = scan_fmt!(&parms, "{} {} {}", usize, usize, usize);
    (genome, k.unwrap(), l.unwrap(), t.unwrap())
}

#[macro_use] extern crate scan_fmt;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

/// find clumps
///
/// # Examples
///
/// ```
/// use clump_finding::find_clumps;
/// use std::collections::HashMap;
///
/// let mut kmer_locations = HashMap::new();
/// kmer_locations.insert("A".to_string(), vec![1,3,5,7]);
/// kmer_locations.insert("B".to_string(), vec![1,2,3,4]);
/// kmer_locations.insert("C".to_string(), vec![1,10,20,30]);
/// let res = find_clumps(kmer_locations, 3, 3, 1);
/// assert_eq!(res, vec!["B".to_string()]);
/// ```
///
/// ```
/// use clump_finding::find_clumps;
/// use clump_finding::locate_kmers;
///
/// let genome = "CGGACTCGACAGATGTGAAGAAATGTGAAGACTGAGTGAAGAGAAGAGGAAACACGACACGACATTGCGACATAATGTACGAATGTAATGTGCCTATGGC";
/// let (k, l, t) = (5, 75, 4);
/// let kmer_locations = locate_kmers(genome, k);
/// let mut clumps = find_clumps(kmer_locations, l, t, k);
/// let mut expected = vec!["CGACA", "GAAGA", "AATGT"];
/// assert_eq!(clumps.sort(), expected.sort());
/// ```
pub fn find_clumps(kmer_locations: HashMap<String, Vec<usize>>,
        l: usize, t: usize, k:usize) -> Vec<String> {
    let mut clumps = Vec::new();

    for (kmer, locations) in &kmer_locations {
        let n = locations.len();
        if n < t { continue; };
        for i in 0..n-t+1 {
            if locations[i] + l - k >= locations[i+t-1] {
                clumps.push(kmer.to_string());
                break;
            }
        }
    }
    clumps
}

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
/// ```
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

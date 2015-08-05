extern crate itertools;
use self::itertools::Itertools;

use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone)]
struct Product {
    n: usize,
    d: usize,
    vals: Vec<usize>,
}

impl Product {
    fn new(n: usize, d: usize) -> Product {
        Product {
            n: n,
            d: d,
            vals: Vec::new(),
        }
    }
}

impl Iterator for Product {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.vals.len() == 0 {
            for _ in 0..self.d {
                self.vals.push(0);
            }
            return Some(self.vals.clone());
        }

        let mut cur_position = self.d - 1;
        loop {
            if self.vals[cur_position] < self.n - 1 {
                self.vals[cur_position] += 1;
                break;
            }
            if cur_position == 0 { return None };
            cur_position -= 1;
        }
        cur_position += 1;
        while cur_position < self.d {
            self.vals[cur_position] = 0;
            cur_position += 1;
        }
        Some(self.vals.clone())
    }
}

struct Combinations {
    n:    usize,
    d:    usize,
    vals: Vec<usize>,
}

impl Combinations {
    fn new(n: usize, d: usize) -> Combinations {
        if d >= n {
            panic!("Can't create these combinations");
        }

        Combinations {
            n: n,
            d: d,
            vals: Vec::new(),
        }
    }
}

impl Iterator for Combinations {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        // if value vector is empty, initialize
        if self.vals.len() == 0 {
            for i in 0..self.d {
                self.vals.push(i);
            }
            return Some(self.vals.clone());
        }

        // check if we can advance index from last to first position
        let mut cur_position = self.d - 1;
        loop {
            if self.vals[cur_position] + (self.d - cur_position) < self. n {
                // advance index in this column and break loop
                self.vals[cur_position] += 1;
                break;
            }

            // if this is the first position, there are no more combinations
            if cur_position == 0 { return None };
            cur_position -= 1;
        }

        // we advanced some position, go from this position to the last to
        // update the combination accordingly
        cur_position += 1;
        while cur_position < self.d {
            self.vals[cur_position] = self.vals[cur_position - 1] + 1;
            cur_position += 1;
        }
        // return resulting vector
        Some(self.vals.clone())
    }
}

struct KmerNeighborhood<'a> {
    chars: &'a [u8],
    n: usize,
    buffer: Vec<u8>,
    nucs: &'a [u8; 4],
    index_iterator: itertools::Product<Combinations, Product>,
}

impl<'a> KmerNeighborhood<'a> {
    fn new(kmer: &str, d: usize) -> KmerNeighborhood {
        let chars = kmer.as_bytes();
        let n = chars.len();
        let mut buffer = Vec::with_capacity(n);
        for i in 0..n { buffer.push(chars[i]); }
        let combinations = combinations(n, d);
        let product = product(4, d);
        let index_iterator = combinations.cartesian_product(product);

        KmerNeighborhood {
            chars: chars,
            n: n,
            buffer: buffer,
            nucs: b"ACGT",
            index_iterator: index_iterator,
        }
    }

    fn build_string(&mut self, ivec: &Vec<usize>, jvec: &Vec<usize>) -> Vec<u8> {
        for i in 0..self.n { self.buffer[i] = self.chars[i] };

        for (i,j) in ivec.iter().zip(jvec) {
            self.buffer[*i] = self.nucs[*j];
        }
        self.buffer.clone()
    }
}

impl<'a> Iterator for KmerNeighborhood<'a> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let indices: Option<(Vec<usize>, Vec<usize>)> = self.index_iterator.next();
        match indices {
            None => None,
            Some((ivec, jvec)) => Some(self.build_string(&ivec, &jvec)),
        }
    }
}

/// generate combinations
///
fn combinations(n: usize, d: usize) -> Combinations {
    Combinations::new(n,d)
}

/// generate cartesian Product
fn product(n: usize, d: usize) -> Product {
    Product::new(n, d)
}

/// generate string neighborhood
///
fn neighborhood(kmer: &str, d: usize) -> HashSet<Vec<u8>> {
    let kmer_neighborhood = KmerNeighborhood::new(kmer, d);
    let mut res = HashSet::new();
    for kmer in kmer_neighborhood {
        res.insert(kmer);
    }
    res
}

use std::thread;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct KmerCounter {
    map: HashMap<Vec<u8>, i32>,
}

impl KmerCounter {
    fn new() -> Self {
        KmerCounter { map: HashMap::new() }
    }

    fn count(&mut self, kmer: &str) {
        let kmer = kmer.bytes().collect();
        self.count_u8(&kmer);
    }

    fn count_u8(&mut self, kmer: &Vec<u8>) {
        let counter = self.map.entry(kmer.clone()).or_insert(0i32);
        *counter += 1;
    }

    fn insert(&mut self, kmer: &str, val: i32) {
        self.map.insert(kmer.bytes().collect(), val);
    }

    fn to_hashmap(&self) -> &HashMap<Vec<u8>, i32> {
        &self.map
    }

    /// get frequent kmers
    ///
    pub fn find_frequent_kmers(&self) ->  Vec<String> {
        let mut res: Vec<String> = Vec::new();
        let mut highest_count: i32 = 0;

        for (kmer, count) in &self.map {
            if *count > highest_count {
                highest_count = count.clone();
                res.clear();
                let s = String::from_utf8(kmer.clone()).unwrap();
                res.push(s);
            } else if *count == highest_count {
                let s = String::from_utf8(kmer.clone()).unwrap();
                res.push(s);
            }
        }
        res
    }
}

/// count kmers
///
pub fn count_kmers<'a>(dna: &'a str, k: usize) -> KmerCounter {
    let kmer_counts: Arc<Mutex<KmerCounter>> = Arc::new(Mutex::new(KmerCounter::new()));
    let dna = dna.to_string();

    let n = dna.len();
    let handles: Vec<_> = (0..n-k+1).map(|start| {
        let dna = dna.clone();
        let kmer_counts = kmer_counts.clone();

        thread::spawn(move || {
            let kmer = &dna[start..start+k];
            let mut kmer_counts = kmer_counts.lock().unwrap();
            kmer_counts.count(kmer);
        })
    }).collect();

    for h in handles { h.join().unwrap(); }
    let ref kmer_counts = *kmer_counts.lock().unwrap();
    kmer_counts.clone()
}

/// count kmers with mismatches
///
pub fn count_mismatch_kmers(text: &str, k: usize, d:usize) -> KmerCounter {
    let mut kmer_counts = KmerCounter::new();

    let n = text.len();
    for start in 0..n-k+1 {
        let kmer = &text[start..start+k];
        let kmer_neighborhood = neighborhood(kmer, d);
        for approximate_kmer in kmer_neighborhood.iter() {
            kmer_counts.count_u8(approximate_kmer);
        }
    }
    kmer_counts
}

/// kmer locator
pub struct KmerLocator {
    map: HashMap<Vec<u8>, Vec<usize>>,
}

impl KmerLocator {
    pub fn new() -> Self {
        KmerLocator { map: HashMap::new() }
    }

    pub fn insert(&mut self, kmer: &str, location: usize) {
        let kmer = kmer.bytes().collect();
        let locations = self.map.entry(kmer).or_insert(Vec::new());
        locations.push(location)
    }

    pub fn insert_locations(&mut self, kmer: &str, locations: Vec<usize>) {
        let kmer = kmer.bytes().collect();
        let kmer_locations = self.map.entry(kmer).or_insert(Vec::new());

        for location in &locations {
            kmer_locations.push(*location);
        }
    }

    pub fn to_hashmap(&self) -> &HashMap<Vec<u8>, Vec<usize>> {
        &self.map
    }

    pub fn find_clumps(&self, l: usize, t: usize, k:usize) -> Vec<String> {
        let mut clumps = Vec::new();

        for (kmer, locations) in &self.map {
            let n = locations.len();
            if n < t { continue; };
            for i in 0..n-t+1 {
                if locations[i] + l - k >= locations[i+t-1] {
                    let s = String::from_utf8(kmer.clone()).unwrap();
                    clumps.push(s);
                    break;
                }
            }
        }
        clumps
    }
}

/// locate kmer locator
///
pub fn locate_kmers(genome: &str, k: usize) -> KmerLocator {
    let mut kmer_locations = KmerLocator::new();
    let n = genome.len();

    for location in 0..n-k+1 {
        let kmer = &genome[location..location+k];
        kmer_locations.insert(kmer, location)
    }
    kmer_locations
}

/// find approximate matches
///
pub fn find_matches(pattern: &str, text: &str, d: usize) -> Vec<usize> {
    let neighborhood = neighborhood(pattern, d);
    let text = text.as_bytes();
    let k = pattern.len();
    let n = text.len();
    let mut result = Vec::new();

    for i in 0..n-k+1 {
        let kmer = &text[i..i+k];
        if neighborhood.contains(kmer) {
            result.push(i);
        }
    }
    result
}

/// kmer composition
///
pub fn kmer_composition(text: &str, k: usize) -> Vec<Vec<u8>> {
    let n = text.len();
    let text = text.as_bytes();
    let mut kmers = Vec::new();

    for i in 0..n-k+1 {
        let kmer = text[i..i+k].to_vec();
        kmers.push(kmer);
    }
    kmers.sort();
    kmers
}

#[cfg(test)]
mod test {
    use dna_utils::num_mismatches;

    #[test]
    fn count_kmers() {
        let dna = "ACAACTATGCATACTATCGGGAACTATCCT";
        let kmer_counts = super::count_kmers(&dna, 5);
        let kmer_hash = kmer_counts.to_hashmap().clone();
        let count = kmer_hash[&b"ACTAT"[..]];
        assert_eq!(count, 3);
    }

    #[test]
    fn combinations() {
        let combs: Vec<_> = super::combinations(3, 2).collect();
        assert_eq!(combs, vec![[0,1], [0,2], [1,2]]);
    }

    #[test]
    fn product() {
        let prod: Vec<_> = super::product(3,2).collect();
        assert_eq!(prod, vec![[0,0],[0,1],[0,2],[1,0],[1,1],[1,2],[2,0],[2,1],[2,2]]);
    }

    #[test]
    fn neighborhood() {
        let res = super::neighborhood("ATCGA", 2);
        assert!(res.iter().all(|x| num_mismatches(b"ATCGA", x) <= 2));
        assert!(res.contains(&b"ATCGA"[..]));
        assert!(res.contains(&b"TTCGA"[..]));
        assert!(res.contains(&b"ATTGA"[..]));
        assert!(!res.contains(&b"AAAAA"[..]));
        assert!(!res.contains(&b"ATCG"[..]));
        assert!(super::neighborhood("GTTG", 1).contains(&b"GATG"[..]));
    }

    #[test]
    fn count_mismatch_kmers() {
        let text = "ACGTTGCATGTCGCATGATGCATGAGAGCT";
        let counts = super::count_mismatch_kmers(text, 4, 1).to_hashmap().clone();
        assert_eq!(counts[&b"GATG"[..]], 5);
    }

    #[test]
    fn find_frequent_kmers() {
        let mut kmer_counts = super::KmerCounter::new();
        kmer_counts.insert("AB", 4);
        kmer_counts.insert("AC", 2);
        kmer_counts.insert("AD", 4);
        kmer_counts.insert("AE", 1);
        let mut frequent_kmers = kmer_counts.find_frequent_kmers();
        frequent_kmers.sort();
        assert_eq!(frequent_kmers, ["AB", "AD"]);
    }

    #[test]
    fn approximate_matches() {
        let pattern = "ATTCTGGA";
        let text = "CGCCCGAATCCAGAACGCATTCCCATATTTCGGGACCACTGGCCTCCACGGTACGGACGTCAATCAAATGCCTAGCGGCTTGTGGTTTCTCCTACGCTCC";
        let result = super::find_matches(pattern, text, 3);
        assert_eq!(result, vec![6, 7, 26, 27, 78]);
    }

    #[test]
    fn kmer_composition() {
        let text = "CAATCCAAC";
        let kmers = super::kmer_composition(text, 5);
        assert_eq!(kmers, vec![b"AATCC",
                               b"ATCCA",
                               b"CAATC",
                               b"CCAAC",
                               b"TCCAA"]);
    }

    #[test]
    fn locate_kmers() {
        let genome = "CGACACGACATTGCGACATA";
        let res = super::locate_kmers(genome, 5).to_hashmap().clone();
        assert!(res.contains_key(&b"CGACA"[..]));
        let locations = res.get(&b"CGACA"[..]).unwrap();
        assert_eq!(*locations, vec![0, 5, 13]);
    }


    #[test]
    fn find_clumps() {
        let mut kmer_locations = super::KmerLocator::new();
        kmer_locations.insert_locations("A", vec![1,3,5,7]);
        kmer_locations.insert_locations("B", vec![1,2,3,4]);
        kmer_locations.insert_locations("C", vec![1,10,20,30]);

        let res = kmer_locations.find_clumps(3, 3, 1);
        assert_eq!(res, vec!["B".to_string()]);
    }

    #[test]
    fn find_clumps2() {
        let genome = "CGGACTCGACAGATGTGAAGAAATGTGAAGACTGAGTGAAGAGAAGAGGAAACACGACACGACATTGCGACATAATGTACGAATGTAATGTGCCTATGGC";
        let (k, l, t) = (5, 75, 4);
        let kmer_locations = super::locate_kmers(genome, k);
        let mut clumps = kmer_locations.find_clumps(l, t, k);
        let mut expected = vec!["CGACA", "GAAGA", "AATGT"];
        assert_eq!(clumps.sort(), expected.sort());
    }
}

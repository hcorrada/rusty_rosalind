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

/// count kmers with mismatches
///
pub fn count_mismatch_kmers(text: &str, k: usize, d:usize) -> HashMap<Vec<u8>, i32> {
    let mut kmer_counts = HashMap::new();

    let n = text.len();
    for start in 0..n-k+1 {
        let kmer = &text[start..start+k];
        let kmer_neighborhood = neighborhood(kmer, d);
        for approximate_kmer in kmer_neighborhood.iter() {
            let counter = kmer_counts.entry(approximate_kmer.clone()).or_insert(0 as i32);
            *counter += 1;
        }
    }
    kmer_counts
}

/// get frequent kmers
///
pub fn find_frequent_kmers(kmer_counts: &HashMap<Vec<u8>, i32>) ->  Vec<String> {
    let mut res: Vec<String> = Vec::new();
    let mut highest_count: i32 = 0;
    for (kmer, count) in kmer_counts {
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
    use std::collections::HashMap;
    use dna_utils::num_mismatches;


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
        let counts = super::count_mismatch_kmers(text, 4, 1);
        assert_eq!(counts[&b"GATG"[..]], 5);
    }

    #[test]
    fn find_frequent_kmers() {
        let mut kmer_counts = HashMap::new();
        kmer_counts.insert(b"AB".to_vec(), 4);
        kmer_counts.insert(b"AC".to_vec(), 2);
        kmer_counts.insert(b"AD".to_vec(), 4);
        kmer_counts.insert(b"AE".to_vec(), 1);
        let mut frequent_kmers = super::find_frequent_kmers(&kmer_counts);
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
}

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use std::collections::HashSet;
use std::collections::HashMap;

mod neighborhood;

use neighborhood::Combinations;
use neighborhood::Product;
use neighborhood::KmerNeighborhood;

/// generate combinations
///
/// # Examples
///
/// ```
/// use rosalind_lib::combinations;
///
/// let combs: Vec<_> = combinations(3, 2).collect();
/// assert_eq!(combs, vec![[0,1], [0,2], [1,2]]);
pub fn combinations(n: usize, d: usize) -> Combinations {
    Combinations::new(n,d)
}

/// generate cartesian Product
///
/// # Examples
///
/// ```
/// use rosalind_lib::product;
///
/// let prod: Vec<_> = product(3,2).collect();
/// assert_eq!(prod, vec![[0,0],[0,1],[0,2],[1,0],[1,1],[1,2],[2,0],[2,1],[2,2]]);
pub fn product(n: usize, d: usize) -> Product {
    Product::new(n, d)
}

/// generate string neighborhood
///
/// # Examples
///
/// ```
/// use rosalind_lib::neighborhood;
/// use rosalind_lib::num_mismatches;
///
/// let res = neighborhood("ATCGA", 2);
/// assert!(res.iter().all(|x| num_mismatches(b"ATCGA", x) <= 2));
/// assert!(res.contains(&b"ATCGA"[..]));
/// assert!(res.contains(&b"TTCGA"[..]));
/// assert!(res.contains(&b"ATTGA"[..]));
/// assert!(!res.contains(&b"AAAAA"[..]));
/// assert!(!res.contains(&b"ATCG"[..]));
/// ```
///
/// ```
/// use rosalind_lib::neighborhood;
/// assert!(neighborhood("GTTG", 1).contains(&b"GATG"[..]));
/// ```
pub fn neighborhood(kmer: &str, d: usize) -> HashSet<Vec<u8>> {
    let kmer_neighborhood = KmerNeighborhood::new(kmer, d);
    let mut res = HashSet::new();
    for kmer in kmer_neighborhood {
        res.insert(kmer);
    }
    res
}

/// edit distance between strings
///
/// # Examples
///
/// ```
/// use rosalind_lib::num_mismatches;
///
/// assert_eq!(num_mismatches(b"AAA",b"AAA"),0);
/// assert_eq!(num_mismatches(b"AAA",b"ACA"),1);
/// assert_eq!(num_mismatches(b"TAT",b"AAA"),2);
pub fn num_mismatches(left: &[u8], right: &[u8]) -> usize {
    left.iter().zip(right)
        .filter(|&(x,y)| x != y)
        .count()
}

/// count kmers with mismatches
///
/// # Examples
///
/// ```
/// use rosalind_lib::count_mismatch_kmers;
///
/// let text = "ACGTTGCATGTCGCATGATGCATGAGAGCT";
/// let counts = count_mismatch_kmers(text, 4, 1);
/// assert_eq!(counts[&b"GATG"[..]], 5);
/// ```
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
/// # Examples
///
/// ```
/// use rosalind_lib::find_frequent_kmers;
/// use std::collections::HashMap;
///
/// let mut kmer_counts = HashMap::new();
/// kmer_counts.insert(b"AB".to_vec(), 4);
/// kmer_counts.insert(b"AC".to_vec(), 2);
/// kmer_counts.insert(b"AD".to_vec(), 4);
/// kmer_counts.insert(b"AE".to_vec(), 1);
/// let frequent_kmers = find_frequent_kmers(&kmer_counts);
/// assert_eq!(frequent_kmers, ["AB", "AD"]);
/// ```
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
/// use rosalind_lib::find_matches;
///
/// let pattern = "ATTCTGGA";
/// let text = "CGCCCGAATCCAGAACGCATTCCCATATTTCGGGACCACTGGCCTCCACGGTACGGACGTCAATCAAATGCCTAGCGGCTTGTGGTTTCTCCTACGCTCC";
/// let result = find_matches(pattern, text, 3);
/// assert_eq!(result, vec![6, 7, 26, 27, 78]);
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

/// reverse a String
///
/// # Examples
///
/// ```
/// use rosalind_lib::reverse;
///
/// let dna = "ATTCGA";
/// let res = reverse(dna);
/// assert_eq!(res, "AGCTTA");
/// ```
pub fn reverse(dna: &str) -> String {
    dna.chars().rev().collect()
}

/// complement a dna String
///
/// # Examples
///
/// ```
/// use rosalind_lib::complement;
///
/// let dna = "AGCTTA";
/// let res = complement(dna);
/// assert_eq!(res, "TCGAAT");
/// ```
pub fn complement(dna: &str) -> String {
    // map returns an iterator, collect
    // collects iterator characters into string
    dna.chars().map(|c| {
        match c {
            'A' => 'T',
            'C' => 'G',
            'G' => 'C',
            'T' => 'A',
             _  => 'N',
        }
    }).collect()
}

/// Compute reverse complement
///
/// # Examples
///
/// ```
/// use rosalind_lib::revcomp;
///
/// let dna = "AAAACCCGGT";
/// let res = revcomp(dna);
/// assert_eq!(res, "ACCGGGTTTT");
/// ```
pub fn revcomp(dna: &str) -> String {
    let revdna = reverse(&dna);
    complement(&revdna)
}

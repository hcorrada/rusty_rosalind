use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use std::collections::HashSet;

mod neighborhood;

use neighborhood::Combinations;
use neighborhood::Product;
use neighborhood::KmerNeighborhood;

/// generate combinations
///
/// # Examples
///
/// ```
/// use frequent_words_mismatch::combinations;
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
/// use frequent_words_mismatch::product;
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
/// use frequent_words_mismatch::neighborhood;
/// use frequent_words_mismatch::num_mismatches;
///
/// let res = neighborhood("ATCGA", 2);
/// assert!(res.iter().all(|x| num_mismatches(b"ATCGA", x) <= 2));
/// assert!(res.contains(&b"ATCGA"[..]));
/// assert!(res.contains(&b"TTCGA"[..]));
/// assert!(res.contains(&b"ATTGA"[..]));
/// assert!(!res.contains(&b"AAAAA"[..]));
/// assert!(!res.contains(&b"ATCG"[..]));
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
/// use frequent_words_mismatch::num_mismatches;
///
/// assert_eq!(num_mismatches(b"AAA",b"AAA"),0);
/// assert_eq!(num_mismatches(b"AAA",b"ACA"),1);
/// assert_eq!(num_mismatches(b"TAT",b"AAA"),2);
pub fn num_mismatches(left: &[u8], right: &[u8]) -> usize {
    left.iter().zip(right)
        .filter(|&(x,y)| x != y)
        .count()
}

/// find approximate matches
///
/// use frequent_words_mismatch::find_matches;
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

    for i in 0..n-k {
        let kmer = &text[i..i+k];
        if neighborhood.contains(kmer) {
            result.push(i);
        }
    }
    result
}

/// read input
///
/// # Examples
///
/// ```
/// use frequent_words_mismatch::read_input;
///
/// let res = read_input("test.txt");
/// assert_eq!(res.0, "ACGTTGCATGTCGCATGATGCATGAGAGCT".to_string());
/// assert_eq!(res.1, 4);
/// assert_eq!(res.2, 1);
/// ```
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

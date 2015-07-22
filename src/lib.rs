use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

mod neighborhood;

use neighborhood::Combinations;
use neighborhood::Product;

/// generate combinations
///
/// # Examples
///
/// ```
/// use approximate_matching::combinations;
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
/// use approximate_matching::product;
///
/// let prod: Vec<_> = product(3,2).collect();
/// assert_eq!(prod, vec![[0,0],[0,1],[0,2],[1,0],[1,1],[1,2],[2,0],[2,1],[2,2]]);
pub fn product(n: usize, d: usize) -> Product {
    Product::new(n, d)
}

/// read input
///
/// # Examples
///
/// ```
/// use approximate_matching::read_input;
///
/// let res = read_input("test.txt");
/// assert_eq!(res.0, "ATTCTGGA".to_string());
/// assert_eq!(res.1, "CGCCCGAATCCAGAACGCATTCCCATATTTCGGGACCACTGGCCTCCACGGTACGGACGTCAATCAAATGCCTAGCGGCTTGTGGTTTCTCCTACGCTCC".to_string());
/// assert_eq!(res.2, 3);
/// ```
pub fn read_input(filename: &str) -> (String, String, usize) {
    let fhandle = File::open(filename)
        .ok()
        .expect("Couldn't open file");

    let mut lines = BufReader::new(fhandle).lines();

    let pattern = if let Some(Ok(x)) = lines.next() {
        x
    } else {
        panic!("Could not read pattern");
    };

    let text = if let Some(Ok(x)) = lines.next() {
        x
    } else {
        panic!("Could not read text");
    };

    let dstr = if let Some(Ok(x)) = lines.next() {
        x
    } else {
        panic!("Could not read d");
    };

    let d = if let Ok(x) = dstr.parse() { x
    } else {
        panic!("Could not parse d");
    };

    (pattern, text, d)
}

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

mod neighborhood;

use neighborhood::Combinations;

/// generate combinations
pub trait Combinator : Iterator {
    fn combinations(self) -> Combinations<Self>
        where Self: Sized + Clone, Self::Item: Clone {

        Combinations::new(self)
    }
}

impl<T: ?Sized> Combinator for T where T: Iterator { }


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

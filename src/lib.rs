use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

/// read input
///
/// # Examples
///
/// ```
/// use frequent_words_mismatch_revcomp::read_input;
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

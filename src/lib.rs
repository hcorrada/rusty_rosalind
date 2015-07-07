use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

/// read problem input
///
/// # Examples
///
/// ```
/// use frequent_words::read_input;
///
/// let (dna, k) = read_input("test.txt");
/// assert_eq!(dna, "ACGTTGCATGTCGCATGATGCATGAGAGCT");
/// assert_eq!(k, 4);
/// ```
pub fn read_input(filename: &str) -> (String, i32) {
    let fhandle = File::open(filename)
        .ok()
        .expect("Could not open file");
    let reader = BufReader::new(fhandle);
    let mut lines = reader.lines();

    let dna = lines.next().unwrap().unwrap();
    let k: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    return (dna, k);
}

use std::fs::File;
use std::io::Read;

/// Compute reverse complement
///
/// # Examples
///
/// ```
/// use revcomp::revcomp;
///
/// let dna = "AAAACCCGGT";
/// let res = revcomp(dna);
/// assert_eq!(res, "ACCGGGTTTT");
/// ```
pub fn revcomp(dna: &str) -> String {
    let revdna = reverse(&dna);
    complement(&revdna)
}

/// reverse a String
///
/// # Examples
///
/// ```
/// use revcomp::reverse;
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
/// use revcomp::complement;
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

/// read string from file
///
/// # Examples
///
///```
/// use revcomp::read_input;
///
/// let filename = "test.txt";
/// let res = read_input(filename);
/// assert_eq!(res, "AAAACCCGGT");
///```
pub fn read_input(filename: &str) -> String {
    let mut fhandle = File::open(filename)
        .ok()
        .expect("Could not open file");

    let mut string = String::new();
    fhandle.read_to_string(&mut string)
        .ok()
        .expect("Could not read file");
    string.trim().to_string()
}

/// revcomp string read from file
///
/// # Examples
///
/// ```
/// use revcomp::runit;
///
/// let filename = "test.txt";
/// let res = runit(filename);
/// assert_eq!(res, "ACCGGGTTTT")
/// ```
pub fn runit(filename: &str) -> String {
    let string = read_input(filename);
    revcomp(&string)
}

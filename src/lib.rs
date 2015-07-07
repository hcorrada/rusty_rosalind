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
    return complement(&revdna);
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
    let res = dna.chars().rev().collect();
    return res;
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
    let res = dna.chars().map(|c| {
        match c {
            'A' => 'T',
            'C' => 'G',
            'G' => 'C',
            'T' => 'A',
             _  => 'N',
        }
    }).collect();
    return res;
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
    let string = string.trim().to_string();
    return string;
}

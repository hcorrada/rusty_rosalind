use std::fs::File;
use std::io::Read;

/// Transcribe DNA to RNA
///
/// # Examples
///
/// ```
/// use transcribe::transcribe;
/// let dna = "GATGGAACTTGACTACGTAAATT";
/// let res = transcribe(dna);
/// assert_eq!(res, "GAUGGAACUUGACUACGUAAAUU")
/// ```
pub fn transcribe(dna: &str) -> String {
    let rna = dna.replace("T", "U");
    return rna;
}

/// Read problem input from file
///
/// # Examples
///
/// ```
/// use transcribe::read_input;
///
/// let filename = "test.txt";
/// let dna = "GATGGAACTTGACTACGTAAATT";
/// let res = read_input(filename);
/// assert_eq!(res, dna);
/// ```
pub fn read_input(filename: &str) -> String {
    let mut fhandle = File::open(filename)
        .ok()
        .expect("Could not open file");

    let mut dna = String::new();
    fhandle.read_to_string(&mut dna)
        .ok()
        .expect("Unable to read");

    let dna = dna.trim().to_string();
    return dna;
}

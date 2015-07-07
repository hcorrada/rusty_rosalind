use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

/// compute gc content of a string
///
/// # Examples
///
/// ```
/// use gc_content::gc_content;
///
/// let dna = "AGCTATAG";
/// assert_eq!(gc_content(dna), 37.5);
/// ```
pub fn gc_content(dna: &str) -> f32 {
    let mut count = 0.0;
    for c in dna.chars() {
        count += match c {
            'C' | 'G' => 1.0,
            'A' | 'T' => 0.0,
            _ => unreachable!(),
        }
    }
    return 100.0 * count / dna.len() as f32;
}

/// parse fasta file
///
/// # Examples
///
/// ```
/// use gc_content::parse_fasta_file;
///
/// let records = parse_fasta_file("test.txt");
/// assert!(records.contains_key("Rosalind_6404"));
/// assert_eq!(records.get("Rosalind_6404").unwrap(), "CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCCTCCCACTAATAATTCTGAGG");
/// ```
pub fn parse_fasta_file(filename: &str) -> HashMap<String, String> {
    let fhandle = File::open(filename)
        .ok()
        .expect("Could not open file");
    let reader = BufReader::new(fhandle);
    let mut records = HashMap::new();

    let mut curline;
    let mut id = String::new();
    let mut dna = String::new();

    for line in reader.lines() {
        if line.is_err() { continue };

        curline = line.unwrap();
        if curline.starts_with(">") {
            records.insert(id, dna);
            id = curline[1..].to_string();
            dna = String::new();
        } else {
            dna.push_str(&curline);
        }
    }

    records.insert(id, dna);
    return records;
}

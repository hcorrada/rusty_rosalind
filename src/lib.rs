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
    100.0 * count / dna.len() as f32
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
/// assert!(records.contains_key("Rosalind_5959"));
/// assert!(records.contains_key("Rosalind_0808"));
/// assert_eq!(records.get("Rosalind_6404").unwrap(), "CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCCTCCCACTAATAATTCTGAGG");
/// ```
pub fn parse_fasta_file(filename: &str) -> HashMap<String, String> {
    let fhandle = File::open(filename)
        .ok()
        .expect("Could not open file");
    let reader = BufReader::new(fhandle);
    let mut records = HashMap::new();

    let mut id = String::new();
    let mut dna = String::new();

    for line in reader.lines() {
        if line.is_err() {
            break;
        };

        let curline = line.unwrap();
        if curline.starts_with(">") {
            if id.len() > 0 { records.insert(id, dna); };
            id = curline[1..].to_string();
            dna = String::new();
        } else {
            dna.push_str(&curline);
        }
    }
    if id.len() > 0 { records.insert(id, dna); };
    records
}

/// run counter on fasta file return record with highest gc content
///
/// # Examples
///
/// ```
/// use gc_content::runit;
///
/// let res = runit("test.txt");
/// assert_eq!(res, ("Rosalind_0808".to_string(), 60.919540));
/// ```
pub fn runit(filename: &str) -> (String, f32) {
    let records = parse_fasta_file(&filename);
    let mut maxgc = -1.0 as f32;
    let mut maxid = "";

    for (id, dna) in &records {
        let curgc = gc_content(&dna);
        if curgc > maxgc {
            maxgc = curgc;
            maxid = id;
        }
    }
    (maxid.to_string(), maxgc)
}

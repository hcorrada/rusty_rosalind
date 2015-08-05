use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

pub fn parse_fasta_file(filename: &str) -> HashMap<String, String> {
    let fhandle = File::open(filename)
        .ok()
        .expect("Could not open file");
    let mut lines = BufReader::new(fhandle).lines();
    let mut records = HashMap::new();

    let mut id = String::new();
    let mut dna = String::new();

    while let Some(Ok(curline)) = lines.next() {
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

#[cfg(test)]
mod test {
    #[test]
    fn parse_fasta_file() {
        let records = super::parse_fasta_file("test.fa");
        assert!(records.contains_key("Rosalind_6404"));
        assert!(records.contains_key("Rosalind_5959"));
        assert!(records.contains_key("Rosalind_0808"));
        assert_eq!(records.get("Rosalind_6404").unwrap(), "CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCCTCCCACTAATAATTCTGAGG");
    }
}

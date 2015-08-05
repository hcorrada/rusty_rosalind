/// reverse a String
///
pub fn reverse(dna: &str) -> String {
    dna.chars().rev().collect()
}

/// complement a dna String
///
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

/// Compute reverse complement
///
pub fn revcomp(dna: &str) -> String {
    let revdna = reverse(&dna);
    complement(&revdna)
}

/// edit distance between strings
///
pub fn num_mismatches(left: &[u8], right: &[u8]) -> usize {
    left.iter().zip(right)
        .filter(|&(x,y)| x != y)
        .count()
}

use std::collections::BTreeMap;

/// count nucleotides in a string
///
pub fn count_nucleotides(string: &str) -> Vec<i32> {
    // use a BTreeMap to order the keys of the hash so that
    // keys are ordered (ACGT)
    let mut map = BTreeMap::new();

    // add 1 to counter hash entry for each character
    // in string
    for c in string.chars() {
        *map.entry(c).or_insert(0) += 1;
    }

    // return vector of nucleotide counts
    map.values().cloned().collect()
}

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

/// Transcribe DNA to RNA
///
pub fn transcribe(dna: &str) -> String {
    dna.replace("T", "U")
}

#[cfg(test)]
mod test {
    #[test]
    fn reverse() {
        let dna = "ATTCGA";
        let res = super::reverse(dna);
        assert_eq!(res, "AGCTTA");
    }

    #[test]
    fn complement() {
        let dna = "AGCTTA";
        let res = super::complement(dna);
        assert_eq!(res, "TCGAAT");
    }

    #[test]
    fn revcomp() {
        let dna = "AAAACCCGGT";
        let res = super::revcomp(dna);
        assert_eq!(res, "ACCGGGTTTT");
    }

    #[test]
    fn num_mismatches() {
        assert_eq!(super::num_mismatches(b"AAA",b"AAA"),0);
        assert_eq!(super::num_mismatches(b"AAA",b"ACA"),1);
        assert_eq!(super::num_mismatches(b"TAT",b"AAA"),2);
    }

    #[test]
    fn count_nucleotides() {
        let string = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";
        let res = super::count_nucleotides(string);
        assert_eq!(res, vec![20, 12, 17, 21])
    }

    #[test]
    fn gc_content() {
        let dna = "AGCTATAG";
        assert_eq!(super::gc_content(dna), 37.5);
    }

    #[test]
    fn transcribe() {
        let dna = "GATGGAACTTGACTACGTAAATT";
        let res = super::transcribe(dna);
        assert_eq!(res, "GAUGGAACUUGACUACGUAAAUU")
    }
}

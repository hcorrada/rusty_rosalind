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
}

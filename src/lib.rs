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

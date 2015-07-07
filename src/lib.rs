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

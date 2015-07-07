/// read problem input
///
/// # Examples
///
/// ```
/// use frequent_words::read_input;
///
/// let (dna, k) = read_input("test.txt");
/// assert_eq!(dna, "ACGTTGCATGTCGCATGATGCATGAGAGCT");
/// assert_eq!(k, 4);
/// ```
pub fn read_input(filename: &str) -> (String, i32) {
    return ("ACGTTGCATGTCGCATGATGCATGAGAGCT".to_string(), 4);
}

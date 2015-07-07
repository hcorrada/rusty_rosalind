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

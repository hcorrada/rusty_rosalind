use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

/// do pattern matching
///
/// # Examples
///
/// ```
/// use pattern_matching::match_pattern;
///
/// let pattern = "ATAT";
/// let genome = "GATATATGCATATACTT";
/// let res = match_pattern(pattern, genome);
/// assert_eq!(res, vec![1, 3, 9]);
/// ```
pub fn match_pattern(pattern: &str, genome: &str) -> Vec<usize> {
    let k = pattern.len();
    let n = genome.len();
    let mut res = Vec::new();

    for i in 0..n-k {
        if pattern == &genome[i..i+k] { res.push(i); }
    }
    res
}

/// read input from file
///
/// # Examples
///
/// ```
/// use pattern_matching::read_input;
///
/// let (pattern, genome) = read_input("test.txt");
/// assert_eq!(pattern, "ATAT".to_string());
/// assert_eq!(genome, "GATATATGCATATACTT".to_string());
/// ```
pub fn read_input(filename: &str) -> (String, String) {
    // open file and get lines iterator
    let fhandle = File::open(filename)
                    .ok()
                    .expect("Could not open file");
    let reader  = BufReader::new(fhandle);
    let mut lines = reader.lines();

    // read the pattern line
    let pattern: String = match lines.next() {
        Some(v) => v.ok()
                    .expect("Could not read pattern input line")
                    .clone(),
        None => panic!("No input pattern line found"),
    };

    // read the genome line
    let genome: String = match lines.next() {
        Some(v) => v.ok()
                    .expect("Could not read genome input line")
                    .clone(),
        None => panic!("No input genome line found"),
    };

    // return input
    (pattern, genome)
}

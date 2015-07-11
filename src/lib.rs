use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

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

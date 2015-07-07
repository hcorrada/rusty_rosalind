use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

/// get frequent kmers
///
/// # Examples
///
/// ```
/// use frequent_words::find_frequent_kmers;
/// use std::collections::HashMap;
///
/// let mut kmer_counts = HashMap::new();
/// kmer_counts.insert("AB".to_string(), 4);
/// kmer_counts.insert("AC".to_string(), 2);
/// kmer_counts.insert("AD".to_string(), 4);
/// kmer_counts.insert("AE".to_string(), 1);
/// let frequent_kmers = find_frequent_kmers(kmer_counts);
/// assert_eq!(frequent_kmers, ["AB", "AD"]);
/// ```
pub fn find_frequent_kmers(kmer_counts: HashMap<String, i32>) ->  Vec<String> {
    let mut res: Vec<String> = Vec::new();
    let mut highest_count: i32 = 0;
    for (kmer, count) in &kmer_counts {
        if *count > highest_count {
            highest_count = count.clone();
            res = Vec::new();
            res.push(kmer.to_string());
        } else if *count == highest_count {
            res.push(kmer.to_string());
        }
    }
    return res;
}

/// count kmers
///
/// # Examples
///
/// ```
/// use frequent_words::count_kmers;
///
/// let dna = "ACAACTATGCATACTATCGGGAACTATCCT";
/// let kmer_counts = count_kmers(dna, 5);
/// assert_eq!(kmer_counts["ACTAT"], 3);
/// ```
pub fn count_kmers(dna: &str, k: usize) -> HashMap<String, i32> {
    let mut kmer_counts = HashMap::new();

    let n = dna.len();
    for start in 0..n-k+1 {
        let kmer = &dna[start..start+k];
        let counter = kmer_counts.entry(kmer.to_string()).or_insert(0 as i32);
        *counter += 1;
    }

    return kmer_counts;
}

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
pub fn read_input(filename: &str) -> (String, usize) {
    let fhandle = File::open(filename)
        .ok()
        .expect("Could not open file");
    let reader = BufReader::new(fhandle);
    let mut lines = reader.lines();

    let dna = lines.next().unwrap().unwrap();
    let k: usize = lines.next().unwrap().unwrap().parse().unwrap();
    return (dna, k);
}

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
/// let frequent_kmers = find_frequent_kmers(&kmer_counts);
/// assert_eq!(frequent_kmers, ["AB", "AD"]);
/// ```
pub fn find_frequent_kmers(kmer_counts: &HashMap<String, i32>) ->  Vec<String> {
    let mut res: Vec<String> = Vec::new();
    let mut highest_count: i32 = 0;
    for (kmer, count) in kmer_counts {
        if *count > highest_count {
            highest_count = count.clone();
            res.clear();
            res.push(kmer.to_string());
        } else if *count == highest_count {
            res.push(kmer.to_string());
        }
    }
    res
}

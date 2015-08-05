/// do pattern matching
///
pub fn naive(pattern: &str, genome: &str) -> Vec<usize> {
    let k = pattern.len();
    let n = genome.len();
    let mut res = Vec::new();

    for i in 0..n-k {
        if pattern == &genome[i..i+k] { res.push(i); }
    }
    res
}

#[cfg(test)]
mod test {
    #[test]
    fn naive() {
        let pattern = "ATAT";
        let genome = "GATATATGCATATACTT";
        let res = super::naive(pattern, genome);
        assert_eq!(res, vec![1, 3, 9]);
    }
}

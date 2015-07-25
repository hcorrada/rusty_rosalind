extern crate itertools;
use self::itertools::Itertools;

#[derive(Clone)]
pub struct Product {
    n: usize,
    d: usize,
    vals: Vec<usize>,
}

impl Product {
    pub fn new(n: usize, d: usize) -> Product {
        Product {
            n: n,
            d: d,
            vals: Vec::new(),
        }
    }
}

impl Iterator for Product {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.vals.len() == 0 {
            for _ in 0..self.d {
                self.vals.push(0);
            }
            return Some(self.vals.clone());
        }

        let mut cur_position = self.d - 1;
        while cur_position >= 0 {
            if self.vals[cur_position] < self.n - 1 {
                self.vals[cur_position] += 1;
                break;
            }
            if cur_position == 0 { return None };
            cur_position -= 1;
        }
        cur_position += 1;
        while cur_position < self.d {
            self.vals[cur_position] = 0;
            cur_position += 1;
        }
        Some(self.vals.clone())
    }
}

pub struct Combinations {
    n:    usize,
    d:    usize,
    vals: Vec<usize>,
}

impl Combinations {
    pub fn new(n: usize, d: usize) -> Combinations {
        if d >= n {
            panic!("Can't create these combinations");
        }

        Combinations {
            n: n,
            d: d,
            vals: Vec::new(),
        }
    }
}

impl Iterator for Combinations {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        // if value vector is empty, initialize
        if self.vals.len() == 0 {
            for i in 0..self.d {
                self.vals.push(i);
            }
            return Some(self.vals.clone());
        }

        // check if we can advance index from last to first position
        let mut cur_position = self.d - 1;
        while cur_position >= 0 {
            if self.vals[cur_position] + (self.d - cur_position) < self. n {
                // advance index in this column and break loop
                self.vals[cur_position] += 1;
                break;
            }

            // if this is the first position, there are no more combinations
            if cur_position == 0 { return None };
            cur_position -= 1;
        }

        // we advanced some position, go from this position to the last to
        // update the combination accordingly
        cur_position += 1;
        while cur_position < self.d {
            self.vals[cur_position] = self.vals[cur_position - 1] + 1;
            cur_position += 1;
        }
        // return resulting vector
        Some(self.vals.clone())
    }
}

pub struct KmerNeighborhood {
    chars: Vec<char>,
    index_iterator: itertools::Product<Combinations, Product>,
}

impl KmerNeighborhood {
    pub fn new(kmer: &str, d: usize) -> KmerNeighborhood {
        let chars: Vec<_> = kmer.chars().collect();
        let n = chars.len();
        let combinations = Combinations::new(n, d);
        let product = Product::new(n, d);
        let index_iterator = combinations.cartesian_product(product);

        KmerNeighborhood {
            chars: chars.clone(),
            index_iterator: index_iterator,
        }
    }

    pub fn build_string(&self, ivec: &Vec<usize>, jvec: &Vec<usize>) -> String {
        let mut new_chars = self.chars.clone();
        for (i,j) in ivec.iter().zip(jvec) {
            new_chars[*i] = self.chars[*j];
        }
        new_chars.iter().join("")
    }
}

impl Iterator for KmerNeighborhood {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let indices: Option<(Vec<usize>, Vec<usize>)> = self.index_iterator.next();
        match indices {
            None => None,
            Some((ivec, jvec)) => Some(self.build_string(&ivec, &jvec)),
        }
    }
}

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

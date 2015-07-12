pub struct Combinations<I: Iterator> {
    iter: I,
    next_iter: I,
    val: Option<I::Item>,
}

impl<I> Combinations<I> where I: Iterator + Clone {
    pub fn new(iter: I) -> Combinations<I> {
        Combinations {
            next_iter: iter.clone(),
            iter: iter,
            val: None,
        }
    }
}

impl<I> Iterator for Combinations<I>
    where I: Iterator + Clone,
          I::Item: Clone {

    type Item = (I::Item, I::Item);
    fn next(&mut self) -> Option<Self::Item> {
        if self.val.is_none() {
            self.val = self.iter.next();
            self.next_iter = self.iter.clone();
        }

        let elt = match self.val {
            Some(ref x) => x.clone(),
            None => return None,
        };

        match self.next_iter.next() {
            Some(ref x) => {
                return Some((elt, x.clone()));
            },
            None => {
                self.val = None;
            }
        }
        self.next()
    }
}

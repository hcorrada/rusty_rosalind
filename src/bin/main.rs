extern crate gc_content;

use std::env;
use gc_content::runit;

fn main() {
    let filename = env::args().nth(1)
        .expect("Need one argument");

    let (id, gc) = runit(&filename);
    println!("{}", id);
    println!("{}", gc);
}

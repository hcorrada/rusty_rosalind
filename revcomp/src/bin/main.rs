extern crate revcomp;

use std::env;
use revcomp::runit;

fn main() {
    let filename = env::args().nth(1)
        .expect("Need one input");
    let res = runit(&filename);
    println!("{}", res);
}

extern crate transcribe;

use std::env;

fn main() {
    let filename = env::args().nth(1)
        .expect("Need one argument");

    let string = transcribe::read_input(&filename);
    let res = transcribe::transcribe(&string);
    println!("{}", res);
}
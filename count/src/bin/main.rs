use std::env;
extern crate count_nucleotides;

fn main() {
    let filename = env::args().nth(1)
        .expect("Need one argument");
    let string = count_nucleotides::read_input(&filename);
    let res = count_nucleotides::count_nucleotides(&string);
    let res = count_nucleotides::make_output(&res);
    println!("{}", &res);
}

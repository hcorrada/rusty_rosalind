extern crate gc_content;

use gc_content::parse_fasta_file;

fn main() {
    let res = parse_fasta_file("test.txt");
    println!("{:?}", res);
}

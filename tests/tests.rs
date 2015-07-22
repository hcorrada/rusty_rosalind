extern crate approximate_matching;

use approximate_matching::Combinator;

#[test]
fn combinations1() {
    let combs: Vec<_> = (0..3).combinations(2).collect();
    assert_eq!(combs, vec![(0,1), (0,2), (1,2)]);
}

extern crate algorithms;

use algorithms::boomerangs::count_boomerangs;
use std::vec;

#[test]
fn it_counts_boomerangs() {
    {
        assert_eq!(2, count_boomerangs(vec![(0, 0), (1, 0), (2, 0)]));
    }
    {
        assert_eq!(2, count_boomerangs(vec![(1, 1), (2, 2), (3, 3)]));
    }
    {
        assert_eq!(0, count_boomerangs(vec![(1, 1)]));
    }
    {
        assert_eq!(0, count_boomerangs(vec![]));
    }
}

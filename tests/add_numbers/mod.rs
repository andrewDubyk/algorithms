extern crate algorithms;

use algorithms::add_numbers::add_two_lists;
use std::collections::LinkedList;

#[test]
fn it_adds_two_lists() {
    {
        let mut l1 = LinkedList::new();
        l1.push_back(7);
        l1.push_back(2);
        l1.push_back(4);
        l1.push_back(3);

        let mut l2 = LinkedList::new();
        l2.push_back(5);
        l2.push_back(6);
        l2.push_back(4);

        let mut result = LinkedList::new();
        result.push_back(7);
        result.push_back(8);
        result.push_back(0);
        result.push_back(7);

        assert_eq!(result, add_two_lists(l1, l2));
    }
    {
        let mut l1 = LinkedList::new();
        l1.push_back(7);
        l1.push_back(0);
        l1.push_back(9);
        l1.push_back(9);

        let mut l2 = LinkedList::new();
        l2.push_back(5);
        l2.push_back(6);
        l2.push_back(4);
        l2.push_back(1);
        l2.push_back(8);

        let mut result = LinkedList::new();
        result.push_back(6);
        result.push_back(3);
        result.push_back(5);
        result.push_back(1);
        result.push_back(7);

        assert_eq!(result, add_two_lists(l1, l2));
    }
}

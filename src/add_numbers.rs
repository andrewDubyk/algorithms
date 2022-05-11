use std::collections::LinkedList;

/// Add the two numbers (represented as linked lists) and return the sum as a linked list.
/// Linked list stores a number where the most significant digit comes first and each of list nodes contains a single digit.
///
/// # Arguments
///
/// * `l1` - Linked list with a first number
/// * `l2` - Linked list with a second number
///
pub fn add_two_lists(mut l1: LinkedList<i32>, mut l2: LinkedList<i32>) -> LinkedList<i32> {
    let mut sum;
    let mut carry = 0;
    let mut result: LinkedList<i32> = LinkedList::new();

    while !l1.is_empty() || !l2.is_empty() || carry != 0 {
        // Get last element from first list
        let n1 = l1.pop_back().unwrap_or(0);
        // Get last element from second list
        let n2 = l2.pop_back().unwrap_or(0);

        sum = n1 + n2 + carry;
        carry = sum / 10;

        // Add elements in reverse order to avoid reversing of resulted list
        result.push_front(sum % 10);
    }

    result
}

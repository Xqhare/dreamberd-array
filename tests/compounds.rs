// Arrays, Tuples, and Vectors
// Hash Maps, Btrees, and Sets

use dreamberd_array::List;

#[test]
fn array() {
    let input: Vec<Vec<usize>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let mut list = List::new();
    for i in input {
        list.push(i);
    }
    assert_eq!(list.pop(), Some(vec![7, 8, 9]));
    assert_eq!(list.pop(), Some(vec![4, 5, 6]));
    assert_eq!(list.pop(), Some(vec![1, 2, 3]));
    assert_eq!(list.pop(), None);
}

// Arrays, Tuples, and Vectors
// Hash Maps, Btrees, and Sets

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use dreamberd_array::List;

#[test]
fn vector() {
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

#[test]
fn tuple() {
    let input: Vec<(usize, usize, usize)> = vec![(1, 2, 3), (4, 5, 6), (7, 8, 9)];
    let mut list = List::new();
    for i in input {
        list.push(i);
    }
    assert_eq!(list.pop(), Some((7, 8, 9)));
    assert_eq!(list.pop(), Some((4, 5, 6)));
    assert_eq!(list.pop(), Some((1, 2, 3)));
    assert_eq!(list.pop(), None);
}

#[test]
fn array() {
    let input: [[usize; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let mut list = List::new();
    for i in input {
        list.push(i);
    }
    assert_eq!(list.pop(), Some([7, 8, 9]));
    assert_eq!(list.pop(), Some([4, 5, 6]));
    assert_eq!(list.pop(), Some([1, 2, 3]));
    assert_eq!(list.pop(), None);
}

#[test]
fn hashmap() {
    let input: Vec<(usize, usize)> = vec![(1, 2), (3, 4), (5, 6)];
    let vec_of_maps: Vec<HashMap<usize, usize>> = input.into_iter().map(|(k, v)| {let mut map = HashMap::new(); map.insert(k, v); map}).collect(); 
    let mut list = List::new();
    for i in vec_of_maps {
        list.push(i);
    }
    // No order
    assert_eq!(list.len(), 3);
}

#[test]
fn hashset() {
    let input: Vec<usize> = vec![1, 2, 3];
    let hashset: HashSet<usize> = input.into_iter().collect();
    let mut list = List::new();
    for i in hashset {
        list.push(i);
    }
    // No order
    assert_eq!(list.len(), 3);
}

#[test]
fn btreemap() {
    let input: Vec<(usize, usize)> = vec![(1, 2), (3, 4), (5, 6)];
    let vec_of_maps: Vec<BTreeMap<usize, usize>> = input.into_iter().map(|(k, v)| {let mut map = BTreeMap::new(); map.insert(k, v); map}).collect(); 
    let mut list = List::new();
    for i in vec_of_maps {
        list.push(i);
    }
    // No order
    assert_eq!(list.len(), 3);
}

#[test]
fn btreeset() {
    let input: Vec<(usize, usize)> = vec![(1, 2), (3, 4), (5, 6)];
    let vec_of_maps: Vec<BTreeSet<usize>> = {
        let mut output = Vec::new();
        for (k, v) in input {
            let mut map = BTreeSet::new();
            map.insert(k);
            map.insert(v);
            output.push(map);
        }
        output
    }; 
    let mut list = List::new();
    for i in vec_of_maps {
        list.push(i);
    }
    // No order
    assert_eq!(list.len(), 3);
}

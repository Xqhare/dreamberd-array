use dreamberd_array::List;

#[test]
fn usize() {
    let input: Vec<usize> = vec![1, 2, 3];
    let mut list = List::new();
    for i in input {
        list.push(i);
    }
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(2));
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);
}

#[test]
fn isize() {
    let input: Vec<isize> = vec![-1, -2, -3];
    let mut list = List::new();
    for i in input {
        list.push(i);
    }
    assert_eq!(list.pop(), Some(-3));
    assert_eq!(list.pop(), Some(-2));
    assert_eq!(list.pop(), Some(-1));
    assert_eq!(list.pop(), None);
}

#[test]
fn float() {
    let input: Vec<f32> = vec![1.0, 2.0, 3.0];
    let mut list = List::new();
    for i in input {
        list.push(i);
    }
    assert_eq!(list.pop(), Some(3.0));
    assert_eq!(list.pop(), Some(2.0));
    assert_eq!(list.pop(), Some(1.0));
    assert_eq!(list.pop(), None);

    let input_64: Vec<f64> = vec![1.0, 2.0, 3.0];
    let mut list_64 = List::new();
    for i in input_64 {
        list_64.push(i);
    }
    assert_eq!(list_64.pop(), Some(3.0));
    assert_eq!(list_64.pop(), Some(2.0));
    assert_eq!(list_64.pop(), Some(1.0));
    assert_eq!(list_64.pop(), None);
}

#[test]
fn char() {
    let input: Vec<char> = vec!['a', 'b', 'c'];
    let mut list = List::new();
    for i in input {
        list.push(i);
    }
    assert_eq!(list.pop(), Some('c'));
    assert_eq!(list.pop(), Some('b'));
    assert_eq!(list.pop(), Some('a'));
    assert_eq!(list.pop(), None);
}

#[test]
fn bool() {
    let input: Vec<bool> = vec![true, false, true];
    let mut list = List::new();
    for i in input {
        list.push(i);
    }
    assert_eq!(list.pop(), Some(true));
    assert_eq!(list.pop(), Some(false));
    assert_eq!(list.pop(), Some(true));
    assert_eq!(list.pop(), None);
}


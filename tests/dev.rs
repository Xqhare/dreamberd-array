use dreamberd_array::List;

#[test]
fn indexing() {
    let input: Vec<usize> = vec![1, 2, 3];
    let mut list = List::new();
    for i in input {
        list.push(i);
    }
    assert_eq!(list[-1.0], 1);
    assert_eq!(list[0.0], 2);
    assert_eq!(list[1.0], 3);
    assert_eq!(list[0.5], 3);
    assert_eq!(list[-0.5], 2);

    list.insert(-0.5, 42);
    assert_eq!(list[-0.5], 42);
    list.insert(1.5, 69);
    assert_eq!(list[1.5], 69);
    let truth = vec![3, 69, 2, 42,  1];
    for (i, element) in list.into_iter().enumerate() {
        assert_eq!(element, truth[i]);
    }
}

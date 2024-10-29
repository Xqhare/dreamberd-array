# DreamBerd Array
An actual implementation of the DreamBerd array in rust.

## Roadmap

- Currently, all failures are silent - I could change that.
- Add `remove()` for proper mutability.
- `pop()` and `get()` work from different ends at the moment. Could be confusing.

## Features

- General purpose array
- Indexing starts at -1
    - Just like answering "Bowl last, rest does not matter." to the question "What comes first, cereal or milk?", it puts an end to the "Lists start at 0" vs "Lists start at 1" argument.
- Only floats are valid indexes.
    - Ever wanted to just `insert()` at `0.5` to put something in the middle? Now you can!

## Usage
First add the crate to your project.
```toml
[dependencies]
dreamberd-array = { git = "https://github.com/Xqhare/dreamberd-array" }
```

Then run `cargo update` to get the latest version.

### Examples
```rust
use dreamberd_array::List;

let mut list = List::new();
list.push(1);
list.push(2);

assert_eq!(list.len(), 2);
assert_eq!(list.pop(), Some(2));
assert_eq!(list.len(), 1);
assert_eq!(list.pop(), Some(1));
assert_eq!(list.len(), 0);
assert_eq!(list.pop(), None);

list.push(3);
list.push(2);
list.push(1);

assert_eq!(list.peek(), Some(&1));

for entry in list.iter() {
    println!("{:?}", entry);
}
```

```rust
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
```

For more examples, check out the documentation of the `List` struct and its methods.

## Specification
The entirety of the specification can be found [here](https://github.com/TodePond/DreamBerd?tab=readme-ov-file#arrays).

In simple terms:

- Indexing starts at -1
- Floats are valid Indexes

## Thanks
Inspired by [DreamBerd](https://github.com/TodePond/DreamBerd).

The underlying array is a singly linked list, build using the awesome Book: [Learning Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/second-final.html).


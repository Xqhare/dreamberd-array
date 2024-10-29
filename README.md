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
    - Ever wanted to just write `insert(0.5)` to put something in the middle? Now you can!

## Specification
The entirety of the specification can be found [here](https://github.com/TodePond/DreamBerd?tab=readme-ov-file#arrays).

In simple terms:

- Indexing starts at -1
- Floats are valid Indexes

## Thanks
Inspired by [DreamBerd](https://github.com/TodePond/DreamBerd).

The underlying array is a singly linked list, build using the awesome Book: [Learning Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/second-final.html).


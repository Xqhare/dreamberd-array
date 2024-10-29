/*!
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
*/


use std::ops::{Index, IndexMut};

/// `List` is a simple linked list, designed after the DreamBerd array.
///
/// The index starts at -1 and increases by 1 for each element.
/// All indexing is floating-point based. This means you can finally `insert` at `0.5`!
///
/// It supports any type.
#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

/* impl<T: Default> Default for Node<T> {
    fn default() -> Self {
        Node {
            elem: Default::default(),
            next: None,
        }
    }
} */

pub struct IntoIter<T>(List<T>);
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    /// Creates a new and empty `List`.
    ///
    /// # Example
    /// ```
    /// use dreamberd_array::List;
    ///
    /// let mut list: List<isize> = List::new();
    /// assert_eq!(list.len(), 0);
    /// ```
    pub fn new() -> Self {
        List { head: None }
    }

    /// Pushes an element to the front of the list.
    /// 
    /// # Example
    /// ```
    /// use dreamberd_array::List;
    ///
    /// let mut list = List::new();
    /// list.push(1);
    ///
    /// assert_eq!(list.len(), 1);
    /// ```
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    /// Removes and returns the element from the front of the list.
    ///
    /// # Example
    /// ```
    /// use dreamberd_array::List;
    /// 
    /// let mut list = List::new();
    /// list.push(1);
    /// list.push(2);
    ///
    /// assert_eq!(list.len(), 2);
    /// assert_eq!(list.pop(), Some(2));
    /// assert_eq!(list.len(), 1);
    /// assert_eq!(list.pop(), Some(1));
    /// assert_eq!(list.len(), 0);
    /// assert_eq!(list.pop(), None);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    /// Returns a reference to the first element in the list.
    ///
    /// # Example
    /// ```
    /// use dreamberd_array::List;
    ///
    /// let mut list = List::new();
    /// list.push(1);
    ///
    /// assert_eq!(list.peek(), Some(&1));
    /// ```
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    /// Returns a mutable reference to the first element in the list.
    ///
    /// # Example
    /// ```
    /// use dreamberd_array::List;
    ///
    /// let mut list = List::new();
    /// list.push(1);
    ///
    /// assert_eq!(list.peek_mut(), Some(&mut 1));
    /// 
    /// list.peek_mut().map(|value| {
    ///     *value = 42
    /// });
    ///
    /// assert_eq!(list.peek(), Some(&42));
    /// ```
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

    /// Consumes the list and returns an iterator over it.
    ///
    /// # Example
    /// ```
    /// use dreamberd_array::List;
    ///
    /// let mut list = List::new();
    /// list.push(1);
    /// list.push(2);
    /// list.push(3);
    ///
    /// let mut iter = list.into_iter();
    /// assert_eq!(iter.next(), Some(3));
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next(), Some(1));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    /// Returns an iterator over the list.
    ///
    /// # Example
    /// ```
    /// use dreamberd_array::List;
    ///
    /// let mut list = List::new();
    /// list.push(1);
    /// list.push(2);
    /// list.push(3);
    ///
    /// let mut iter = list.iter();
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    /// Returns an iterator over the list.
    ///
    /// # Example
    /// ```
    /// use dreamberd_array::List;
    ///
    /// let mut list = List::new();
    /// list.push(1);
    /// list.push(2);
    /// list.push(3);
    ///
    /// let mut iter = list.iter_mut();
    /// assert_eq!(iter.next(), Some(&mut 3));
    /// assert_eq!(iter.next(), Some(&mut 2));
    /// assert_eq!(iter.next(), Some(&mut 1));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.head.as_deref_mut() }
    }

    /// Returns the number of elements in the list.
    ///
    /// # Example
    /// ```
    /// use dreamberd_array::List;
    ///
    /// let mut list = List::new();
    /// list.push(1);
    /// list.push(2);
    /// list.push(3);
    ///
    /// assert_eq!(list.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut cur_link = self.head.as_deref();
        while let Some(node) = cur_link {
            count += 1;
            cur_link = node.next.as_deref();
        }
        count
    }

    /// Returns whether the list is empty.
    ///
    /// # Example
    /// ```
    /// use dreamberd_array::List;
    ///
    /// let mut list = List::new();
    /// list.push(1);
    /// list.push(2);
    /// list.push(3);
    ///
    /// assert_eq!(list.is_empty(), false);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// Returns the element at the given index.
    ///
    /// The index starts with -1 at the end of the list and is in fractional form.
    /// 
    /// # Example
    /// ```
    /// use dreamberd_array::List;
    ///
    /// let mut list = List::new();
    /// list.push(1);
    /// list.push(2);
    /// list.push(3);
    ///
    /// assert_eq!(list.get(-1.0), Some(&1));
    /// assert_eq!(list.get(0.0), Some(&2));
    /// assert_eq!(list.get(1.0), Some(&3));
    /// assert_eq!(list.get(2.0), None);
    /// ```
    pub fn get(&self, index: f32) -> Option<&T> {
        let mut count = self.len() as f32 - 2.0;
        let mut cur_link = self.head.as_deref();
        if index < -1.0 {
            return None;
        } else if index - count > 0.0 {
            return None;
        }
        let mut index2 = index;
        if index.fract() != 0.0 {
            index2 += 1.0;
        }
        while let Some(node) = cur_link {
            if count <= index2 {
                return Some(&node.elem);
            }
            count -= 1.0;
            cur_link = node.next.as_deref();
        }
        None
    }

    /// Returns the element at the given index.
    ///
    /// The index starts with -1 at the end of the list and is in fractional form.
    /// 
    /// # Example
    /// ```
    /// use dreamberd_array::List;
    ///
    /// let mut list = List::new();
    /// list.push(1);
    /// list.push(2);
    /// list.push(3);
    ///
    /// assert_eq!(list.get_mut(-1.0), Some(&mut 1));
    /// assert_eq!(list.get_mut(0.0), Some(&mut 2));
    /// assert_eq!(list.get_mut(1.0), Some(&mut 3));
    /// assert_eq!(list.get_mut(2.0), None);
    /// ```
    pub fn get_mut(&mut self, index: f32) -> Option<&mut T> {
        let mut count = self.len() as f32 - 2.0;
        let mut cur_link = self.head.as_deref_mut();
        if index < -1.0 {
            return None;
        } else if index - count > 0.0 {
            return None;
        }
        let mut index2 = index;
        if index.fract() != 0.0 {
            index2 += 1.0;
        }
        while let Some(node) = cur_link {
            if count <= index2 {
                return Some(&mut node.elem);
            }
            count -= 1.0;
            cur_link = node.next.as_deref_mut();
        }
        None
    }

    /// Inserts an element at the given index.
    ///
    /// The index starts with -1 at the end of the list and is in fractional form.
    /// 
    /// # Example
    /// ```
    /// use dreamberd_array::List;
    ///
    /// let mut list = List::new();
    /// list.push(1);
    /// list.push(2);
    /// list.push(3);
    ///
    /// assert_eq!(list.len(), 3);
    /// assert_eq!(list.get(0.0), Some(&2));
    ///
    /// list.insert(0.0, 0);
    /// assert_eq!(list.len(), 4);
    /// assert_eq!(list.get(-1.0), Some(&1));
    /// assert_eq!(list.get(0.0), Some(&0));
    /// assert_eq!(list.get(1.0), Some(&2));
    /// assert_eq!(list.get(2.0), Some(&3));
    /// ```
    pub fn insert(&mut self, index: f32, elem: T) {
        let mut count = self.len() as f32 - 2.0;
        let mut cur_link = self.head.as_deref_mut();
        if index < -1.0 {
            return;
        } else if index - count > 0.0 {
            return;
        }
        let mut index2 = index;
        if index.fract() != 0.0 {
            index2 += 1.0;
        }
        while let Some(node) = cur_link {
            if count <= index2 {
                let new_node = Box::new(Node {
                    elem,
                    next: node.next.take(),
                });
                node.next = Some(new_node);
                return;
            }
            count -= 1.0;
            cur_link = node.next.as_deref_mut();
        }
    }
}

// TODO: Below:
// Does not work, and I am really lacking the knwoldge of lifetimes and generics to make this work. I think.
/* 
impl<'a, T: Clone + Default> List<T> {
    pub fn remove(&'a mut self, index: f32) -> Option<T> {
        let mut count = self.len() as f32 - 2.0;
        let mut cur_link = self.head.as_deref_mut();
        if index < -1.0 {
            return None;
        } else if index - count > 0.0 {
            return None;
        }
        let mut new_list = List::new();
        let mut new_node: Node<T> = Default::default();
        let mut out = None;
        while let Some(node) = cur_link {
            if count <= index {
                new_list.head = node.next.take();
                out = Some(node.elem.clone());
            } else {
                new_list.push(node.elem.clone());
                new_node = Node {
                    elem: node.elem.clone(),
                    next: node.next.take(),
                };
            }
            count -= 1.0;
            cur_link = node.next.as_deref_mut();
        }
        self.head = Some(Box::new(new_node));
        out
    }
} */

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

impl<T> Index<f32> for List<T> {
    type Output = T;
    fn index(&self, index: f32) -> &Self::Output {
        &self.get(index).unwrap()
    }
}

impl<T> IndexMut<f32> for List<T> {
    fn index_mut(&mut self, index: f32) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}


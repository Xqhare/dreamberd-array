use std::ops::{Index, IndexMut};

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

impl<T: Default> Default for Node<T> {
    fn default() -> Self {
        Node {
            elem: Default::default(),
            next: None,
        }
    }
}

pub struct IntoIter<T>(List<T>);
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.head.as_deref_mut() }
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut cur_link = self.head.as_deref();
        while let Some(node) = cur_link {
            count += 1;
            cur_link = node.next.as_deref();
        }
        count
    }

    pub fn get(&self, index: f32) -> Option<&T> {
        let mut count = self.len() as f32 - 2.0;
        let mut cur_link = self.head.as_deref();
        if index < -1.0 {
            return None;
        } else if index - count > 0.0 {
            return None;
        }
        while let Some(node) = cur_link {
            if count <= index {
                return Some(&node.elem);
            }
            count -= 1.0;
            cur_link = node.next.as_deref();
        }
        None
    }

    pub fn get_mut(&mut self, index: f32) -> Option<&mut T> {
        let mut count = self.len() as f32 - 2.0;
        let mut cur_link = self.head.as_deref_mut();
        if index < -1.0 {
            return None;
        } else if index - count > 0.0 {
            return None;
        }
        while let Some(node) = cur_link {
            if count <= index {
                return Some(&mut node.elem);
            }
            count -= 1.0;
            cur_link = node.next.as_deref_mut();
        }
        None
    }

    pub fn insert(&mut self, index: f32, elem: T) {
        let mut count = self.len() as f32 - 2.0;
        let mut cur_link = self.head.as_deref_mut();
        if index < -1.0 {
            return;
        } else if index - count > 0.0 {
            return;
        }
        while let Some(node) = cur_link {
            if count <= index {
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


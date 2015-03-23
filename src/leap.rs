use errors::ArtError;
use types::ArtResult;

#[derive(Copy)]
enum Content<T> {
    Full(T),
    Empty
}

#[derive(Copy)]
struct Node<T> {
    content: Content<T>,
    next: usize
}

pub struct Leap<T> {
    nodes: Vec<Node<T>>,
    pub tail: usize,
    length: usize
}

impl<T> Leap<T> {
    pub fn new() -> Leap<T> {
        Leap {
            nodes: Vec::with_capacity(0),
            tail: 0,
            length: 0
        }
    }

    pub fn with_capacity(capacity: usize) -> Leap<T> {
        let mut nodes = Vec::with_capacity(capacity);
        for i in range(0, capacity) {
            nodes.push(Node {
                content: Content::Empty,
                next: (i + 1) % capacity
            })
        }

        Leap {
            nodes: nodes,
            tail: 0,
            length: 0
        }
    }

    pub fn push(&mut self, value: T) -> ArtResult<usize> {
        if self.length + 1 > self.nodes.len() {
            return Err(
                ArtError::BufferOverflow
            );
        }

        let tail = self.tail;
        self.tail = self.set(tail, value);
        Ok(self.tail)
    }

    pub fn set(&mut self, index: usize, value: T) -> usize {
        let node = &mut self.nodes[index];
        node.content = Content::Full(value);
        node.next
    }

    pub fn free(&mut self, mut index: usize, count: usize) {
        let old_tail = self.tail;
        self.tail = index;
        for i in range(0, count) {
            let node = &mut self.nodes[index];
            node.content = Content::Empty;
            index = node.next;

            if i == count - 1 {
                node.next = old_tail;
            }
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn capacity(&self) -> usize {
        self.nodes.capacity()
    }

    pub fn iter(&self, index: usize) -> Iter<T> {
        Iter {
            nodes: &self.nodes,
            index: index
        }
    }

    pub fn iter_mut(&mut self, index: usize) -> IterMut<T> {
        IterMut {
            nodes: &mut self.nodes,
            index: index
        }
    }
}

pub struct Iter<'a, T: 'a> {
    index: usize,
    nodes: &'a [Node<T>]
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        let node = &self.nodes[self.index];
        self.index = node.next;

        match node.content {
            Content::Full(ref value) => Some(value),
            Content::Empty => None
        }
    }
}

pub struct IterMut<'a, T: 'a> {
    index: usize,
    nodes: &'a mut [Node<T>]
}

impl<'a, T: 'a> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<&'a mut T> {
        let node = &mut self.nodes[self.index];
        self.index = node.next;

        match node.content {
            Content::Full(ref mut value) => unsafe {
                Some(&mut *(value as *mut _))
            },
            Content::Empty => None
        }
    }
}

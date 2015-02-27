use errors::ArtError;
use types::ArtResult;

#[derive(Copy)]
enum Content<T> {
    Start,
    Full(T),
    Empty
}

impl<T> Content<T> {
    pub fn is_start(&self) -> bool {
        match *self {
            Content::Start => true,
            _ => false
        }
    }

    pub fn is_full(&self) -> bool {
        match *self {
            Content::Full(_) => true,
            _ => false
        }
    }

    pub fn is_empty(&self) -> bool {
        match *self {
            Content::Empty => true,
            _ => false
        }
    }
}

#[derive(Copy)]
struct Node<T> {
    content: Content<T>,
    next: usize
}

pub struct Leap<T> {
    nodes: Vec<Node<T>>,
    tail: usize,
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

    pub fn alloc(&mut self, size: usize) -> ArtResult<usize> {
        if self.length + size + 1 > self.nodes.len() {
            return Err(
                ArtError::BufferOverflow
            );
        }

        let index = try!(self.do_push(Content::Start));
        Ok(index)
    }

    pub fn free(&mut self, mut index: usize) -> ArtResult<()> {
        let mut start = true;
        let mut last_index = index;
        let old_tail = self.tail;
        loop {
            let node = &mut self.nodes[index];
            if start {
                if !node.content.is_start() {
                    return Err(ArtError::IndexError);
                }
                self.tail = index;
            }
            else if !node.content.is_full() {
                break;
            }

            node.content = Content::Empty;
            self.length -= 1;

            last_index = index;
            index = node.next;
            start = false;
        }

        self.nodes[last_index].next = old_tail;
        Ok(())
    }

    pub fn push(&mut self, value: T) -> ArtResult<usize> {
        if self.length == self.nodes.len() {
            return Err(
                ArtError::BufferOverflow
            );
        }
        self.do_push(Content::Full(value))
    }


    fn do_push(&mut self, content: Content<T>) -> ArtResult<usize> {
        let index = self.tail;
        let node = &mut self.nodes[index];
        if !node.content.is_empty() {
            return Err(
                ArtError::BufferOverflow
            );
        }
        node.content = content;
        self.tail = node.next;
        self.length += 1;
        Ok((index))
    }

    pub fn iter(&self, index: usize) -> ArtResult<Iter<T>> {
        let node = &self.nodes[index];
        match &node.content {
            &Content::Start => Ok(
                Iter {
                    nodes: &self.nodes,
                    index: node.next
                }
            ),
            _ => Err(
                ArtError::IndexError
            )
        }
    }
}

pub struct Iter<'a, T: 'a> {
    index: usize,
    nodes: &'a Vec<Node<T>>
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        let node = &self.nodes[self.index];
        self.index = node.next;

        match node.content {
            Content::Full(ref value) => Some(value),
            _ => None
        }
    }
}

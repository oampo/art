use errors::ArtError;
use types::ArtResult;

use opcode::DspOpcode;

enum Content {
    Opcode(DspOpcode),
    Start,
    Empty
}

impl Content {
    pub fn is_opcode(&self) -> bool {
        match *self {
            Content::Opcode(_) => true,
            _ => false
        }
    }

    pub fn is_start(&self) -> bool {
        match *self {
            Content::Start => true,
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

struct Node {
    content: Content,
    next: usize
}

pub struct ExpressionStore {
    nodes: Vec<Node>,
    tail: usize,
    length: usize
}

impl ExpressionStore {
    pub fn new() -> ExpressionStore {
        ExpressionStore {
            nodes: Vec::with_capacity(0),
            tail: 0,
            length: 0
        }
    }

    pub fn with_capacity(capacity: usize) -> ExpressionStore {
        let mut nodes = Vec::with_capacity(capacity);
        for i in range(0, capacity) {
            nodes.push(Node {
                content: Content::Empty,
                next: (i + 1) % capacity
            })
        }

        ExpressionStore {
            nodes: nodes,
            tail: 0,
            length: 0
        }
    }

    pub fn push_start(&mut self, num_opcodes: usize) -> ArtResult<usize> {
        if self.length + num_opcodes + 1 > self.nodes.len() {
            return Err(
                ArtError::BufferOverflow
            );
        }

        self.do_push(Content::Start);
        Ok(self.tail)
    }

    pub fn push(&mut self, opcode: DspOpcode) -> ArtResult<usize> {
        if self.length == self.nodes.len() {
            return Err(
                ArtError::BufferOverflow
            );
        }
        self.do_push(Content::Opcode(opcode));
        Ok(self.tail)
    }

    fn do_push(&mut self, content: Content) {
        let next = self.nodes[self.tail].next;
        let node = &mut self.nodes[next];
        assert!(node.content.is_empty());
        node.content = content;
        self.tail = next;
        self.length += 1;
    }

    pub fn remove(&mut self, mut index: usize) -> ArtResult<()> {
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
            else if !node.content.is_opcode() {
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

    pub fn iter(&self, index: usize) -> ArtResult<Iter> {
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

pub struct Iter<'a> {
    index: usize,
    nodes: &'a Vec<Node>
}

impl<'a> Iterator for Iter<'a> {
    type Item = DspOpcode;
    fn next(&mut self) -> Option<DspOpcode> {
        let node = &self.nodes[self.index];
        self.index = node.next;

        match &node.content {
            &Content::Opcode(ref opcode) => Some(*opcode),
            _ => None
        }
    }
}

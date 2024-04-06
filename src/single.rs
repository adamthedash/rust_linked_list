use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::mem::{replace, take};

pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}


impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.next {
            None => { write!(f, "{}", self.value) }
            Some(next) => { write!(f, "{} -> {}", self.value, next) }
        }
    }
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.head {
            None => { write!(f, "[]") }
            Some(head) => { write!(f, "[ {} ]", head) }
        }
    }
}


impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    fn get_node(&self, index: usize) -> Result<&Node<T>, ()> {
        let mut cur = &self.head;

        for _ in 0..index {
            match cur {
                None => return Err(()),
                Some(c) => cur = &c.next
            }
        }

        match cur {
            None => Err(()),
            Some(c) => Ok(c)
        }
    }

    fn get_node_mut(&mut self, index: usize) -> Result<&mut Box<Node<T>>, ()> {
        let mut cur = &mut self.head;
        for _ in 0..index {
            match cur {
                None => return Err(()),
                Some(c) => cur = &mut c.next
            }
        }


        match cur {
            None => Err(()),
            Some(c) => Ok(c)
        }
    }

    pub fn get_value(&self, index: usize) -> Result<&T, ()> {
        let val = self.get_node(index)?;
        return Ok(&val.value);
    }

    pub fn len(&self) -> usize {
        let mut cur = &self.head;

        let mut i = 0;
        loop {
            match cur {
                None => return i,
                Some(c) => {
                    i += 1;
                    cur = &c.next;
                }
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn append(&mut self, item: T) {
        self.insert(item, self.len()).unwrap()
    }

    pub fn insert(&mut self, item: T, index: usize) -> Result<(), ()> {
        // Get preceeding item handle - either valid node, or head
        let preceeding = match index {
            0 => &mut self.head,
            _ => {
                &mut self.get_node_mut(index - 1)?.next
            }
        };

        // Box up value as node
        let new_node = Node {
            value: item,
            next: None,
        };

        // Swap in new item
        let tail = replace(preceeding, Some(Box::new(new_node)));

        // Add tail back on
        self.get_node_mut(index)?.next = tail;

        Ok(())
    }

    pub fn pop(&mut self, index: usize) -> Result<T, ()> {
        // Get preceeding item handle - either valid node, or head
        let preceeding = match index {
            0 => &mut self.head,
            _ => {
                &mut self.get_node_mut(index - 1)?.next
            }
        };
        if preceeding.is_none() {
            // Out of bounds error
            return Err(());
        }

        // Chop off current + tail
        let mut current = take(preceeding).unwrap();

        // Chop off tail
        let tail = take(&mut current.next);

        // Put tail back on
        *preceeding = tail;


        // Return value
        Ok(current.value)
    }

    pub fn swap(&mut self, index1: usize, index2: usize) -> Result<(), ()> {
        assert_ne!(index1, index2);

        // Sort indices
        let [index2, index1] = match index1.cmp(&index2) {
            Ordering::Greater => [index1, index2],
            _ => [index2, index1]
        };

        // Pop elements
        let val1 = self.pop(index1)?;
        let val2 = self.pop(index2 - 1)?;

        // Insert elements
        self.insert(val2, index1)?;
        self.insert(val1, index2)?;

        Ok(())
    }
}


impl<T: PartialOrd> LinkedList<T> {
    pub fn sort(&mut self) {
        todo!()
    }
}
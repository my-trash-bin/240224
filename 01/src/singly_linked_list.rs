struct SinglyLinkedListNode<T> {
    data: T,
    next: Option<Box<SinglyLinkedListNode<T>>>,
}

pub struct SinglyLinkedList<T> {
    head: Option<Box<SinglyLinkedListNode<T>>>,
}

impl<T> SinglyLinkedList<T> {
    pub fn new() -> SinglyLinkedList<T> {
        SinglyLinkedList { head: None }
    }

    pub fn nth(&self, n: usize) -> Option<&T> {
        let mut node = &self.head;
        for _ in 0..n {
            if let Some(current) = node {
                node = &current.next;
            } else {
                return None;
            }
        }
        if let Some(current) = node {
            Some(&current.data)
        } else {
            None
        }
    }

    pub fn push(&mut self, data: T) -> () {
        let mut tail = &mut self.head;
        while let Some(node) = tail {
            tail = &mut node.next;
        }
        *tail = Some(Box::new(SinglyLinkedListNode { data, next: None }))
    }
}

use std::ptr;

// pub struct List<'a, T> {
pub struct List<T> {
    head: Link<T>,
    // tail: Option<&'a mut Node<T>>,
    tail: *mut Node<T>, // DANGER
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

// impl<'a, T> List<'a, T> {
impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            // tail: None,
            tail: ptr::null_mut(),
        }
    }

    // pub fn push(&'a mut self, elem: T) {
    pub fn push(&mut self, elem: T) {
        // let new_tail = Box::new(Node { elem, next: None });
        // let new_tail = match self.tail.take() {
        //     Some(mut old_tail) => {
        //         old_tail.next = Some(new_tail);
        //         old_tail.next.as_deref_mut()
        //     }
        //     None => {
        //         self.head = Some(new_tail);
        //         self.head.as_deref_mut()
        //     }
        // };
        // self.tail = new_tail;

        let mut new_tail = Box::new(Node { elem, next: None });

        let raw_tail: *mut _ = &mut *new_tail;
        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        } else {
            self.head = Some(new_tail);
        }
        self.tail = raw_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;
            if self.head.is_none() {
                // self.tail = None;
                self.tail = ptr::null_mut();
            }
            head.elem
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check noraml removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        // Check the exhaustion case fixed the pointer right
        list.push(6);
        list.push(7);

        // Check normal removal
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }
}

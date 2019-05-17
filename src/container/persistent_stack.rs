use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
} 

pub struct PersistentStack<T> {
    head: Link<T>,
}

impl<T> PersistentStack<T> {
    pub fn new() -> Self { Self { head: None } }
    pub fn push(&self, elem: T) -> Self {
        Self {
            head: Some(Rc::new( Node {
                elem: elem, 
                next: self.head.clone(),
            } ))
        }
    }
    pub fn pop(&self) -> Self {
        Self {
            head: self.head.as_ref().and_then(|node| node.next.clone())
        }
    }
    pub fn top(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }
}

impl<T> Drop for PersistentStack<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            }
            else { break; }
        }
    }
}

pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(| node | {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}


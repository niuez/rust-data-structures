use std::cmp::Ordering::{Equal, Greater, Less};
use std::mem;

pub trait Monoid {
    fn ope(&self, r: &Self) -> Self;
    fn ide() -> Self;
}

pub struct SplayTree<T: Monoid> {
    root: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T: Monoid> {
    elem: T,
    size: usize,
    left: Link<T>,
    right: Link<T>,
    fold: T
}

impl<T: Monoid> Node<T> {
    fn new(elem: T) -> Node<T> {
        let mut node = Node { elem: elem, size: 1, left: None, right: None, fold: T::ide() };
        fix(&mut node);
        node
    }
}

fn size<T: Monoid>(link: & Link<T>) -> usize {
    match link {
        &Some(ref boxed_node) => boxed_node.size,
        &None => 0
    }
}

fn fix<T: Monoid>(node: &mut Node<T>) {
    let sz = size(&node.left) + size(&node.right) + 1;
    node.size = sz;
    node.fold = match node.left { Some(ref left) => left.as_ref().fold.ope(&node.elem), None => T::ide().ope(&node.elem) };
    node.fold = match node.right { Some(ref right) => node.fold.ope(&right.as_ref().fold), None => node.fold.ope(&T::ide()) };
}

fn cut_left<T: Monoid>(node: &mut Node<T>) -> Link<T> {
    let nn = node.left.take();
    fix(node);
    nn
}

fn cut_right<T: Monoid>(node: &mut Node<T>) -> Link<T> {
    let nn = node.right.take();
    fix(node);
    nn
}

fn set_left<T: Monoid>(node: &mut Node<T>, left: Link<T>) {
    //assert!(node.left.is_none(), "cant set");
    node.left = left;
    fix(node);
}

fn set_right<T: Monoid>(node: &mut Node<T>, right: Link<T>) {
    //assert!(node.right.is_none(), "cant set");
    node.right = right;
    fix(node);
}

fn fix_sub_left<T: Monoid>(n: &mut Link<T>) {
    if n.is_none() { return }
    let t = n;
    fix_sub_left(&mut t.as_mut().unwrap().right);
    {
        let no = t.as_mut().unwrap();
        fix(no);
    }
}

fn fix_sub_right<T: Monoid>(n: &mut Link<T>) {
    if n.is_none() { return }
    let t = n;
    fix_sub_right(&mut t.as_mut().unwrap().left);
    {
        let no = t.as_mut().unwrap();
        fix(no);
    }
}

fn splay<T: Monoid>(node: &mut Box<Node<T>>, n: usize) {
    let mut sz = n;
    let mut sub_left: Link<T> = None;
    let mut sub_right: Link<T> = None;
    {
        let mut le = &mut sub_left;
        let mut ri = &mut sub_right;
        loop {
            match size(&node.left).cmp(&sz) {
                Equal => { break }
                Greater => {
                    let mut left = match cut_left(node) {
                        Some(ll) => ll,
                        None => break
                    };
                    if size(&left.left).cmp(&sz) == Greater {
                        set_left(node, cut_right(&mut left));
                        mem::swap(node, &mut left);
                        let next = cut_left(node);
                        set_right(node, Some(left));
                        match next {
                            Some(l) => { left = l; },
                            None => { break }
                        }
                    }
                    *ri = Some(mem::replace(node, left));
                    let t = ri;
                    ri = &mut t.as_mut().unwrap().left;
                }
                Less => {
                    sz = sz - size(&node.left) - 1;
                    let mut right = match cut_right(node) {
                        Some(rr) => rr,
                        None => break
                    };
                    if size(&right.left).cmp(&sz) == Less {
                        sz = sz - size(&right.left) - 1;
                        set_right(node, cut_left(&mut right));
                        mem::swap(node, &mut right);
                        let next = cut_right(node);
                        set_left(node, Some(right));
                        match next {
                            Some(r) => { right = r; },
                            None => { break }
                        }
                    }
                    *le = Some(mem::replace(node, right));
                    let t = le;
                    le = &mut t.as_mut().unwrap().right;
                }
            }
        }
        mem::swap(&mut cut_left(node), le);
        mem::swap(&mut cut_right(node), ri);
    }
    fix_sub_left(&mut sub_left);
    fix_sub_right(&mut sub_right);
    set_left(node, sub_left);
    set_right(node, sub_right);
    
}

fn split<T: Monoid>(node: Link<T>, i: usize) -> (Link<T>, Link<T>) {
    //assert!(i <= size(&node), "over");
    match node {
        None => (None, None),
        Some(mut r) => {
            if i == 0 { (None, Some(r)) }
            else if i == r.size { (Some(r), None) }
            else {
                splay(&mut r , i - 1);
                let right = cut_right(&mut r);
                (Some(r), right)
            }
        }
    }
}

fn merge<T: Monoid>(left: Link<T>, right: Link<T>) -> Link<T> {
    match (left, right) {
        (Some(mut l), r) => {
            let sz = l.size;
            splay(&mut l, sz - 1);
            set_right(&mut l, r);
            Some(l)
        }
        (None, r) => {
            r
        }
    }
}

impl<T: Monoid> SplayTree<T>{
    pub fn new() -> Self {
        SplayTree { root: None, }
    }

    pub fn at(&mut self, i: usize) -> &T {
        //assert!(!self.root.is_none(), "what at");
        splay(self.root.as_mut().unwrap(), i);
        &self.root.as_ref().unwrap().elem
    }

    pub fn at_mut(&mut self, i: usize) -> &mut T {
        //assert!(!self.root.is_none(), "what at");
        splay(self.root.as_mut().unwrap(), i);
        &mut self.root.as_mut().unwrap().elem
    }

    pub fn set(&mut self, i: usize, elem: T) {
        splay(self.root.as_mut().unwrap(), i);
        self.root.as_mut().unwrap().elem = self.root.as_ref().unwrap().elem.ope(&elem);
        fix(self.root.as_mut().unwrap());
    }

    pub fn split(self, i: usize) -> (Self, Self) {
        let (l, r) = split(self.root, i);
        (SplayTree { root: l }, SplayTree { root: r })
    }

    pub fn merge(&mut self, mut right: Self) {
        self.root = merge(self.root.take(), right.root.take());
    }

    pub fn fold(&self) -> &T {
        &self.root.as_ref().unwrap().fold
    }

    pub fn insert(&mut self, i: usize, elem: T) {
        let mut node = Box::new(Node::new(elem));
        fix(&mut node);
        let (left, right) = split(self.root.take(), i);
        set_left(&mut node, left);
        set_right(&mut node, right);
        self.root = Some(node);
    }

    pub fn erase(&mut self, i: usize) {
        let (ll, right) = split(self.root.take(), i + 1);
        let (left, _) = split(ll, i);
        self.root = merge(left, right);
    }
}

#[cfg(test)]
mod test {
}

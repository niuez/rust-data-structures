use std::cmp::Ordering::{ Equal, Greater, Less };
use std::mem;
use std::ptr::{ NonNull, self };
use std::alloc::{ Layout, alloc };

pub trait Monoid {
    fn ope(&self, r: &Self) -> Self;
    fn ide() -> Self;
}

pub struct SplayTree<T: Monoid> {
    root: Link<T>
}

type Link<T> = Option<NonNull<Node<T>>>;

struct Node<T: Monoid> {
    elem: T,
    size: usize, 
    left: Link<T>,
    right: Link<T>,
    fold: T,
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
        &Some(ref node_ptr) => unsafe { node_ptr.as_ref().size },
        &None => 0
    }
}

fn fix<T: Monoid>(node: &mut Node<T>) {
    let sz = size(&node.left) + size(&node.right) + 1;
    node.size = sz;
    node.fold = match node.left { Some(ref left) => unsafe { left.as_ref().fold.ope(&node.elem) }, None => T::ide().ope(&node.elem) };
    node.fold = match node.right { Some(ref right) => unsafe { node.fold.ope(&right.as_ref().fold) }, None => node.fold.ope(&T::ide()) };
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
    unsafe {
        fix_sub_left(&mut t.unwrap().as_mut().right);
        {
            fix(t.unwrap().as_mut());
        }
    }
}

fn fix_sub_right<T: Monoid>(n: &mut Link<T>) {
    if n.is_none() { return }
    let t = n;
    unsafe {
        fix_sub_right(&mut t.unwrap().as_mut().left);
        {
            fix(t.unwrap().as_mut());
        }
    }
}

fn splay<T: Monoid>(node: &mut NonNull<Node<T>>, n: usize) {
    unsafe {
        let mut sz = n;
        let mut sub_left: Link<T> = None;
        let mut sub_right: Link<T> = None;
        {
            let mut le = &mut sub_left;
            let mut ri = &mut sub_right;
            loop {
                match size(&node.as_ref().left).cmp(&sz) {
                    Equal => { break }
                    Greater => {
                        let mut left = match cut_left(node.as_mut()) {
                            Some(ll) => ll,
                            None => break
                        };
                        if size(&left.as_ref().left).cmp(&sz) == Greater {
                            set_left(node.as_mut(), cut_right(left.as_mut()));
                            mem::swap(node.as_mut(), left.as_mut());
                            let next = cut_left(node.as_mut());
                            set_right(node.as_mut(), Some(left));
                            match next {
                                Some(l) => { left = l; },
                                None => { break }
                            }
                        }
                        *ri = Some(mem::replace(node, left));
                        let t = ri;
                        ri = &mut t.as_mut().unwrap().as_mut().left;
                    }
                    Less => {
                        sz = sz - size(&node.as_ref().left) - 1;
                        let mut right = match cut_right(node.as_mut()) {
                            Some(rr) => rr,
                            None => break
                        };
                        if size(&right.as_ref().left).cmp(&sz) == Less {
                            sz = sz - size(&right.as_ref().left) - 1;
                            set_right(node.as_mut(), cut_left(right.as_mut()));
                            mem::swap(node.as_mut(), right.as_mut());
                            let next = cut_right(node.as_mut());
                            set_left(node.as_mut(), Some(right));
                            match next {
                                Some(r) => { right = r; },
                                None => { break }
                            }
                        }
                        *le = Some(mem::replace(node, right));
                        let t = le;
                        le = &mut t.as_mut().unwrap().as_mut().right;
                    }
                }
            }
            mem::swap(&mut cut_left(node.as_mut()), le);
            mem::swap(&mut cut_right(node.as_mut()), ri);
        }
        fix_sub_left(&mut sub_left);
        fix_sub_right(&mut sub_right);
        set_left(node.as_mut(), sub_left);
        set_right(node.as_mut(), sub_right);
    }
}


fn split<T: Monoid>(node: Link<T>, i: usize) -> (Link<T>, Link<T>) {
    //assert!(i <= size(&node), "over");
    match node {
        None => (None, None),
        Some(mut r) => unsafe {
            if i == 0 { (None, Some(r)) }
            else if i == r.as_ref().size { (Some(r), None) }
            else {
                splay(&mut r , i - 1);
                let right = cut_right(r.as_mut());
                (Some(r), right)
            }
        }
    }
}


fn merge<T: Monoid>(left: Link<T>, right: Link<T>) -> Link<T> {
    match (left, right) {
        (Some(mut l), r) => unsafe {
            let sz = l.as_ref().size;
            splay(&mut l, sz - 1);
            set_right(l.as_mut(), r);
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
        unsafe { &self.root.as_ref().unwrap().as_ref().elem }
    }

    pub fn at_mut(&mut self, i: usize) -> &mut T {
        //assert!(!self.root.is_none(), "what at");
        unsafe {
            splay(self.root.as_mut().unwrap(), i);
            &mut self.root.as_mut().unwrap().as_mut().elem
        }
    }

    pub fn set(&mut self, i: usize, elem: T) {
        unsafe {
            splay(self.root.as_mut().unwrap(), i);
            self.root.unwrap().as_mut().elem = self.root.unwrap().as_ref().elem.ope(&elem);
            fix(self.root.unwrap().as_mut());
        }
    }

    pub fn split(self, i: usize) -> (Self, Self) {
        let (l, r) = split(self.root, i);
        (SplayTree { root: l }, SplayTree { root: r })
    }

    pub fn merge(&mut self, mut right: Self) {
        self.root = merge(self.root.take(), right.root.take());
    }

    pub fn fold(&self) -> &T {
        unsafe { &self.root.as_ref().unwrap().as_ref().fold } 
    }

    pub fn insert(&mut self, i: usize, elem: T) {
        unsafe {
            let align = mem::align_of::<Node<T>>();
            let elem_size = mem::size_of::<Node<T>>();
            let mut node_ptr = alloc(Layout::from_size_align_unchecked(elem_size, align));
            ptr::write(node_ptr as *mut _, Node::new(elem));
            let mut node = NonNull::new_unchecked(node_ptr as *mut _);
            fix(node.as_mut());
            let (left, right) = split(self.root.take(), i);
            set_left(node.as_mut(), left);
            set_right(node.as_mut(), right);
            self.root = Some(node);
        }
    }

    pub fn erase(&mut self, i: usize) {
        let (ll, right) = split(self.root.take(), i + 1);
        let (left, _) = split(ll, i);
        self.root = merge(left, right);
    }
}

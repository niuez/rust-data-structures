use lctree::node_traits::*;

struct ValNode<T> {
    ch: [Link<Self>; 2],
    par: Link<Self>,
    val: T,
    rev: bool,
    sz: usize,
}

impl<T> Node for ValNode<T> {
    type Value = T;
    fn new(val: T) -> Self {
        ValNode {
            ch: [ None, None ],
            par: None,
            val: val,
            rev: false,
            sz: 1
        }
    }
    fn push(&mut self) {
        if self.rev {
            if let Some(mut left) = self.ch[0] {
                unsafe { left.as_mut().reverse(); }
            }
            if let Some(mut right) = self.ch[1] {
                unsafe { right.as_mut().reverse(); }
            }
            self.rev = false;
        }
    }
    fn reverse(&mut self) {
        self.ch.swap(0, 1);
        self.rev ^= true;
    }
    fn child(&self, dir: usize) -> &Link<Self> {
        &self.ch[dir]
    }
    fn child_mut(&mut self, dir: usize) -> &mut Link<Self> {
        &mut self.ch[dir]
    }
    fn parent(&self) -> &Link<Self> {
        &self.par
    }
    fn parent_mut(&mut self) -> &mut Link<Self> {
        &mut self.par
    }
    fn fix(&mut self) {
        self.sz = 1;
        unsafe { 
            if let Some(left) = self.ch[0] {
                self.sz += left.as_ref().sz;
            }
            if let Some(right) = self.ch[1] {
                self.sz += right.as_ref().sz;
            }
        }
    }
    fn value(&self) -> &T { &self.val }
    fn value_mut(&mut self) -> &mut T { &mut self.val }
    fn size(&self) -> usize { self.sz }
}

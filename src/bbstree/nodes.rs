use algebra::*;
use bbstree::node_traits::*;
use std::cmp::max;

pub struct ArrNode<T> {
    val: T,
    size: usize,
    height: isize,
    child: [Link<ArrNode<T>>; 2]
} 

impl<T> ArrNode<T> {
    pub fn new(val: T) -> Self {
        Self {
            val: val,
            size: 1,
            height: 1,
            child: [ None, None ],
        }
    }
}

impl<T> Node for ArrNode<T> {
    type Value = T;
    fn fix(&mut self) {
        self.size = size(&self.child[0]) + size(&self.child[1]) + 1;
        self.height = max(height(&self.child[0]), height(&self.child[1])) + 1;
    }
    fn child(&mut self, dir: usize) -> &mut Link<Self> { &mut self.child[dir] } 
    fn child_imut(&self, dir: usize) -> &Link<Self> { &self.child[dir] } 
    fn cut(&mut self, dir: usize) -> Link<Self> {
        let nn = self.child[dir].take();
        self.fix();
        nn
    }
    fn set(&mut self, dir_node: Link<Self>, dir: usize) {
        self.child[dir] = dir_node;
        self.fix();
    }
    fn val(&self) -> &Self::Value { &self.val }
    fn val_mut(&mut self) -> &mut Self::Value { &mut self.val }
}

impl<T> ArrayNode for ArrNode<T> {
    fn size(&self) -> usize { self.size }
}

impl<T> AVLNode for ArrNode<T> {
    fn height(&self) -> isize { self.height }
}


pub struct ArrFoldNode<T: Monoid> {
    val: T,
    fold: T,
    size: usize,
    height: isize,
    child: [Link<ArrFoldNode<T>>; 2]
} 

impl<T: Monoid> ArrFoldNode<T> {
    pub fn new(val: T) -> Self {
        Self {
            val: val,
            fold: T::identity(),
            size: 1,
            height: 1,
            child: [ None, None ],
        }
    }
}

impl<T: Monoid> Node for ArrFoldNode<T> {
    type Value = T;
    fn fix(&mut self) {
        self.size = size(&self.child[0]) + size(&self.child[1]) + 1;
        self.height = max(height(&self.child[0]), height(&self.child[1])) + 1;
        let lf = match self.child[0] { Some(ref node) => node.fold().clone(), None => T::identity() };
        let rf = match self.child[1] { Some(ref node) => node.fold().clone(), None => T::identity() };
        self.fold = lf.op(&self.val).op(&rf);
    }
    fn child(&mut self, dir: usize) -> &mut Link<Self> { &mut self.child[dir] } 
    fn child_imut(&self, dir: usize) -> &Link<Self> { &self.child[dir] } 
    fn cut(&mut self, dir: usize) -> Link<Self> {
        let nn = self.child[dir].take();
        self.fix();
        nn
    }
    fn set(&mut self, dir_node: Link<Self>, dir: usize) {
        self.child[dir] = dir_node;
        self.fix();
    }
    fn val(&self) -> &Self::Value { &self.val }
    fn val_mut(&mut self) -> &mut Self::Value { &mut self.val }
}

impl<T: Monoid> ArrayNode for ArrFoldNode<T> {
    fn size(&self) -> usize { self.size }
}

impl<T: Monoid> AVLNode for ArrFoldNode<T> {
    fn height(&self) -> isize { self.height }
}

impl<T: Monoid> FoldNode for ArrFoldNode<T> {
    fn fold(&self) -> &T { &self.fold } 
}

use algebra::Monoid;
use std::cmp::max;

pub type Link<N> = Option<Box<N>>;

pub trait Fix: Sized {
    fn fix(&mut self, _left: Option<&Self>, _right: Option<&Self>) {}
}

pub trait Value: Fix {
    type Type;
    fn new(val: Self::Type) -> Self;
    fn val(&self) -> &Self::Type;
    fn val_mut(&mut self) -> &mut Self::Type;
}

pub trait Foldable: Value where Self::Type: Monoid {
    fn fold(&self) -> &Self::Type;
}

pub struct Element<T>(T);

impl<T> Fix for Element<T> {}

impl<T> Value for Element<T> {
    type Type = T;
    fn new(val: Self::Type) -> Self { Element(val) }
    fn val(&self) -> &Self::Type { &self.0 }
    fn val_mut(&mut self) -> &mut Self::Type { &mut self.0 }
}

pub struct FoldElement<M: Monoid>(M, M);

impl<M: Monoid> Fix for FoldElement<M> {
    fn fix(&mut self, left: Option<&Self>, right: Option<&Self>) {
        self.1 = match left { Some(e) => e.1.clone(), None => M::identity() }
        .op(&self.0)
        .op(& match right { Some(e) => e.1.clone(), None => M::identity() });
    }
}

impl<M: Monoid> Value for FoldElement<M> {
    type Type = M;
    fn new(val: Self::Type) -> Self { FoldElement(val, M::identity()) }
    fn val(&self) -> &Self::Type { &self.0 }
    fn val_mut(&mut self) -> &mut Self::Type { &mut self.0 }
}

impl<M: Monoid> Foldable for FoldElement<M> {
    fn fold(&self) -> &Self::Type { &self.1 }
}



pub trait Data: Fix {
    fn new() -> Self;
}

pub trait NodeSize: Data {
    fn size(&self) -> usize;
}

pub trait NodeHeight: Data {
    fn height(&self) -> isize;
}

pub struct Size(usize);
impl Fix for Size {
    fn fix(&mut self, left: Option<&Self>, right: Option<&Self>) {
        self.0 = match left { Some(s) => s.0, None => 0 } + 
                 match right { Some(s) => s.0 , None => 0} + 1;
    }
}
impl Data for Size { fn new() -> Self { Size(1) } }
impl NodeSize for Size { fn size(&self) -> usize { self.0 } }

pub struct Height(isize);
impl Fix for Height {
    fn fix(&mut self, left: Option<&Self>, right: Option<&Self>) {
        self.0 = max (match left { Some(s) => s.0, None => 0 },
                 match right { Some(s) => s.0 , None => 0}) + 1;
    }
}

impl Data for Height { fn new() -> Self { Height(1) } }
impl NodeHeight for Height { fn height(&self) -> isize { self.0 } }

impl<F1: Fix, F2: Fix> Fix for (F1, F2) {
    fn fix(&mut self, left: Option<&Self>, right: Option<&Self>) {
        self.0.fix(left.map(|s| &s.0), right.map(|s| &s.0));
        self.1.fix(left.map(|s| &s.1), right.map(|s| &s.1));
    }
}

impl<F1: Data, F2: Data> Data for (F1, F2) {
    fn new() -> Self { (F1::new(), F2::new()) }
}


pub trait Node: Sized {
    type Value;
    fn fix(&mut self);
    fn child(&mut self, dir: usize) -> &mut Link<Self>;
    fn child_imut(&self, dir: usize) -> &Link<Self>;
    fn cut(&mut self, dir: usize) -> Link<Self>;
    fn set(&mut self, dir_node: Link<Self>, dir: usize);
    fn val(&self) -> &Self::Value;
    fn val_mut(&mut self) -> &mut Self::Value;
}

pub trait ArrayNode: Node {
    fn size(&self) -> usize;
}

pub trait MapNode: Node {
    type Key;
}

pub trait AVLNode: Node {
    fn height(&self) -> isize;
    fn diff(&self) -> isize { height(self.child_imut(0)) - height(self.child_imut(1)) }
}

pub trait FoldNode where Self: Node, <Self as Node>::Value: Monoid {
    fn fold(&self) -> &<Self as Node>::Value;
}

pub fn size<N: ArrayNode>(link: &Link<N>) -> usize {
    match link {
        &Some(ref node) => node.size(),
        &None => 0,
    }
}

pub fn height<N: AVLNode>(link: &Link<N>) -> isize {
    match link {
        &Some(ref node) => node.height(),
        &None => 0,
    }
}

pub fn diff<N: AVLNode>(link: &Link<N>) -> isize {
    match link {
        &Some(ref node) => node.diff(),
        &None => 0
    }
}

pub fn cut<N: Node>(mut node: Box<N>, dir: usize) -> (Box<N>, Link<N>) {
    let nn = node.cut(dir);
    (node, nn)
}

pub fn set<N: Node>(mut node: Box<N>, dir_node: Link<N>, dir: usize) -> Box<N> {
    node.set(dir_node, dir);
    node
}

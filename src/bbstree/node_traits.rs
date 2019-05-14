use algebra::*;

pub type Link<N> = Option<Box<N>>;

pub trait Fix: Sized {
    fn fix(&mut self) {}
}

pub trait Value: Sized {
    type ValType;
    fn val(&self) -> &Self::ValType;
    fn val_mut(&mut self) -> &mut Self::ValType;
}

pub trait Foldable: Value where Self::ValType: Monoid {
    fn fold(&self) -> &Self::ValType;
}

pub trait Size: Sized {
    fn size(&self) -> usize;
}

pub trait Height: Sized {
    fn height(&self) -> isize;
}

pub trait Node: Sized + Fix {
    fn child(&mut self, dir: usize) -> &mut Link<Self>;
    fn child_imut(&self, dir: usize) -> &Link<Self>;
    fn cut(&mut self, dir: usize) -> Link<Self> {
        let nn = self.child(dir).take();
        self.fix();
        nn
    }
    fn set(&mut self, dir_node: Link<Self>, dir: usize) {
        *self.child(dir) = dir_node;
        self.fix();
    }
}

pub trait NodeValue: Node {
    type ValType;
    fn val(&self) -> &Self::ValType;
    fn val_mut(&mut self) -> &mut Self::ValType;
}

pub trait NodeSize: Node {
    fn size(&self) -> usize;
}

pub trait NodeHeight: Node {
    fn height(&self) -> isize;
    fn diff(&self) -> isize { height(self.child_imut(0)) - height(self.child_imut(1)) }
}

pub trait NodeFoldable where Self: NodeValue, Self::ValType: Monoid {
    fn fold(&self) -> &Self::ValType;
}

pub fn size<N: NodeSize>(link: &Link<N>) -> usize {
    match link {
        &Some(ref node) => node.size(),
        &None => 0,
    }
}

pub fn height<N: NodeHeight>(link: &Link<N>) -> isize {
    match link {
        &Some(ref node) => node.height(),
        &None => 0,
    }
}

pub fn diff<N: NodeHeight>(link: &Link<N>) -> isize {
    match link {
        &Some(ref node) => node.diff(),
        &None => 0
    }
}

pub fn fold<N: NodeFoldable>(link: &Link<N>) -> N::ValType where N::ValType: Monoid {
    match link {
        &Some(ref node) => node.fold().clone(),
        &None => N::ValType::identity()
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

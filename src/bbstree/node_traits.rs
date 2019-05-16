use algebra::Monoid;

pub type Link<N> = Option<Box<N>>;

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

pub trait SizeNode: Node {
    fn size(&self) -> usize;
}

pub trait MapNode: Node {
    type Key;
}

pub trait HeightNode: Node {
    fn height(&self) -> isize;
    fn diff(&self) -> isize { height(self.child_imut(0)) - height(self.child_imut(1)) }
}

pub trait FoldNode where Self: Node, <Self as Node>::Value: Monoid {
    fn fold(&self) -> &<Self as Node>::Value;
}

pub fn size<N: SizeNode>(link: &Link<N>) -> usize {
    match link {
        &Some(ref node) => node.size(),
        &None => 0,
    }
}

pub fn height<N: HeightNode>(link: &Link<N>) -> isize {
    match link {
        &Some(ref node) => node.height(),
        &None => 0,
    }
}

pub fn diff<N: HeightNode>(link: &Link<N>) -> isize {
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

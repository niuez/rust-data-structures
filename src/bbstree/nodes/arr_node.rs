use bbstree::node_traits::*;
use algebra::*;

pub struct ArrNode<D> {
    pub data: D,
    child: [Link<ArrNode<D>>; 2],
} 

impl<D> ArrNode<D> {
    pub fn new(data: D) -> Self {
        Self {
            data: data,
            child: [ None, None ],
        }
    }
}

impl<D> Node for ArrNode<D> where Self: Fix {
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
}

impl<D: Value> NodeValue for ArrNode<D> where Self: Node {
    type ValType = D::ValType;
    fn val(&self) -> &Self::ValType { self.data.val() }
    fn val_mut(&mut self) -> &mut Self::ValType { self.data.val_mut() }
}

impl<D: Size> NodeSize for ArrNode<D> where Self: Node {
    fn size(&self) -> usize { self.data.size() }
}

impl<D: Height> NodeHeight for ArrNode<D> where Self: Node {
    fn height(&self) -> isize { self.data.height() }
}

impl<D: Foldable> NodeFoldable for ArrNode<D> where D::ValType: Monoid, Self: Node {
    fn fold(&self) -> &D::ValType { self.data.fold() }
}

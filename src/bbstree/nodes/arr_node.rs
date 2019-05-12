use bbstree::node_traits::*;
use algebra::*;

pub struct ArrNode<D: Data, V: Value> {
    data: D,
    val: V,
    child: [Link<ArrNode<D, V>>; 2],
} 

impl<D: Data, V: Value> ArrNode<D, V> {
    pub fn new(val: V) -> Self {
        Self {
            data: D::new(),
            val: val,
            child: [ None, None ],
        }
    }
}

impl<D: Data, V: Value> Node for ArrNode<D, V> {
    type Value = V::Type;
    fn fix(&mut self) {
        self.data.fix(self.child[0].as_ref().map(|c| &c.data), self.child[1].as_ref().map(|c| &c.data));
        self.val.fix(self.child[0].as_ref().map(|c| &c.val), self.child[1].as_ref().map(|c| &c.val));
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
    fn val(&self) -> &Self::Value { self.val.val() }
    fn val_mut(&mut self) -> &mut Self::Value { self.val.val_mut() }
}

impl<D: NodeSize, V: Value> ArrayNode for ArrNode<D, V> {
    fn size(&self) -> usize { self.data.size() }
}

impl<D: NodeHeight, V: Value> AVLNode for ArrNode<D, V> {
    fn height(&self) -> isize { self.data.height() }
}

impl<D: Data, V: Foldable> FoldNode for ArrNode<D, V> where V::Type: Monoid {
    fn fold(&self) -> &V::Type { self.val.fold() }
}

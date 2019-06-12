use std::ptr::NonNull;
use lctree::node_traits::*;
use algebra::*;

#[derive(Clone, Copy)]
pub struct LctNode<N: Node> {
    node: NonNull<N>
}

impl<N: Node> LctNode<N> {
    pub fn new(val: N::Value) -> Self {
        unsafe {
            LctNode {
                node: NonNull::new_unchecked(Box::into_raw(Box::new(Node::new(val))))
            }
        }
    }
    pub fn link(&self, parent: &Self) { lct_link(self.node, parent.node); }
    pub fn cut(&self) { lct_cut(self.node); }
    pub fn evert(&self) { lct_evert(self.node); }
    pub fn value(&self) -> &N::Value { unsafe { self.node.as_ref().value() } }
    pub fn value_mut(&mut self) -> &mut N::Value {
        expose(self.node);
        unsafe { self.node.as_mut().value_mut() }
    }
    pub fn lca(&self, v: &Self) -> Option<Self> { lct_lca(self.node, v.node).map(|lca| LctNode { node: lca }) }
}

impl<N: EffectNode> LctNode<N> where N::Value: Monoid {
    pub fn effect(&mut self, e: N::Effector) { lct_effect(self.node, e); }
    pub fn fold(&self) -> &N::Value {
        expose(self.node);
        unsafe { self.node.as_ref().fold() }
    }
}

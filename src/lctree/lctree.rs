use std::ptr::NonNull;
use lctree::node_traits::*;
use algebra::*;

#[derive(Clone, Copy)]
pub struct LctNode<N: Node> {
    node: NonNull<N>
}

impl<N: Node> PartialEq for LctNode<N> {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
    }
}

impl<N: Node> Eq for LctNode<N> {}

impl<N: Node> LctNode<N> {
    pub fn new(val: N::Value) -> Self {
        unsafe {
            LctNode {
                node: NonNull::new_unchecked(Box::into_raw(Box::new(Node::new(val))))
            }
        }
    }
    pub fn link(&self, parent: &Self) { lct_link(parent.node, self.node); }
    pub fn cut(&self) { lct_cut(self.node); }
    pub fn evert(&self) { lct_evert(self.node); }
    pub fn value(&self) -> &N::Value {
        expose(self.node);
        unsafe { self.node.as_ref().value() }
    }
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

#[cfg(test)]
mod lct_test {
    use lctree::lctree::LctNode;
    use lctree::valnode::ValNode;
    use lctree::effectnode::EffNode;
    use algebra::*;

    #[test]
    fn lca_test() {
        let n = 8;
        let links = [
            vec![1, 2, 3],
            vec![4, 5],
            vec![],
            vec![],
            vec![],
            vec![6, 7],
            vec![],
            vec![],
        ];
        let query = [
            (4, 6, 1),
            (4, 7, 1),
            (4, 3, 0),
            (5, 2, 0),
        ];
        let nodes: Vec<_> = (0..n).map(|i| LctNode::<ValNode<usize>>::new(i)).collect();
        for i in 0..n {
            for v in links[i].iter() {
                nodes[*v].link(&nodes[i]);
            }
        }
        for (u, v, ans) in query.iter() {
            assert_eq!(nodes[*u].lca(&nodes[*v]).unwrap().value(), nodes[*ans].value());
        }
    }

    #[derive(Clone, Debug)]
    struct Sm(usize);

    impl Magma for Sm {
        fn op(&self, right: &Self) -> Self { Sm(self.0 + right.0) }
    }
    impl Associative for Sm {}
    impl Unital for Sm {
        fn identity() -> Self { Sm(0) }
    }
    impl Reversible for Sm {}

    #[derive(Clone, Debug)]
    struct Aq(usize);

    impl Magma for Aq {
        fn op(&self, right: &Self) -> Self {
            Aq(self.0 + right.0)
        }
    }
    impl Associative for Aq {}
    impl Unital for Aq {
        fn identity() -> Self { Aq(0) }
    }
    impl Effector for Aq {
        type Target = Sm;
        fn effect(&self, t: &Self::Target, s: usize) -> Self::Target {
            Sm(t.0 + self.0 * s)
        }
    }
    #[derive(Clone, Copy)]
    enum Query {
        Update(usize, usize),
        Get(usize, usize),
    }

    #[test]
    fn test_path_query_1() {
        let n = 6;
        let links = [
            vec![1, 2],
            vec![3, 5],
            vec![],
            vec![],
            vec![],
            vec![4],
        ];
        let query = [
            Query::Get(1, 0),
            Query::Update(3, 10),
            Query::Get(2, 0),
            Query::Update(4, 20),
            Query::Get(3, 10),
            Query::Update(5, 40),
            Query::Get(4, 60),
        ];
        let mut nodes: Vec<_> = (0..n).map(|_| LctNode::<EffNode<Sm, Aq>>::new(Sm::identity())).collect();
        for i in 0..n {
            for v in links[i].iter() {
                nodes[*v].link(&nodes[i]);
            }
        }
        for q in query.iter() {
            match *q {
                Query::Update(v, w) => {
                    let val = nodes[v].value().clone();
                    *nodes[v].value_mut() = val.op(&Sm(w));
                }
                Query::Get(v, ans) => {
                    assert_eq!(nodes[v].fold().0, ans);
                }
            }
        }
    }

    #[test]
    fn test_path_query_2() {
        let n = 6;
        let links = [
            vec![1, 2],
            vec![3, 5],
            vec![],
            vec![],
            vec![],
            vec![4],
        ];
        let query = [
            Query::Get(1, 0),
            Query::Update(3, 10),
            Query::Get(2, 0),
            Query::Update(4, 20),
            Query::Get(3, 40),
            Query::Update(5, 40),
            Query::Get(4, 150),
        ];
        let mut nodes: Vec<_> = (0..n).map(|_| LctNode::<EffNode<Sm, Aq>>::new(Sm::identity())).collect();
        for i in 0..n {
            for v in links[i].iter() {
                nodes[*v].link(&nodes[i]);
            }
        }
        for q in query.iter() {
            match *q {
                Query::Update(v, w) => {
                    nodes[v].effect(Aq(w));
                }
                Query::Get(v, ans) => {
                    assert_eq!(nodes[v].fold().0 - nodes[0].value().0, ans);
                }
            }
        }
    }
}

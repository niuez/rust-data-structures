use bbstree::node_traits::*;
use algebra::{ Monoid, Unital };

pub trait AVLArrayNode: Node + NodeSize + NodeHeight {}

impl<N: Node + NodeSize + NodeHeight> AVLArrayNode for N {}

fn rotate<N: AVLArrayNode>(x: Box<N>, dir: usize) -> Box<N> {
    let (x, y) = cut(x, 1 - dir);
    let (y, b) = cut(y.unwrap(), dir);
    let x = set(x, b, 1 - dir);
    set(y, Some(x), dir)
}

fn balance<N: AVLArrayNode>(mut node: Box<N>) -> Box<N> {
    if node.diff() == 2 {
        if diff(node.child(0)) == -1 {
            let (n, ch) = cut(node, 0);
            node = set(n, Some(rotate(ch.unwrap(), 0)), 0);
        }
        rotate(node, 1)
    }
    else if node.diff() == -2 {
        if diff(node.child(1)) == 1 {
            let (n, ch) = cut(node, 1);
            node = set(n, Some(rotate(ch.unwrap(), 1)), 1);
        }
        rotate(node, 0)
    }
    else { node }
}

fn deepest_node<N: AVLArrayNode>(node: Box<N>, dir: usize) -> (Box<N>, Link<N>) {
    let (mut n, ch) = cut(node, dir);
    match ch {
        Some(dir_node) => {
            let (deepest_node, dirn) = deepest_node(dir_node, dir);
            n = set(n, dirn, dir);
            (deepest_node, Some(balance(n)))
        }
        None => {
            cut(n, 1 - dir)
        }
    }
}

fn merge_dir<N: AVLArrayNode>(dst: Box<N>, mut root: Box<N>, src: Link<N>, dir: usize) -> Box<N> {
    if (dst.height() - height(&src)).abs() <= 1 {
        root = set(root, src, dir);
        root = set(root, Some(dst), 1 - dir);
        root
    }
    else {
        let (d, ch) = cut(dst, dir);
        match ch {
            Some(sch) => {
                let ch = Some(merge_dir(sch, root, src, dir));
                balance(set(d, ch, dir))
            }
            None => {
                balance(set(d, Some(balance(set(root, src, dir))), dir))
            }
        }
    }
}

fn merge<N: AVLArrayNode>(left: Link<N>, right: Link<N>) -> Link<N> {
    match left {
        Some(ln) => {
            match right {
                Some(rn) => {
                    if ln.height() >= rn.height() {
                        let (deep_left, src) = deepest_node(rn, 0);
                        Some(merge_dir(ln, deep_left, src, 1))
                    }
                    else {
                        let (deep_right, src) = deepest_node(ln, 1);
                        Some(merge_dir(rn, deep_right, src, 0))
                    }
                }
                None => Some(ln),
            }
        }
        None => right,
    } 
}

fn split<N: AVLArrayNode>(node: Box<N>, i: usize) -> (Link<N>, Link<N>) {
    if i == node.size() { return (Some(node), None); }
    let (node, left) = cut(node, 0);
    let (node, right) = cut(node, 1);
    if i < size(&left) {
        let (sp_left, sp_right) = split(left.unwrap(), i);
        let nright = match right {
            Some(nright) => Some(merge_dir(nright, node, sp_right, 0)),
            None => merge(sp_right, Some(node)),
        };
        (sp_left, nright)
    }
    else if i == size(&left) {
        (left, merge(Some(node), right))
    }
    else {
        let (sp_left, sp_right) = split(right.unwrap(), i - size(&left) - 1);
        let nleft = match left {
            Some(nleft) => Some(merge_dir(nleft, node, sp_left, 1)),
            None => merge(Some(node), sp_left),
        };
        (nleft, sp_right)
    }
}

fn at<N: AVLArrayNode + NodeValue>(node: &Box<N>, i: usize) -> &N::ValType {
    if size(&node.child_imut(0)) == i {
        node.val()
    }
    else if size(&node.child_imut(0)) < i {
        at(node.child_imut(1).as_ref().unwrap(), i - size(&node.child_imut(0)) - 1) } else {
        at(node.child_imut(0).as_ref().unwrap(), i)
    }
}

fn at_set<N: AVLArrayNode + NodeValue>(node: &mut Box<N>, i: usize, val: N::ValType) {
    let sz = size(&node.child(0));
    if sz == i {
        *node.as_mut().val_mut() = val
    }
    else if sz < i {
        at_set(node.child(1).as_mut().unwrap(), i - sz - 1, val); 
    } else {
        at_set(node.child(0).as_mut().unwrap(), i, val);
    }
    node.fix()
}

pub struct AVLTreeArray<N: AVLArrayNode> {
    root: Link<N>,
}

impl<N: AVLArrayNode> AVLTreeArray<N> {
    pub fn none() -> Self {
        Self { root: None }
    }
    pub fn new(n: N) -> Self {
        Self { root: Some(Box::new(n)) }
    }
    pub fn merge(self, right: Self) -> Self {
        Self { root: merge(self.root, right.root) }
    }
    pub fn split(self, i: usize) -> (Self, Self) {
        match self.root {
            Some(rn) => {
                let (l, r) = split(rn, i);
                ( Self { root: l }, Self { root: r } )
            }
            None => ( Self { root: None }, Self { root: None } )
        }
    }
}

impl<N: AVLArrayNode + NodeValue> AVLTreeArray<N> {
    pub fn at(&self, i: usize) -> &N::ValType {
        assert!(i < size(&self.root), "at(): out of range");
        at(self.root.as_ref().unwrap(), i)
    }
    pub fn at_set(&mut self, i: usize, val: N::ValType) {
        assert!(i < size(&self.root), "at_set(): out of range");
        at_set(self.root.as_mut().unwrap(), i, val);
    }
}

impl<N: AVLArrayNode + NodeFoldable> AVLTreeArray<N> where N::ValType: Monoid {
    pub fn fold(&self) -> N::ValType {
        match self.root {
            Some(ref node) => node.fold().clone(),
            None => <N::ValType as Unital>::identity(),
        }
    }
}



#[cfg(test)]
mod avlrsq_test {
    use algebra::*;
    use bbstree::node_traits::*;
    use bbstree::nodes::ArrNode;
    use std::cmp::max;

    #[derive(Clone)]
    struct Am(usize);

    impl Magma for Am {
        fn op(&self, right: &Self) -> Self { Am(self.0 + right.0) }
    }
    impl Associative for Am {}

    impl Unital for Am {
        fn identity() -> Self { Am(0) }
    }

    struct Qnode {
        size: usize,
        height: isize,
        val: Am,
        fold: Am,
        child: [Link<Qnode>; 2]
    }

    impl Qnode {
        fn new(val: Am) -> Self {
            Qnode {
                size: 1,
                height: 1,
                val: val,
                fold: Am::identity(),
                child: [None, None]
            }
        }
    }

    impl Fix for Qnode {
        fn fix(&mut self) {
            self.size = size(&self.child[0]) + size(&self.child[1]) + 1;
            self.height = max(height(&self.child[0]), height(&self.child[1])) + 1;
            self.fold = fold(&self.child[0])
                       .op(&self.val)
                       .op(&fold(&self.child[1]));
        }
    }
    impl Node for Qnode {
        fn child(&mut self, dir: usize) -> &mut Link<Self> { &mut self.child[dir] }
        fn child_imut(&self, dir: usize) -> &Link<Self> { &self.child[dir] }
    }
    impl NodeSize for Qnode { fn size(&self) -> usize { self.size } }
    impl NodeHeight for Qnode { fn height(&self) -> isize { self.height } }
    impl NodeValue for Qnode {
        type ValType = Am;
        fn val(&self) -> &Am { &self.val }
        fn val_mut(&mut self) -> &mut Am { &mut self.val }
    }
    impl NodeFoldable for Qnode { fn fold(&self) -> &Am { &self.fold } }

    #[test]
    fn avlrsq_test()  {
        use bbstree::avl_tree_array::AVLTreeArray;
        let arr = AVLTreeArray::none();
        let arr = arr.merge(AVLTreeArray::new(Qnode::new(Am(1))));
        let arr = arr.merge(AVLTreeArray::new(Qnode::new(Am(2))));
        let mut arr = arr.merge(AVLTreeArray::new(Qnode::new(Am(3))));
        {
            let (center, right) = arr.split(2);
            let (left, center) = center.split(0);
            assert!(center.fold().0 == Am(3).0);
            arr = left.merge(center).merge(right);
        }
        {
            let (center, right) = arr.split(2);
            let (left, center) = center.split(1);
            assert!(center.fold().0 == Am(2).0);
            let _ = left.merge(center).merge(right);
        }
    }
}

use algebra::*;

pub struct Node<M: Monoid> {
    left: Option<Box<Node<M>>>,
    right: Option<Box<Node<M>>>,
    val: M,
}

impl<M: Monoid> Node<M> {
    fn new() -> Self {
        Node {
            left: None,
            right: None,
            val: M::identity(),
        }
    }
}

pub fn value<M: Monoid>(node: &Option<Box<Node<M>>>) -> M {
    match node.as_ref() {
        Some(n) => n.as_ref().val.clone(),
        None => M::identity()
    }
}

pub fn update_node<M: Monoid>(node: &mut Node<M>, i: usize, x: M, l: usize, r: usize) {
    if l + 1 == r {
        node.val = x;
    }
    else {
        let m = (l + r) >> 1;
        if i < m {
            {
                if node.left.is_none() {
                    node.left = Some(Box::new(Node::new()));
                }
                let left = node.left.as_mut().unwrap().as_mut();
                update_node(left, i, x, l, m);
            }
        }
        else {
            {
                if node.right.is_none() {
                    node.right = Some(Box::new(Node::new()));
                }
                let right = node.right.as_mut().unwrap().as_mut();
                update_node(right, i, x, m, r);
            }
        }
        node.val = value(&node.left).op(&value(&node.right));
    }
}

pub fn fold<M: Monoid>(node: &Node<M>, a: usize, b: usize, l: usize, r: usize) -> M {
    if a <= l && r <= b { node.val.clone() }
    else if r <= a || b <= l { M::identity() }
    else {
        match node.left.as_ref() {
            Some(le) => fold(le, a, b, l, (l + r) >> 1),
            None => M::identity(),
        }.op(& match node.right.as_ref() {
            Some(ri) => fold(ri, a, b, (l + r) >> 1, r),
            None => M::identity(),
        })
    }
}

pub struct DynamicSegmentTree<M: Monoid> {
    root: Node<M>,
    n: usize,
}

impl<M: Monoid> DynamicSegmentTree<M> {
    pub fn new(n: usize) -> Self {
        let mut sz = 1;
        while sz < n { sz = sz << 1; }
        DynamicSegmentTree {
            root: Node::new(),
            n: sz
        }
    }

    pub fn update(&mut self, i: usize, x: M) {
        update_node(&mut self.root, i, x, 0, self.n);
    }

    pub fn fold(&self, a: usize, b: usize) -> M {
        fold(&self.root, a, b, 0, self.n)
    }
}

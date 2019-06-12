use algebra::*;

pub struct LazySegmentTree<T: Monoid, E: Effector<Target=T>> {
    node: Vec<T>,
    lazy: Vec<E>,
    sz: usize,
}

impl<T: Monoid, E: Effector<Target=T>> LazySegmentTree<T, E> {
    pub fn init(vec: Vec<T>) -> Self {
        let mut sz = 1;
        while sz < vec.len() { sz *= 2 }
        let mut node = vec![T::identity(); sz << 1];
        let lazy = vec![E::identity(); sz << 1];
        for i in 0..vec.len() { node[i + sz] = vec[i].clone(); }
        for i in (1..sz).rev() { node[i] = node[i << 1].op(&node[(i << 1) + 1]); }
        Self { node: node, lazy: lazy, sz: sz }
    }

    fn push(&mut self, i: usize, sz: usize) {
        self.node[i] = self.lazy[i].effect(&self.node[i], sz);
        if (i << 1) < self.node.len() {
            self.lazy[i << 1] = self.lazy[i << 1].op(&self.lazy[i]);
            self.lazy[(i << 1) + 1] = self.lazy[(i << 1) + 1].op(&self.lazy[i]);
        }
        self.lazy[i] = E::identity();
    }

    fn update_raw(&mut self, i: usize, a: usize, b: usize, l: usize, r: usize, e: &E) {
        self.push(i, r - l);
        if b <= l || r <= a { return; }
        else if a <= l && r <= b {
            self.lazy[i] = self.lazy[i].op(e);
            self.push(i, r - l);
        }
        else {
            self.update_raw(i << 1, a, b, l, (l + r) >> 1, e);
            self.update_raw((i << 1) + 1, a, b, (l + r) >> 1, r, e);
            self.node[i] = self.node[i << 1].op(&self.node[(i << 1) + 1]);
        }
    }

    pub fn update_range(&mut self, l: usize, r: usize, e: E) {
        let sz = self.sz;
        self.update_raw(1, l, r, 0, sz, &e);
    }

    fn fold_raw(&mut self, i: usize, a: usize, b: usize, l: usize, r: usize) -> T {
        self.push(i, r - l);
        if b <= l || r <= a { T::identity() }
        else if a <= l && r <= b { self.node[i].clone() }
        else {
            self.fold_raw(i << 1, a, b, l, (l + r) >> 1)
                .op(&self.fold_raw((i << 1) + 1, a, b, (l + r) >> 1, r))
        }
    }

    pub fn fold(&mut self, l: usize, r: usize) -> T {
        let sz = self.sz;
        self.fold_raw(1, l, r, 0, sz)
    }
}

#[cfg(test)]
mod rmq_ruq_test {
    use algebra::*;
    use segment_tree::lazy_segment_tree::LazySegmentTree;
    use std::cmp::min;

    #[derive(Clone)]
    struct Mm(usize);

    impl Magma for Mm {
        fn op(&self, right: &Self) -> Self { Mm(min(self.0, right.0)) }
    }
    impl Associative for Mm {}
    impl Unital for Mm {
        fn identity() -> Self { Mm(std::usize::MAX) }
    }

    #[derive(Clone)]
    struct Uq(Option<usize>);

    impl Magma for Uq {
        fn op(&self, right: &Self) -> Self {
            if right.0.is_none() { self.clone() }
            else { right.clone() }
        }
    }
    impl Associative for Uq {}
    impl Unital for Uq {
        fn identity() -> Self { Uq(None) }
    }
    impl Effector for Uq {
        type Target = Mm;
        fn effect(&self, t: &Self::Target, _: usize) -> Self::Target {
            match self.0 {
                Some(u) => Mm(u),
                None => t.clone(),
            }
        }
    }

    #[test]
    fn rmq_ruq_test() {
        let mut seg = LazySegmentTree::init(vec![Mm::identity(); 3]);
        seg.update_range(0, 2, Uq(Some(1)));
        seg.update_range(1, 3, Uq(Some(3)));
        seg.update_range(2, 3, Uq(Some(2)));
        assert_eq!(seg.fold(0, 3).0, 1);
        assert_eq!(seg.fold(1, 3).0, 2);
    }
}

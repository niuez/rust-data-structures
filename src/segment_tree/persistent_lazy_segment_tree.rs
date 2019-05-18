use algebra::*;

use std::rc::Rc;

type Link<T, E> = Option<Rc<Node<T, E>>>;

struct Node<T: Monoid, E: Effector<Target=T>> {
    data: T,
    eff: E,
    left: Link<T, E>,
    right: Link<T, E>,
}

impl<T: Monoid, E: Effector<Target=T>> Node<T, E> {
    fn new(data: T) -> Self {
        Node { data: data, eff: E::identity(), left: None, right: None }
    }
    fn build(l: usize, r: usize) -> Self {
        if l + 1 >= r { Node::new(T::identity()) }
        else {
            let left = Some(Rc::new(Node::<T, E>::build(l, (l + r) >> 1)));
            let right = Some(Rc::new(Node::<T, E>::build((l + r) >> 1, r)));
            Node {
                data: match left.as_ref() { Some(n) => n.data.clone(), None => T::identity() }
                      .op(& match right.as_ref() { Some(n) => n.data.clone(), None => T::identity() }),
                eff: E::identity(),
                left: left,
                right: right,
            }
        }
    }

    fn effect_range(&self, a: usize, b: usize, new_eff: E, l: usize, r: usize, fold_eff: E) -> Self {
        if a <= l && r <= b {
            let eff = fold_eff.op(&new_eff);
            Node {
                data: eff.effect(&self.data),
                eff: self.eff.op(&eff),
                left: self.left.clone(),
                right: self.right.clone(),
            }
        }
        else if r <= a || b <= l {
            Node {
                data: fold_eff.effect(&self.data),
                eff: self.eff.op(&fold_eff),
                left: self.left.clone(),
                right: self.right.clone(),
            }
        }
        else {
            let left = Some(Rc::new(self.left.as_ref().unwrap().effect_range(a, b, new_eff.clone(), l, (l + r) >> 1, self.eff.op(&fold_eff))));
            let right = Some(Rc::new(self.right.as_ref().unwrap().effect_range(a, b, new_eff.clone(), (l + r) >> 1, r, self.eff.op(&fold_eff))));
            Node {
                data: match left.as_ref() { Some(n) => n.data.clone(), None => T::identity() }
                      .op(& match right.as_ref() { Some(n) => n.data.clone(), None => T::identity() }),
                eff: E::identity(),
                left: left,
                right: right,
            }
        }
    }

    fn fold(&self, a: usize, b: usize, l: usize, r: usize, eff: E) -> T {
        if a <= l && r <= b { eff.effect(&self.data.clone()) }
        else if r <= a || b <= l { T::identity() }
        else {
            match self.left.as_ref() { Some(n) => n.fold(a, b, l, (l + r) >> 1, self.eff.op(&eff)), None => T::identity() }
                .op(& match self.right.as_ref() { Some(n) => n.fold(a, b, (l + r) >> 1, r, self.eff.op(&eff)), None => T::identity() })
        }
    }
}

impl<T: Monoid, E: Effector<Target=T>> Drop for Node<T, E> {
    fn drop(&mut self) {
        if let Some(left) = self.left.take() {
            if let Ok(_) = Rc::try_unwrap(left) {}
        }
        if let Some(right) = self.right.take() {
            if let Ok(_) = Rc::try_unwrap(right) {}
        }
    }
}


pub struct PersistentLazySegmentTree<T: Monoid, E: Effector<Target=T>> {
    root: Node<T, E>,
    sz: usize,
}

impl<T: Monoid, E: Effector<Target=T>> PersistentLazySegmentTree<T, E> {
    pub fn new(n: usize) -> Self {
        Self { root: Node::build(0, n), sz: n }
    }
    pub fn effect_range(&self, l: usize, r: usize, eff: E) -> Self {
        Self { root: self.root.effect_range(l, r, eff, 0, self.sz, E::identity()), sz: self.sz }
    }
    pub fn fold(&self, l: usize, r: usize) -> T {
        self.root.fold(l, r, 0, self.sz, E::identity())
    }
}

#[cfg(test)]
mod persistent_lazy_segment_tree_test {
    use algebra::*;
    use segment_tree::persistent_lazy_segment_tree::PersistentLazySegmentTree;
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
        fn effect(&self, t: &Self::Target) -> Self::Target {
            match self.0 {
                Some(u) => Mm(u),
                None => t.clone(),
            }
        }
    }

    #[test]
    fn rmq_ruq_test() {
        let seg = PersistentLazySegmentTree::new(3);
        let seg = seg.effect_range(0, 2, Uq(Some(1)));
        let seg = seg.effect_range(1, 3, Uq(Some(3)));
        let seg = seg.effect_range(2, 3, Uq(Some(2)));
        assert_eq!(seg.fold(0, 3).0, 1);
        assert_eq!(seg.fold(1, 3).0, 2);
    }


    #[derive(Clone, Debug)]
    struct Sm(usize, usize);

    impl Magma for Sm {
        fn op(&self, right: &Self) -> Self { Sm(self.0 + right.0, self.1 + right.1) }
    }
    impl Associative for Sm {}
    impl Unital for Sm {
        fn identity() -> Self { Sm(0, 1) }
    }

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
        fn effect(&self, t: &Self::Target) -> Self::Target {
            Sm(t.0 + self.0 * t.1, t.1)
        }
    }

    #[test]
    fn rsq_raq_test() {
        let seg = PersistentLazySegmentTree::new(3)
            .effect_range(0, 2, Aq(1))
            .effect_range(1, 3, Aq(2))
            .effect_range(2, 3, Aq(3));
        assert_eq!(seg.fold(0, 2).0, 4);
        assert_eq!(seg.fold(1, 3).0, 8);

    }
}

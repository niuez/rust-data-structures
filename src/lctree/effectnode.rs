use lctree::node_traits::*;
use algebra::*;

struct EffNode<T: Monoid, E: Effector<Target=T>> {
    ch: [Link<Self>; 2],
    par: Link<Self>,
    val: T,
    fold: T,
    eff: E,
    rev: bool,
    sz: usize,
}

impl<T: Monoid, E: Effector<Target=T>> Node for EffNode<T, E> {
    type Value = T;
    fn new(val: T) -> Self {
        EffNode {
            ch: [ None, None ],
            par: None,
            val: val.clone(),
            fold: val,
            eff: E::identity(),
            rev: false,
            sz: 1
        }
    }
    fn push(&mut self) {
        {
            if let Some(mut left) = self.ch[0] {
                unsafe { left.as_mut().effect(self.eff.clone()); }
            }
            if let Some(mut right) = self.ch[1] {
                unsafe { right.as_mut().effect(self.eff.clone()); }
            }
            self.eff = E::identity();
        }
        if self.rev {
            if let Some(mut left) = self.ch[0] {
                unsafe { left.as_mut().reverse(); }
            }
            if let Some(mut right) = self.ch[1] {
                unsafe { right.as_mut().reverse(); }
            }
            self.rev = false;
        }
    }
    fn reverse(&mut self) {
        self.ch.swap(0, 1);
        self.rev ^= true;
    }
    fn child(&self, dir: usize) -> &Link<Self> {
        &self.ch[dir]
    }
    fn child_mut(&mut self, dir: usize) -> &mut Link<Self> {
        &mut self.ch[dir]
    }
    fn parent(&self) -> &Link<Self> {
        &self.par
    }
    fn parent_mut(&mut self) -> &mut Link<Self> {
        &mut self.par
    }
    fn fix(&mut self) {
        self.sz = 1;
        self.fold = self.val.clone();
        unsafe { 
            if let Some(left) = self.ch[0] {
                self.sz += left.as_ref().sz;
                self.fold = left.as_ref().fold().op(&self.fold);
            }
            if let Some(right) = self.ch[1] {
                self.sz += right.as_ref().sz;
                self.fold = self.fold.op(right.as_ref().fold());
            }
        }
    }
    fn value(&self) -> &T { &self.val }
    fn value_mut(&mut self) -> &mut T { &mut self.val }
    fn size(&self) -> usize { self.sz }
}

impl<T: Monoid, E: Effector<Target=T>> EffectNode for EffNode<T, E> {
    type Effector = E;
    fn effect(&mut self, e: E) {
        self.val = self.eff.effect(&self.val, 1);
        self.fold = self.eff.effect(&self.fold, self.size());
        self.eff = self.eff.op(&e);
    }
    fn fold(&self) -> &T { &self.fold }
}

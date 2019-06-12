use std::ptr::NonNull;
use algebra::*;

pub type Link<N> = Option<NonNull<N>>;

pub trait Node: Sized {
    type Value;
    fn new(val: Self::Value) -> Self;
    fn push(&mut self);
    fn fix (&mut self);
    fn reverse(&mut self);
    fn child(&self, dir: usize) -> &Link<Self>;
    fn child_mut(&mut self, dir: usize) -> &mut Link<Self>;
    fn parent(&self) -> &Link<Self>;
    fn parent_mut(&mut self) -> &mut Link<Self>;
    fn value(&self) -> &Self::Value;
    fn value_mut(&mut self) -> &mut Self::Value;
    fn size(&self) -> usize;
}

pub trait EffectNode: Node where Self::Value: Monoid {
    type Effector: Effector<Target=Self::Value>;
    fn effect(&mut self, e: Self::Effector);
    fn fold(&self) -> &Self::Value;
}

fn is_root<N: Node>(node: &NonNull<N>) -> bool {
    unsafe {
        match node.as_ref().parent().clone() {
            None => true,
            Some(p) => {
                ( match p.as_ref().child(0) {
                    &None => true,
                    &Some(left) => left != *node,
                } ) &&
                ( match p.as_ref().child(1) {
                    &None => true,
                    &Some(right) => right != *node,
                } )
            }
        }
    }
} 

fn parent_dir<N: Node>(parent: &Link<N>, child: &NonNull<N>) -> Option<usize> {
    unsafe {
        match parent {
            None => None,
            Some(p) => {
                if *p.as_ref().child(0) == Some(*child) { Some(0) }
                else if *p.as_ref().child(1) == Some(*child) { Some(1) }
                else { None }
            }
        }
    }
}

fn rotate<N: Node>(mut t: NonNull<N>, dir: usize) {
    unsafe {
        let mut x = t.as_ref().parent().unwrap().clone();
        let y = x.as_ref().parent().clone();

        *x.as_mut().child_mut(dir ^ 1) = *t.as_ref().child(dir);
        if let Some(mut tr) = *t.as_ref().child(dir) {
            *tr.as_mut().parent_mut() = Some(x);
        }
        *t.as_mut().child_mut(dir) = Some(x);
        *x.as_mut().parent_mut() = Some(t);
        x.as_mut().fix();
        t.as_mut().fix();
        *t.as_mut().parent_mut() = y;
        if let Some(mut yy) = y {
            if let Some(xdir) = parent_dir(&y, &x) {
                *yy.as_mut().child_mut(xdir) = Some(t);
                yy.as_mut().fix();
            }
        }
    }
}

fn splay<N: Node>(mut t: NonNull<N>) {
    unsafe {
        t.as_mut().push();
        while !is_root(&t) {
            let mut q = t.as_ref().parent().clone().unwrap();
            if is_root(&q) {
                q.as_mut().push();
                t.as_mut().push();
                rotate(t, parent_dir(&Some(q), &t).unwrap() ^ 1);
            }
            else {
                let mut r = q.as_ref().parent().clone().unwrap();
                r.as_mut().push();
                q.as_mut().push();
                t.as_mut().push();
                let rq_dir = parent_dir(&Some(r), &q).unwrap();
                let qt_dir = parent_dir(&Some(q), &t).unwrap();
                if rq_dir == qt_dir {
                    rotate(q, rq_dir ^ 1);
                    rotate(t, qt_dir ^ 1);
                }
                else {
                    rotate(t, qt_dir ^ 1);
                    rotate(t, rq_dir ^ 1);
                }
            }
        }
    }
}

pub fn expose<N: Node>(t: NonNull<N>) -> Link<N> {
    let mut rp = None;
    let mut cur = Some(t);
    unsafe {
        while let Some(mut cc) = cur {
            splay(cc);
            *cc.as_mut().child_mut(1) = rp;
            cc.as_mut().fix();
            rp = cur;
            cur = cc.as_ref().parent().clone();
        }
    }
    splay(t);
    rp
}

pub fn lct_link<N: Node>(mut parent: NonNull<N>, mut child: NonNull<N>) {
    expose(child);
    expose(parent);
    unsafe {
        *child.as_mut().parent_mut() = Some(parent);
        *parent.as_mut().child_mut(1) = Some(child);
        child.as_mut().fix();
    }
}

pub fn lct_cut<N: Node>(mut child: NonNull<N>) {
    expose(child);
    unsafe {
        let mut parent = child.as_ref().child(0).unwrap();
        *child.as_mut().child_mut(0) = None;
        *parent.as_mut().parent_mut() = None;
        child.as_mut().fix();
    }
}

pub fn lct_lca<N: Node>(u: NonNull<N>, v: NonNull<N>) -> Link<N> {
    expose(u);
    expose(v)
}

pub fn lct_evert<N: Node>(mut t: NonNull<N>) {
    expose(t);
    unsafe {
        t.as_mut().reverse();
        t.as_mut().push();
    }
}

pub fn lct_effect<N: EffectNode>(mut t: NonNull<N>, e: N::Effector) where N::Value: Monoid {
    expose(t);
    unsafe {
        t.as_mut().effect(e);
        t.as_mut().push();
    }
}


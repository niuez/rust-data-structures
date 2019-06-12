pub trait Magma: Sized + Clone {
  fn op(&self, rhs: &Self) -> Self;
}

pub trait Associative: Magma {}

pub trait Unital: Magma {
  fn identity() -> Self;
}

pub trait Monoid: Magma + Associative + Unital {}

impl<T: Magma + Associative + Unital> Monoid for T {}

pub trait Effector: Monoid {
    type Target;
    fn effect(&self, t: &Self::Target, sz: usize) -> Self::Target;
}

//pub mod imp;

pub trait ScopedMut {
    type T;

    fn access<R>(&self, f: impl FnOnce(&Self::T)->R) -> R;
    fn access_mut<R>(&mut self, f: impl FnOnce(&mut Self::T)->R) -> R;
}

pub trait Interior {
    type T;

    fn interior_access<R>(&self, f: impl FnOnce(&Self::T)->R) -> R;
    fn interior_access_mut<R>(&self, f: impl FnOnce(&mut Self::T)->R) -> R;
}

pub trait ScopedOrInterior {
    type T;

    fn soi_access<R>(&self, f: impl FnOnce(&Self::T)->R) -> R;
    fn soi_access_mut<R>(&mut self, f: impl FnOnce(&mut Self::T)->R) -> R;
}
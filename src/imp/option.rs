use std::ops::*;

pub trait OptionExt<T> {
    #[inline] fn with<R>(&self, f: impl FnOnce(&T)->R) -> Option<R>;
    #[inline] fn with_mut<R>(&mut self, f: impl FnOnce(&mut T)->R) -> Option<R>;

    #[inline] fn add_to<V>(&mut self, v: V) where T: AddAssign<V>;
    #[inline] fn sub_to<V>(&mut self, v: V) where T: SubAssign<V>;
    #[inline] fn mul_to<V>(&mut self, v: V) where T: MulAssign<V>;
    #[inline] fn div_to<V>(&mut self, v: V) where T: DivAssign<V>;
}

impl<T> OptionExt<T> for Option<T> {
    #[inline] fn with<R>(&self, f: impl FnOnce(&T)->R) -> Option<R> {
        self.as_ref().map(f)
    }
    #[inline] fn with_mut<R>(&mut self, f: impl FnOnce(&mut T)->R) -> Option<R> {
        self.as_mut().map(f)
    }

    #[inline] fn add_to<V>(&mut self, v: V) where T: AddAssign<V> {
        self.with_mut(|s| AddAssign::add_assign(s,v) );
    }
    #[inline] fn sub_to<V>(&mut self, v: V) where T: SubAssign<V> {
        self.with_mut(|s| SubAssign::sub_assign(s,v) );
    }
    #[inline] fn mul_to<V>(&mut self, v: V) where T: MulAssign<V> {
        self.with_mut(|s| MulAssign::mul_assign(s,v) );
    }
    #[inline] fn div_to<V>(&mut self, v: V) where T: DivAssign<V> {
        self.with_mut(|s| DivAssign::div_assign(s,v) );
    }
}
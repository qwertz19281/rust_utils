
use std::iter::{FromIterator, Sum, repeat};
use std::mem::MaybeUninit;
use std::ops::*;

pub trait AsTuple {
    type Dest;

    fn as_tuple(self) -> Self::Dest;
}

pub trait AsArray {
    type Dest;

    fn as_array(self) -> Self::Dest;
}

pub trait TupleFns<T> where T: 'static {
    fn avg<U>(&self) -> U where T: Clone, U: From<T> + From<u8> + Sum<U> + DivAssign<U>;
}

macro_rules! impl_arr {
    {$n:expr;$t:ident $($ts:ident)*;$l:ident $($ls:ident)*} => {
        impl<T> AsTuple for [T; $n] {
            type Dest = ($t,$($ts),*);
            #[inline]
            fn as_tuple(self) -> Self::Dest {
                let [$l,$($ls),*] = self;
                ($l,$($ls),*)
            }
        }
        impl<T> AsArray for ($t,$($ts),*) {
            type Dest = [T; $n];
            #[inline]
            fn as_array(self) -> Self::Dest {
                let ($l,$($ls),*) = self;
                [$l,$($ls),*]
            }
        }
        impl<T> TupleFns<T> for [T; $n] where T: 'static {
            #[inline]
            fn avg<U>(&self) -> U where T: Clone, U: From<T> + From<u8> + Sum<U> + DivAssign<U> {
                let mut dest: U = self.iter().cloned().map(U::from).sum();
                dest /= U::from($n);
                dest
            }
        }
        impl<T> TupleFns<T> for ($t,$($ts),*) where Self: Clone, T: 'static {
            #[inline]
            fn avg<U>(&self) -> U where T: Clone, U: From<T> + From<u8> + Sum<U> + DivAssign<U>{
                (*self).clone().as_array().avg()
            }
        }
        impl_arr!{($n-1);$($ts)*;$($ls)*}
    };
    {$n:expr;;} => {};
}

impl_arr!{
    32;
    T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T;
    a b c d e f g h i j k l m n o p q r s t u v w x y z aa ab ac ad ae af
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_it() {
        let tuple = (1u32,2u32,3u32,4u32);
        let arr = tuple.clone().as_array();
        assert_eq!(arr,[1u32,2,3,4]);
        let t = arr.as_tuple();
        assert_eq!(t,tuple);

        assert_eq!((3u32,6u32,8u32,3u32,5u32,2u32).avg::<u32>(), 4u32);
        assert_eq!((3u32,6u32,8u32,3u32,5u32,2u32).avg::<f64>(), 4.5);
    }
}

pub trait ELA<T,DynFallback> where DynFallback: Sized {
    type ELA: Sized;

    fn ela_filled(&self, t: T) -> Self::ELA where T: Clone;
}

impl<T,U,D,const N: usize> ELA<U,D> for [T;N] {
    type ELA = [U;N];

    fn ela_filled(&self, u: U) -> Self::ELA where U: Clone {
        filled_array(|| u.clone() )
    }
}
impl<T,U,D,const N: usize> ELA<U,D> for &[T;N] {
    type ELA = [U;N];

    fn ela_filled(&self, u: U) -> Self::ELA where U: Clone {
        filled_array(|| u.clone() )
    }
}

impl<T,U,D> ELA<U,D> for [T] where D: FromIterator<U> {
    type ELA = D;

    fn ela_filled(&self, u: U) -> Self::ELA where U: Clone {
        FromIterator::from_iter(
            repeat(u).take(self.len())
        )
    }
}
impl<T,U,D> ELA<U,D> for &[T] where D: FromIterator<U> {
    type ELA = D;

    fn ela_filled(&self, u: U) -> Self::ELA where U: Clone {
        FromIterator::from_iter(
            repeat(u).take(self.len())
        )
    }
}

impl<T,U,D> ELA<U,D> for Vec<T> where D: FromIterator<U>{
    type ELA = D;

    fn ela_filled(&self, u: U) -> Self::ELA where U: Clone {
        FromIterator::from_iter(
            repeat(u).take(self.len())
        )
    }
}
impl<T,U,D> ELA<U,D> for &Vec<T> where D: FromIterator<U> {
    type ELA = D;

    fn ela_filled(&self, u: U) -> Self::ELA where U: Clone {
        FromIterator::from_iter(
            repeat(u).take(self.len())
        )
    }
}

#[inline]
pub fn filled_array<T,const N: usize>(mut f: impl FnMut()->T) -> [T;N] {
    let mut v: [MaybeUninit<T>;N] = unsafe {
        MaybeUninit::uninit().assume_init()
    };

    for v in &mut v {
        *v = MaybeUninit::new(f());
    }

    unsafe{
        std::mem::transmute_copy::<_,[T;N]>(&v)
    }
}

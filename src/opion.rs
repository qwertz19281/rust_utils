///asref_op!('b;E;DestType<E>;Add=add;AddAssign=add_assign;rhs: {...})
#[macro_export]
macro_rules! asref_op {
    ($($lifetimes:lifetime),*;$($generics:ty),*;$dest:expr;$op:ty=$fn:ident;$op_assign:ty=$fn_assign:ident;$rhs:ident: $f:block) => {
        impl<$($lifetimes),*R,$($generics),*> $op_assign<R> for $dest where R: AsRef<SizeAxis> {
            #[inline]
            fn $fn_assign(&mut self, $rhs: R) {
                let $rhs = r.as_ref();
                $f
            }
        }
        impl<$($lifetimes),*,R,$($generics),*> $op<R> for $dest where R: AsRef<SizeAxis> {
            type Output=SizeAxis;
            #[inline]
            fn add(mut self, r: R) -> Self::Output {
                self += r;
                self
            }
        }
        impl<$($lifetimes),*,'a,R,$($generics),*> Add<R> for &'a $dest where R: AsRef<SizeAxis> {
            type Output=SizeAxis;
            #[inline]
            fn add(self, r: R) -> Self::Output {
                self.clone() + r
            }
        }
    };
}

pub struct Test {}

asref_op!(;;Test;Add=add;AddAssign=add_assign;r: {});
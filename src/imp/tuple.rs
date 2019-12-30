
pub trait AsTuple {
    type Dest;

    fn as_tuple(self) -> Self::Dest;
}

pub trait AsArray {
    type Dest;

    fn as_array(self) -> Self::Dest;
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
        impl_arr!{($n-1);$($ts)*;$($ls)*}
    };
    {$n:expr;;} => {};
}

impl_arr!{
    32;
    T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T;
    a b c d e f g h i j k l m n o p q r s t u v w x y z aa ab ac ad ae af
}
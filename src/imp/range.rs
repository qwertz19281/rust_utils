use std::ops::Range;
use std::ops::Sub;

pub trait RangeExtSub2TCopy<T> where T: Sub<Output=T> + Copy {
    #[inline] fn len(&self) -> T;
}

impl<T> RangeExtSub2TCopy<T> for Range<T> where T: Sub<Output=T> + Copy {
    #[inline] fn len(&self) -> T {
        self.end-self.start
    }
}

/*impl RangeExtSub2TCopy<T> for RangeInclusive<T> where T: Sub<Output=T> + Add<Output=T> + One + Copy {
    fn len(&self) -> T {
        *self.end() - *self.start() + T::one()
    }
}*/ //TODO macro rules
pub trait VecExt<T> {
    #[inline] fn push_option(&mut self, o: Option<T>);

    #[inline] fn grow_to_with<F: FnMut() -> T>(&mut self, size: usize, f: F);

    #[inline] fn grow_to(&mut self, size: usize, value: T) where T: Clone;

    #[inline] fn grow_to_default(&mut self, size: usize) where T: Default;
}

impl<T> VecExt<T> for Vec<T> {
    #[inline] fn push_option(&mut self, o: Option<T>) {
        if let Some(o) = o {
            self.push(o);
        }
    }

    #[inline] fn grow_to_with<F: FnMut() -> T>(&mut self, size: usize, f: F) {
        if size > self.len() {
            self.resize_with(size, f);
        }
    }

    #[inline] fn grow_to(&mut self, size: usize, value: T) where T: Clone {
        if size > self.len() {
            self.resize(size, value);
        }
    }

    #[inline] fn grow_to_default(&mut self, size: usize) where T: Default {
        if size > self.len() {
            self.resize_with(size, T::default);
        }
    }
}
pub trait VecExt<T> {
    fn push_option(&mut self, o: Option<T>);

    fn grow_to_with<F: FnMut() -> T>(&mut self, size: usize, f: F);
}

impl<T> VecExt<T> for Vec<T> {
    fn push_option(&mut self, o: Option<T>) {
        if let Some(o) = o {
            self.push(o);
        }
    }

    fn grow_to_with<F: FnMut() -> T>(&mut self, size: usize, f: F) {
        if size > self.len() {
            self.resize_with(size, f);
        }
    }
}

pub trait VecExtClone<T> where T: Clone {
    fn grow_to(&mut self, size: usize, value: T);
}

impl<T> VecExtClone<T> for Vec<T> where T: Clone {
    fn grow_to(&mut self, size: usize, value: T) {
        if size > self.len() {
            self.resize(size, value);
        }
    }
}

pub trait VecExtDefault<T> where T: Default {
    fn grow_to_default(&mut self, size: usize);
}

impl<T> VecExtDefault<T> for Vec<T> where T: Default {
    fn grow_to_default(&mut self, size: usize) {
        if size > self.len() {
            self.resize_with(size, T::default);
        }
    }
}
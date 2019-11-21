pub struct GrowVec<T> {
    store: Vec<T>,
}

impl<T> GrowVec<T> where T: Default {
    /**ensure that the index is spawned */
    pub fn ensure(&mut self, idx: usize) {
        let (stor,idx)=self.vec_mut(idx);

        if idx >= stor.len() {
            stor.reserve(idx-stor.len()+1);
            while stor.len() <= idx {
                stor.push( T::default() );
            }
        }
    }
}

impl<T> Index<usize> for GrowVec<T> where T: Default {
    type Output = T;
    #[inline]
    fn index(&self, i: usize) -> &T {
        &self.store[idx]
    }
}

impl<T> IndexMut<usize> for GrowVec<T> where T: Default {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut T {
        if self.autospawn {self.ensure(i);}
        &mut self.store[idx]
    }
}

impl<T> Default for GrowVec<T> {
    fn default() -> Self {
        Self{store: Vec::new(), autospawn: false}
    }
}
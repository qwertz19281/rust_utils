use super::*;

pub struct PartVec<T> {
    inner: Vec<Vec<T>>,
    factor: usize,
}
pub struct RopeVec<T> {
    inner: Vec<(usize,Vec<T>)>,
}

impl<T> Clone for PartVec<T> {
    fn clone(&self) -> Self {
        todo!()
    }
}


impl<T> From<&[T]> for PartVec<T> where T: Clone {
    fn from(v: &[T]) -> Self {
        todo!()
    }
}

impl<T> Into<Vec<T>> for &PartVec<T> where T: Clone {
    fn into(self) -> Vec<T> {
        todo!()
    }
}
impl<T> Into<Vec<T>> for PartVec<T> where T: Clone {
    fn into(self) -> Vec<T> {
        todo!()
    }
}
impl<T> Into<Vec<T>> for &mut PartVec<T> where T: Clone {
    fn into(self) -> Vec<T> {
        todo!()
    }
}

impl<T> fmt::Debug for PartVec<T> where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl<T,O> PartialEq<O> for PartVec<T> where T: PartialEq, O: AsRef<[T]> {
    fn eq(&self, other: &O) -> bool {
        todo!()
    }
}
impl<T> Eq for PartVec<T> where T: Eq {}

impl<T> PartialOrd for PartVec<T> where T: PartialOrd {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        PartialOrd::partial_cmp(&**self,&**other)
    }
}
impl<T> Ord for PartVec<T> where T: Ord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Ord::cmp(&**self,&**other)
    }
}

impl<T> Extend<T> for PartVec<T> where T: Clone {
    fn extend<I>(&mut self, iter: I) where I: IntoIterator<Item = T> {
        let iter = iter.into_iter();
        let reserve = {
            let (mut lower,upper) = iter.size_hint();
            if let Some(upper) = upper {
                lower = lower.max(upper);
            }
            lower
        };
        let (vec,slice) = self._make_mut_with_capacity(self.len() + reserve);
        vec.truncate(slice.end);
        let reserved = vec.capacity() - slice.end;
        if reserve > reserved {
            vec.reserve(reserve-reserved);
        }
        for i in iter {
            vec.push(i);
            slice.end += 1;
        }
    }
}
impl<'a,T> Extend<&'a T> for PartVec<T> where T: Clone + 'a {
    fn extend<I>(&mut self, iter: I) where I: IntoIterator<Item = &'a T> {
        let iter = iter.into_iter();
        let reserve = {
            let (mut lower,upper) = iter.size_hint();
            if let Some(upper) = upper {
                lower = lower.max(upper);
            }
            lower
        };
        let (vec,slice) = self._make_mut_with_capacity(self.len() + reserve);
        vec.truncate(slice.end);
        let reserved = vec.capacity() - slice.end;
        if reserve > reserved {
            vec.reserve(reserve-reserved);
        }
        for i in iter {
            vec.push(i.clone());
            slice.end += 1;
        }
    }
}

impl<T> Hash for PartVec<T> where T: Hash {
    #[inline]
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        Hash::hash(&**self, state)
    }
}

impl<T> IntoIterator for PartVec<T> where T: Clone {
    type Item = T;
    type IntoIter = IntoIter<T>;

    #[inline]
    fn into_iter(self) -> IntoIter<T> {
        let slice = self.slice;
        match Arc::try_unwrap(self.inner) {
            Ok(mut v) => {
                v.truncate(slice.end);
                let mut iter = v.into_iter();
                for _ in 0..slice.start {
                    assert!(iter.next().is_some());
                }
                iter
            }
            Err(v) => {
                let v = v[slice].to_vec();
                v.into_iter()
            }
        }
    }
}

impl<'a, T> IntoIterator for &'a PartVec<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> slice::Iter<'a, T> {
        self[..].iter()
    }
}
impl<'a, T> IntoIterator for &'a mut PartVec<T> where T: Clone {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> slice::IterMut<'a, T> {
        self[..].iter_mut()
    }
}

impl Write for PartVec<u8> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.extend_from_slice(buf);
        Ok(buf.len())
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        let len = bufs.iter().map(|b| b.len()).sum();
        let (vec,slice) = self._make_mut_with_capacity(self.len() + len);
        vec.truncate(slice.end);
        vec.reserve(len);
        for buf in bufs {
            vec.extend_from_slice(buf);
        }
        slice.end += len;
        Ok(len)
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.write(buf)?;
        Ok(())
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

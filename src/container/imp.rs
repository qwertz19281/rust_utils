use std::sync::Arc;
use std::rc::Rc;
use std::borrow::Cow;
use std::sync::RwLock;
use std::cell::RefCell;
use super::*;

impl<'a,S,T> ScopedMut for &'a mut S where S: ScopedMut<T=T> {
    type T=T;
    #[inline]
    fn access<R>(&self, f: impl FnOnce(&T)->R) -> R {
        <S as ScopedMut>::access(self,f)
    }
    #[inline]
    fn access_mut<R>(&mut self, f: impl FnOnce(&mut T)->R) -> R {
        <S as ScopedMut>::access_mut(self,f)
    }
}

impl<S,T> ScopedMut for Box<S> where S: ScopedMut<T=T> {
    type T=T;
    #[inline]
    fn access<R>(&self, f: impl FnOnce(&T)->R) -> R {
        <S as ScopedMut>::access(self,f)
    }
    #[inline]
    fn access_mut<R>(&mut self, f: impl FnOnce(&mut T)->R) -> R {
        <S as ScopedMut>::access_mut(self,f)
    }
}

impl<S,T> ScopedMut for Cow<'_,S> where S: ScopedMut<T=T> + Clone {
    type T=T;
    #[inline]
    fn access<R>(&self, f: impl FnOnce(&T)->R) -> R {
        <S as ScopedMut>::access(self,f)
    }
    #[inline]
    fn access_mut<R>(&mut self, f: impl FnOnce(&mut T)->R) -> R {
        let mut b = self.to_mut();
        <S as ScopedMut>::access_mut(b,f)
    }
}

impl<S,T> Interior for RefCell<S> where S: ScopedMut<T=T> {
    type T=T;
    #[inline]
    fn interior_access<R>(&self, f: impl FnOnce(&T)->R) -> R {
        let b = self.try_borrow().expect("ContainerMut access failed: borrowed RefCell");
        <S as ScopedMut>::access(&b,f)
    }
    #[inline]
    fn interior_access_mut<R>(&self, f: impl FnOnce(&mut T)->R) -> R {
        let mut b = self.try_borrow_mut().expect("ContainerMut access failed: borrowed RefCell");
        <S as ScopedMut>::access_mut(&mut b,f)
    }
}

impl<S,T> Interior for RwLock<S> where S: ScopedMut<T=T> {
    type T=T;
    #[inline]
    fn interior_access<R>(&self, f: impl FnOnce(&T)->R) -> R {
        let b = self.read().expect("ContainerMut access failed: poisoned RwLock");
        <S as ScopedMut>::access(&b,f)
    }
    #[inline]
    fn interior_access_mut<R>(&self, f: impl FnOnce(&mut T)->R) -> R {
        let mut b = self.write().expect("ContainerMut access failed: poisoned RwLock");
        <S as ScopedMut>::access_mut(&mut b,f)
    }
}

impl<T,C> Interior for Rc<C> where C: Interior<T=T> {
    type T=T;
    #[inline]
    fn interior_access<R>(&self, f: impl FnOnce(&T)->R) -> R {
        <C as Interior>::interior_access(self,f)
    }
    #[inline]
    fn interior_access_mut<R>(&self, f: impl FnOnce(&mut T)->R) -> R {
        <C as Interior>::interior_access_mut(self,f)
    }
}

impl<T,C> Interior for Arc<C> where C: Interior<T=T> {
    type T=T;
    #[inline]
    fn interior_access<R>(&self, f: impl FnOnce(&T)->R) -> R {
        <C as Interior>::interior_access(self,f)
    }
    #[inline]
    fn interior_access_mut<R>(&self, f: impl FnOnce(&mut T)->R) -> R {
        <C as Interior>::interior_access_mut(self,f)
    }
}

impl<T,C> ScopedOrInterior for C where C: ScopedMut<T=T> {
    type T=T;
    #[inline]
    fn soi_access<R>(&self, f: impl FnOnce(&T)->R) -> R {
        <C as ScopedMut>::access(self,f)
    }
    #[inline]
    fn soi_access_mut<R>(&mut self, f: impl FnOnce(&mut T)->R) -> R {
        <C as ScopedMut>::access_mut(self,f)
    }
}

impl<T,C> ScopedOrInterior for C where C: Interior<T=T> {
    type T=T;
    #[inline]
    fn soi_access<R>(&self, f: impl FnOnce(&T)->R) -> R {
        <C as Interior>::access(self,f)
    }
    #[inline]
    fn soi_access_mut<R>(&mut self, f: impl FnOnce(&mut T)->R) -> R {
        <C as Interior>::access_mut(self,f)
    }
}
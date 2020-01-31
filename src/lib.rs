pub mod imp;
pub mod refc;
#[macro_use]
pub mod static_stor;

pub mod scoped;
pub mod if_type;
pub mod not_empty;
pub mod from_into;

pub mod macros;

use std::rc::Rc;
pub use imp::*;
pub use refc::*;
pub use if_type::*;
pub use not_empty::*;

/*pub mod stateful {
    pub use crate::static_stor;
    pub use crate::refc;
    pub use crate::scoped;
}*/

pub trait A<'a> {
    
}

/*struct EE;

struct Boo<'a> {
    pub c: Box<dyn A<'a>>,
    pub d: &'a EE,
}

fn short<'l: 's, 's>(e: Boo<'l>) -> Boo<'s> {
    e
}*/

fn short2<'l: 's, 's>(e: Box<dyn A<'l>>) -> Box<dyn A<'s>> {
    unsafe {
        std::mem::transmute(e)
    }
}

/*fn short3<'l: 's, 's>(e: &'l dyn A<'l>) -> &'s dyn A<'s> {
    e
}

fn short4<'l: 's, 's, S: A<'l>, D: A<'s> >(e: S) -> D {
    e as D
}*/

fn short3<'l: 's, 's>(e: Rc<&'l u32>) -> Rc<&'s u32> {
    e
}

pub trait V<T>: Sized where T: E<I=Self> {

}

pub trait E {
    type I;
}

struct U;

impl<T> V<T> for U where T: E<I=Self> {

}
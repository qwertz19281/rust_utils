pub mod imp;
pub mod refc;
#[macro_use]
pub mod static_stor;

pub mod scoped;
pub mod if_type;
pub mod not_empty;

pub mod macros;

pub use imp::*;
pub use refc::*;
pub use if_type::*;
pub use not_empty::*;

/*pub mod stateful {
    pub use crate::static_stor;
    pub use crate::refc;
    pub use crate::scoped;
}*/
pub mod imp;
pub mod refc;
#[macro_use]
pub mod static_stor;

pub mod scoped;

//pub mod opion;

pub use imp::*;
pub use refc::*;

/*pub mod stateful {
    pub use crate::static_stor;
    pub use crate::refc;
    pub use crate::scoped;
}*/
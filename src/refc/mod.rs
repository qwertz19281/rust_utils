mod imp;

/**
 * Clone function but only does cheap ref-cloning like rc
 */
pub trait RefClonable {
    #[inline] fn refc(&self) -> Self;
}

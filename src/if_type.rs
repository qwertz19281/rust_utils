use std::any::TypeId;

/// run if types are identical
#[inline]
pub fn if_type<Specific: 'static, T: 'static>(f: impl FnOnce() -> Specific) -> Option<T> {
    if TypeId::of::<T>() == TypeId::of::<Specific>() {
        let typed = f();
        let erased: T = unsafe{ std::mem::transmute_copy(&typed) }; //type and alignment ensured by identical type
        std::mem::forget(typed); //else we get some fancy double free
        Some(erased)
    }else{
        None
    }
}

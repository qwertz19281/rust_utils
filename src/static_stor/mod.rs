//code is inspired by query_interface's dynamic Registry, query_interface is dual-licensed under MIT or Apache-2.0
//(https://github.com/Diggsey/query_interface/blob/master/src/dynamic.rs)

/// generate a static global stor which is read fast but updated slow using cow technologies
/// create_static_stor!(VISIBILITY NAME: TYPE);
/// TODO FIXME minimum required visibility is pub(super)
/// stor type must implement Default and Clone
/// generates a pub (TODO: visibility options) module with with and with_mut fns
#[macro_export]
macro_rules! create_static_stor {
    ($v:vis $name:ident: $t:ty) => {
        #[allow(dead_code)]
        $v mod $name {
            $v mod private {
                use std::{cell::RefCell,sync::{Arc,RwLock,atomic::{AtomicUsize,Ordering}}};
                //use std::clone::Clone;

                lazy_static::lazy_static! {
                    pub static ref GLOBAL: RwLock<Arc<$t>> = RwLock::default();
                }

                pub static VERSION: AtomicUsize = AtomicUsize::new(0);

                pub fn update() -> (usize,Arc<$t>) {
                    let lock = GLOBAL.read().unwrap();

                    let s = Clone::clone(&*lock);
                    let v = VERSION.load(Ordering::Acquire);

                    (v,s)
                }

                std::thread_local! {
                    pub static LOCAL: RefCell<(usize,Arc<$t>)> = RefCell::new(update());
                }
            }

            /// access the $t of $name immutable, fast operation
            $v fn with<R, F: FnOnce(&$t) -> R>(f: F) -> R {
                private::LOCAL.with(|s| {
                    let mut s = s.borrow_mut();
                    
                    let v_now = private::VERSION.load(std::sync::atomic::Ordering::Acquire);

                    if v_now != s.0 {
                        *s = private::update();
                    }

                    f(&s.1)
                })
            }
            /// access the $t of $name mutable, slow operation
            $v fn with_mut<R, F: FnOnce(&mut $t) -> R>(f: F) -> R {
                let mut lock = private::GLOBAL.write().unwrap();

                let m = std::sync::Arc::make_mut(&mut lock);

                let r = f(m);

                private::VERSION.fetch_add(1, std::sync::atomic::Ordering::AcqRel);

                r
            }
        }
    };
}

#[cfg(test)]
mod test;
# qwutils (WIP)  

## Traits  

### Impls
```rust
Option {
    fn with(&self, |&T|) //short for as_ref().map()
    fn with_if(&self, &Option<U>, |&T,&U|)
    fn add/sub/mul/div_to(&self, &V)
}
bool {
    fn map(||->R)->R;
    //map_or, map_or_else also available
    fn option(&self) -> Option<()>;
    fn result(&self) -> Result<(),()>;
    more impls: https://crates.io/crates/boolinator
}
Vec {
    fn push_option(&mut self, o: Option<T>);
    fn grow_to_with(&mut self, len, FnOnce()->T);
    fn grow_to(&mut self, len, T) where T: Clone;
    fn grow_to_default(&mut self, len) where T: Default;
}
Result {
    fn expect_nodebug(&self, &str); //for T without Debug
    //expect_err, unwrap, unwrap_err also available
}
Range { //where T: Sub<T>
    fn len(&self) -> T;
}
```

### RefClonable  
```rust
trait RefClonable {
    fn refc(&self) -> Self;
}
```

- Fast reference cloning
- Similar to Clone
- Implemented on Rc/Arc
- ```refcounted.refc()``` behaves like ```Rc/Arc::clone(&refcounted)```

### ScopedAccess (WIP)
```rust
trait ScopedAccess {
    fn access(&self, |&T|)
    fn access_mut(&mut self, |&mut T|)
}
```

- Acces inner types which are only accessible scoped
- implemented on RefCell, RwLock, Rc/Arc<RefCell/RwLock>, references, Box, ...

## Macros

### static_stor

TODO

## Functions

### if_type (WIP)
```rust
pub fn if_type<T,Specific>(FnOnce()->Specific) -> T
```
- calls the given function if T and Specific are the same type (by comparing TypeId)
- Both Types must be statically known (T: 'static, Specific: 'static)
- Check out [transmogrify](https://github.com/sagebind/transmogrify) for casting between (statically known) types if both are the same type
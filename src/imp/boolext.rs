pub trait BoolExtOption {
    #[inline]
    fn option(&self) -> Option<()>;
    #[inline]
    fn result(&self) -> Result<(),()>;
    #[inline]
    fn map<U>(&self, f: impl FnOnce()->U) -> Option<U> {
        self.option().map(#[inline]|_|f())
    }
    #[inline]
    fn map_or<U>(&self, default: U, f: impl FnOnce()->U) -> U {
        self.option().map_or(default,#[inline]|_|f())
    }
    #[inline]
    fn map_or_else<U>(&self, default: impl FnOnce()->U, f: impl FnOnce()->U) -> U {
        self.option().map_or_else(default,#[inline]|_|f())
    }
}

impl BoolExtOption for bool {
    #[inline]
    fn option(&self) -> Option<()> {
        if *self {Some(())} else {None}
    }
    #[inline]
    fn result(&self) -> Result<(),()> {
        if *self {Ok(())} else {Err(())}
    }
}
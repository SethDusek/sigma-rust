use std::sync::Arc;

// TODO: consider passing through impl of Debug, PartialEq, Eq
#[derive(Clone, Debug, PartialEq, Eq)]
/// TODO
pub enum Ref<'ctx, T> {
    /// TODO
    Borrowed(&'ctx T),
    /// TODO
    Rc(Arc<T>),
}

impl<'ctx, T: Clone> Ref<'ctx, T> {
    /// Convert borrowed data to 'static lifetime
    pub fn to_static(&'ctx self) -> Ref<'static, T> {
        Ref::Rc(self.to_arc())
    }
    /// TODO
    pub fn to_arc(&'ctx self) -> Arc<T> {
        match self {
            Ref::Rc(r) => r.clone(),
            Ref::Borrowed(b) => Arc::new((*b).clone()),
        }
    }
}

impl<'ctx, T> From<&'ctx T> for Ref<'ctx, T> {
    fn from(val: &'ctx T) -> Self {
        Ref::Borrowed(val)
    }
}

impl<'ctx, T> From<Arc<T>> for Ref<'ctx, T> {
    fn from(val: Arc<T>) -> Self {
        Ref::Rc(val)
    }
}

impl<'ctx, T> From<T> for Ref<'ctx, T> {
    fn from(val: T) -> Self {
        Ref::Rc(Arc::new(val))
    }
}

impl<'ctx, T> std::ops::Deref for Ref<'ctx, T> {
    type Target = T;
    fn deref(&self) -> &T {
        match self {
            Ref::Borrowed(b) => b,
            Ref::Rc(rc) => &*rc,
        }
    }
}

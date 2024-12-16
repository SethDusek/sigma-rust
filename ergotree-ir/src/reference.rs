//! Reference type used extensively throughout interpreter. a Ref<'ctx, T> can either borrow from Context or be `Arc<T>`
use core::ops::Deref;

use alloc::sync::Arc;

#[derive(Clone, Debug, Eq)]
/// A wrapper type that provides immutable access to T. Ref can either be [`Ref::Borrowed`], holding a reference to some data in Context, or [`Ref::Arc`]
pub enum Ref<'ctx, T> {
    /// Data is borrowed from Context
    Borrowed(&'ctx T),
    /// Data is "owned"
    Arc(Arc<T>),
}

impl<T> core::cmp::PartialEq for Ref<'_, T>
where
    T: core::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}
impl<'ctx, T: Clone> Ref<'ctx, T> {
    /// Convert borrowed data to 'static lifetime
    pub fn to_static(&'ctx self) -> Ref<'static, T> {
        Ref::Arc(self.to_arc())
    }
    /// Convert [`Self`] to `Arc<T>`
    pub fn to_arc(&'ctx self) -> Arc<T> {
        match self {
            Ref::Arc(r) => r.clone(),
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
        Ref::Arc(val)
    }
}

impl<'ctx, T> From<T> for Ref<'ctx, T> {
    fn from(val: T) -> Self {
        Ref::Arc(Arc::new(val))
    }
}

impl<'ctx, T> core::ops::Deref for Ref<'ctx, T> {
    type Target = T;
    fn deref(&self) -> &T {
        match self {
            Ref::Borrowed(b) => b,
            Ref::Arc(rc) => rc,
        }
    }
}

#[cfg(test)]
#[cfg(feature = "arbitrary")]
mod test {
    use crate::reference::Ref;
    use proptest::prelude::*;

    proptest! {
        // Test that PartialEq for Ref<T> correctly passes through to PartialEq for T
        #[test]
        fn test_ref_partialeq(val in any::<u64>()) {
            let borrowed = Ref::from(&val);
            let owned = Ref::from(val);
            assert_eq!(borrowed, owned);
        }
    }
}

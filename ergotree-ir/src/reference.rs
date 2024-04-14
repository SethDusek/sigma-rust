use std::rc::Rc;

// TODO: consider passing through impl of Debug, PartialEq, Eq
#[derive(Clone, Debug, PartialEq, Eq)]
/// TODO
pub enum Ref<'ctx, T> {
    /// TODO
    Borrowed(&'ctx T),
    /// TODO
    Rc(Rc<T>),
}

impl<'ctx, T: Clone> Ref<'ctx, T> {
    /// TODO
    pub fn to_static(&'ctx self) -> Ref<'static, T> {
        Ref::Rc(self.to_rc())
    }
    /// TODO
    pub fn to_rc(&'ctx self) -> Rc<T> {
        match self {
            Ref::Rc(r) => r.clone(),
            Ref::Borrowed(b) => Rc::new((*b).clone()),
        }
    }
}

impl<'ctx, T> From<&'ctx T> for Ref<'ctx, T> {
    fn from(val: &'ctx T) -> Self {
        Ref::Borrowed(val)
    }
}

impl<'ctx, T> From<Rc<T>> for Ref<'ctx, T> {
    fn from(val: Rc<T>) -> Self {
        Ref::Rc(val)
    }
}

impl<'ctx, T> From<T> for Ref<'ctx, T> {
    fn from(val: T) -> Self {
        Ref::Rc(Rc::new(val))
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

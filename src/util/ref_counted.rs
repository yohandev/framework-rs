use std::{ borrow::Borrow, fmt::Pointer, ops::Deref };

/// abstracts over [Rc] and [Arc]
///
/// [Rc]: std::rc::Rc
/// [Arc]: std::sync::Arc
pub trait RefCounted<T: ?Sized>: Pointer
    + AsRef<T>
    + Borrow<T>
    + Clone
    + Deref
    + Drop
    + Unpin
{

}

impl<T: ?Sized> RefCounted<T> for std::rc::Rc<T> { }
impl<T: ?Sized> RefCounted<T> for std::sync::Arc<T> { }
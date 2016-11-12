use std::slice;

/// This trait is used to allow iterators to spawn implementing structures from their C representations.
pub trait AiIteratorAdapter<'a, T: 'a> {
    /// Inner type of the `AiIterator`, usually some FFI pointer or something.
    type Inner;

    /// Converts the raw inner value into some value of `T`, preserving the lifetimes
    fn from(&'a Self::Inner) -> T;
}

/// Using an Iterator to traverse dynamically allocated arrays from Assimp
#[derive(Clone)]
pub struct AiIterator<'a, T: 'a> where T: AiIteratorAdapter<'a, T>, T::Inner: 'a {
    inner: slice::Iter<'a, T::Inner>,
}

/// Avoids direct access to `T::Inner` and `AiIterator` by providing a conversion method
impl<'a, T: 'a> From<slice::Iter<'a, T::Inner>> for AiIterator<'a, T> where T: AiIteratorAdapter<'a, T>, T::Inner: 'a {
    #[inline(always)]
    fn from(s: slice::Iter<'a, T::Inner>) -> AiIterator<'a, T> {
        AiIterator { inner: s }
    }
}

/// Avoids direct access to `T::Inner` and `AiIterator` by providing a conversion method
impl<'a, T: 'a> From<&'a [T::Inner]> for AiIterator<'a, T> where T: AiIteratorAdapter<'a, T>, T::Inner: 'a {
    #[inline(always)]
    fn from(s: &'a [T::Inner]) -> AiIterator<'a, T> {
        AiIterator { inner: s.iter() }
    }
}

/// Iterator implementation of `AiIterator` that transforms `T::Inner` into `T`
impl<'a, T: 'a> Iterator for AiIterator<'a, T> where T: AiIteratorAdapter<'a, T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(inner) = self.inner.next() { Some(T::from(inner as &'a T::Inner)) } else { None }
    }
}

/// Allows `AIIterators` to move backwards
impl<'a, T: 'a> DoubleEndedIterator for AiIterator<'a, T> where T: AiIteratorAdapter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(inner) = self.inner.next_back() { Some(T::from(inner as &'a T::Inner)) } else { None }
    }
}

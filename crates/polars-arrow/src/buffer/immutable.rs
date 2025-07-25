#![allow(unsafe_op_in_unsafe_fn)]
use std::ops::Deref;

use bytemuck::{Pod, Zeroable};
use either::Either;

use super::IntoIter;
use crate::array::{ArrayAccessor, Splitable};
use crate::storage::SharedStorage;

/// [`Buffer`] is a contiguous memory region that can be shared across
/// thread boundaries.
///
/// The easiest way to think about [`Buffer<T>`] is being equivalent to
/// a `Arc<Vec<T>>`, with the following differences:
/// * slicing and cloning is `O(1)`.
/// * it supports external allocated memory
///
/// The easiest way to create one is to use its implementation of `From<Vec<T>>`.
///
/// # Examples
/// ```
/// use polars_arrow::buffer::Buffer;
///
/// let mut buffer: Buffer<u32> = vec![1, 2, 3].into();
/// assert_eq!(buffer.as_ref(), [1, 2, 3].as_ref());
///
/// // it supports copy-on-write semantics (i.e. back to a `Vec`)
/// let vec: Vec<u32> = buffer.into_mut().right().unwrap();
/// assert_eq!(vec, vec![1, 2, 3]);
///
/// // cloning and slicing is `O(1)` (data is shared)
/// let mut buffer: Buffer<u32> = vec![1, 2, 3].into();
/// let mut sliced = buffer.clone();
/// sliced.slice(1, 1);
/// assert_eq!(sliced.as_ref(), [2].as_ref());
/// // but cloning forbids getting mut since `slice` and `buffer` now share data
/// assert_eq!(buffer.get_mut_slice(), None);
/// ```
#[derive(Clone)]
pub struct Buffer<T> {
    /// The internal byte buffer.
    storage: SharedStorage<T>,

    /// A pointer into the buffer where our data starts.
    ptr: *const T,

    // The length of the buffer.
    length: usize,
}

unsafe impl<T: Send + Sync> Sync for Buffer<T> {}
unsafe impl<T: Send + Sync> Send for Buffer<T> {}

impl<T: PartialEq> PartialEq for Buffer<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}

impl<T: Eq> Eq for Buffer<T> {}

impl<T: std::hash::Hash> std::hash::Hash for Buffer<T> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Buffer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&**self, f)
    }
}

impl<T> Default for Buffer<T> {
    #[inline]
    fn default() -> Self {
        Vec::new().into()
    }
}

impl<T> Buffer<T> {
    /// Creates an empty [`Buffer`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Auxiliary method to create a new Buffer
    pub fn from_storage(storage: SharedStorage<T>) -> Self {
        let ptr = storage.as_ptr();
        let length = storage.len();
        Buffer {
            storage,
            ptr,
            length,
        }
    }

    pub fn from_static(data: &'static [T]) -> Self {
        Self::from_storage(SharedStorage::from_static(data))
    }

    /// Returns the number of bytes in the buffer
    #[inline]
    pub fn len(&self) -> usize {
        self.length
    }

    /// Returns whether the buffer is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Returns whether underlying data is sliced.
    /// If sliced the [`Buffer`] is backed by
    /// more data than the length of `Self`.
    pub fn is_sliced(&self) -> bool {
        self.storage.len() != self.length
    }

    /// Expands this slice to the maximum allowed by the underlying storage.
    /// Only expands towards the end, the offset isn't changed. That is, element
    /// i before and after this operation refer to the same element.
    pub fn expand_end_to_storage(self) -> Self {
        unsafe {
            let offset = self.ptr.offset_from(self.storage.as_ptr()) as usize;
            Self {
                ptr: self.ptr,
                length: self.storage.len() - offset,
                storage: self.storage,
            }
        }
    }

    /// Returns the byte slice stored in this buffer
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        // SAFETY:
        // invariant of this struct `offset + length <= data.len()`
        debug_assert!(self.offset() + self.length <= self.storage.len());
        unsafe { std::slice::from_raw_parts(self.ptr, self.length) }
    }

    /// Returns the byte slice stored in this buffer
    ///
    /// # Safety
    /// `index` must be smaller than `len`
    #[inline]
    pub(super) unsafe fn get_unchecked(&self, index: usize) -> &T {
        // SAFETY:
        // invariant of this function
        debug_assert!(index < self.length);
        unsafe { &*self.ptr.add(index) }
    }

    /// Returns a new [`Buffer`] that is a slice of this buffer starting at `offset`.
    /// Doing so allows the same memory region to be shared between buffers.
    /// # Panics
    /// Panics iff `offset + length` is larger than `len`.
    #[inline]
    pub fn sliced(self, offset: usize, length: usize) -> Self {
        assert!(
            offset + length <= self.len(),
            "the offset of the new Buffer cannot exceed the existing length"
        );
        // SAFETY: we just checked bounds
        unsafe { self.sliced_unchecked(offset, length) }
    }

    /// Slices this buffer starting at `offset`.
    /// # Panics
    /// Panics iff `offset + length` is larger than `len`.
    #[inline]
    pub fn slice(&mut self, offset: usize, length: usize) {
        assert!(
            offset + length <= self.len(),
            "the offset of the new Buffer cannot exceed the existing length"
        );
        // SAFETY: we just checked bounds
        unsafe { self.slice_unchecked(offset, length) }
    }

    /// Returns a new [`Buffer`] that is a slice of this buffer starting at `offset`.
    /// Doing so allows the same memory region to be shared between buffers.
    ///
    /// # Safety
    /// The caller must ensure `offset + length <= self.len()`
    #[inline]
    #[must_use]
    pub unsafe fn sliced_unchecked(mut self, offset: usize, length: usize) -> Self {
        debug_assert!(offset + length <= self.len());

        self.slice_unchecked(offset, length);
        self
    }

    /// Slices this buffer starting at `offset`.
    ///
    /// # Safety
    /// The caller must ensure `offset + length <= self.len()`
    #[inline]
    pub unsafe fn slice_unchecked(&mut self, offset: usize, length: usize) {
        self.ptr = self.ptr.add(offset);
        self.length = length;
    }

    /// Returns a pointer to the start of the storage underlying this buffer.
    #[inline]
    pub(crate) fn storage_ptr(&self) -> *const T {
        self.storage.as_ptr()
    }

    /// Returns the start offset of this buffer within the underlying storage.
    #[inline]
    pub fn offset(&self) -> usize {
        unsafe {
            let ret = self.ptr.offset_from(self.storage.as_ptr()) as usize;
            debug_assert!(ret <= self.storage.len());
            ret
        }
    }

    /// # Safety
    /// The caller must ensure that the buffer was properly initialized up to `len`.
    #[inline]
    pub unsafe fn set_len(&mut self, len: usize) {
        self.length = len;
    }

    /// Returns a mutable reference to its underlying [`Vec`], if possible.
    ///
    /// This operation returns [`Either::Right`] iff this [`Buffer`]:
    /// * has no alive clones
    /// * has not been imported from the C data interface (FFI)
    #[inline]
    pub fn into_mut(mut self) -> Either<Self, Vec<T>> {
        // We lose information if the data is sliced.
        if self.is_sliced() {
            return Either::Left(self);
        }
        match self.storage.try_into_vec() {
            Ok(v) => Either::Right(v),
            Err(slf) => {
                self.storage = slf;
                Either::Left(self)
            },
        }
    }

    /// Returns a mutable reference to its slice, if possible.
    ///
    /// This operation returns [`Some`] iff this [`Buffer`]:
    /// * has no alive clones
    /// * has not been imported from the C data interface (FFI)
    #[inline]
    pub fn get_mut_slice(&mut self) -> Option<&mut [T]> {
        let offset = self.offset();
        let slice = self.storage.try_as_mut_slice()?;
        Some(unsafe { slice.get_unchecked_mut(offset..offset + self.length) })
    }

    /// Since this takes a shared reference to self, beware that others might
    /// increment this after you've checked it's equal to 1.
    pub fn storage_refcount(&self) -> u64 {
        self.storage.refcount()
    }
}

impl<T: Pod> Buffer<T> {
    pub fn try_transmute<U: Pod>(mut self) -> Result<Buffer<U>, Self> {
        assert_ne!(size_of::<U>(), 0);
        let ptr = self.ptr as *const U;
        let length = self.length;
        match self.storage.try_transmute() {
            Err(v) => {
                self.storage = v;
                Err(self)
            },
            Ok(storage) => Ok(Buffer {
                storage,
                ptr,
                length: length.checked_mul(size_of::<T>()).expect("overflow") / size_of::<U>(),
            }),
        }
    }
}

impl<T: Clone> Buffer<T> {
    pub fn make_mut(self) -> Vec<T> {
        match self.into_mut() {
            Either::Right(v) => v,
            Either::Left(same) => same.as_slice().to_vec(),
        }
    }
}

impl<T: Zeroable + Copy> Buffer<T> {
    pub fn zeroed(len: usize) -> Self {
        vec![T::zeroed(); len].into()
    }
}

impl<T> From<Vec<T>> for Buffer<T> {
    #[inline]
    fn from(v: Vec<T>) -> Self {
        Self::from_storage(SharedStorage::from_vec(v))
    }
}

impl<T> Deref for Buffer<T> {
    type Target = [T];

    #[inline(always)]
    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> AsRef<[T]> for Buffer<T> {
    #[inline(always)]
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> FromIterator<T> for Buffer<T> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Vec::from_iter(iter).into()
    }
}

impl<T: Copy> IntoIterator for Buffer<T> {
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

unsafe impl<'a, T: 'a> ArrayAccessor<'a> for Buffer<T> {
    type Item = &'a T;

    unsafe fn value_unchecked(&'a self, index: usize) -> Self::Item {
        unsafe { &*self.ptr.add(index) }
    }

    fn len(&self) -> usize {
        Buffer::len(self)
    }
}

impl<T> Splitable for Buffer<T> {
    #[inline(always)]
    fn check_bound(&self, offset: usize) -> bool {
        offset <= self.len()
    }

    unsafe fn _split_at_unchecked(&self, offset: usize) -> (Self, Self) {
        let storage = &self.storage;

        (
            Self {
                storage: storage.clone(),
                ptr: self.ptr,
                length: offset,
            },
            Self {
                storage: storage.clone(),
                ptr: self.ptr.wrapping_add(offset),
                length: self.length - offset,
            },
        )
    }
}

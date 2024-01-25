//! Crazy LLVM optimizations.
//! 
//! Specs: `no_std` & `nightly`
//! 
//! # Example
//! 
//! ```rust
//! # use llvm::*;
//! # use std::thread;
//! # use std::time::Duration;
//! const RANDOM_NUMBER: i32 = 5;// It's random I swear ;)
//! 
//! let is_one_done = false;
//! let done = r(is_one_done);
//! 
//! for i in 0..10 {
//!     thread::spawn(move || {
//!         if i == RANDOM_NUMBER {
//!             thread::sleep(Duration::from_secs(1));
//!             unsafe { *done_ptr.gmut() = true; }
//!             println!("{i} - I Finished!");
//!             return;
//!         }
//!         loop {
//!             if unlikely(done == true) {
//!                 println!("{i} - Somebody else finished!");
//!                 break;
//!             }
//!         }
//!     })
//! }
//! ```

#![no_std]

#![deny(unused, missing_docs)]

#![cfg_attr(feature = "nightly", allow(internal_features))]
#![cfg_attr(feature = "nightly", feature(
    core_intrinsics,
    const_mut_refs
))]

#[cfg(test)]
mod tests;

/// Unsafely assumes the value of an expression to be `true`.
/// 
/// If it is false, it will cause UB.
///
/// The compiler is sometimes able to use this to optimize better, but it often backfires.
#[inline(always)]
pub unsafe fn assume(b: bool) {
    core::intrinsics::assume(b)
}

/// Tells the compiler this `bool` is most likely `false`.
#[inline(always)]
pub fn unlikely(b: bool) -> bool {
    core::intrinsics::unlikely(b)
}

/// Tells the compiler this `bool` is most likely `true`.
#[inline(always)]
pub fn likely(b: bool) -> bool {
    core::intrinsics::likely(b)
}

/// Tells the compiler this code is unreachable.
#[inline(always)]
pub const unsafe fn unreach() -> ! {
    core::hint::unreachable_unchecked()
}

/// Assumes a closure will not panic.
///
/// Calls to `core::panicking` functions will still be generated;
/// however, this function is nounwind and panics will cause UB.
#[inline(always)]
pub unsafe extern "C" fn assume_nopanic<F: FnOnce() -> T, T>(f: F) -> T {
    #[repr(transparent)]
    struct NoPanic;
    impl Drop for NoPanic {
        #[inline(always)]
        fn drop(&mut self) {
            unsafe { unreach(); }
        }
    }
    let no_panic = NoPanic;
    let r = f();
    core::mem::forget(no_panic);
    r
}

/// A reference to a value that is known to be valid.
/// 
/// This is a wrapper around a raw pointer that is `Send` and `Sync`.
/// 
/// # Safety
/// 
/// The pointer must be valid. It is not checked.
#[repr(transparent)]
pub struct R<T>(*const T);

/// Creates a new `R` from a reference.
#[inline(always)]
pub const fn r<T>(t: &T) -> R<T> {
    R(t)
}

impl<T> R<T> {
    /// Returns a mutable reference to the value.
    /// 
    /// # Safety
    /// 
    /// The pointer isn't checked for validity.
    /// Calling this may cause UB if the pointer is not valid.
    #[inline(always)]
    pub const unsafe fn gmut(&self) -> &mut T {
        &mut *(self.0 as *mut T)
    }
    /// Returns a reference to the value.
    /// 
    /// # Safety
    /// 
    /// The pointer isn't checked for validity.
    /// Calling this may cause UB if the pointer is not valid.
    #[inline(always)]
    pub const unsafe fn gref(&self) -> &T {
        &*self.0
    }
    /// Returns the internal raw pointer
    #[inline(always)]
    pub const fn gptr(&self) -> *const T {
        self.0
    }
}

impl<T> core::ops::Deref for R<T> {
    type Target = T;
    /// Quickly dereferences the pointer.
    /// 
    /// # Safety
    /// 
    /// The pointer isn't checked for validity.
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { self.gref() }
    }
}

unsafe impl<T> Send for R<T> {}
unsafe impl<T> Sync for R<T> {}
impl<T> Copy  for R<T> {}
impl<T> Clone for R<T> {
    /// Clones the reference.
    /// 
    /// # Safety
    /// 
    /// The pointer isn't checked for validity.
    #[inline(always)]
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T: PartialEq> PartialEq<T> for R<T> {
    /// Compares the pointer.
    /// 
    /// # Safety
    /// 
    /// The pointer isn't checked for validity.
    #[inline(always)]
    fn eq(&self, other: &T) -> bool {
        unsafe { self.gref() == other }
    }
}

impl<T: PartialEq> PartialEq for R<T> {
    /// Compares the pointer.
    /// 
    /// # Safety
    /// 
    /// The pointer isn't checked for validity.
    #[inline(always)]
    fn eq(&self, other: &R<T>) -> bool {
        unsafe { self.gref() == other.gref() }
    }
}

impl<T: Eq> Eq for R<T> {}

impl<T: PartialOrd> PartialOrd<T> for R<T> {
    /// Compares the pointer.
    /// 
    /// # Safety
    /// 
    /// The pointer isn't checked for validity.
    #[inline(always)]
    fn partial_cmp(&self, other: &T) -> Option<core::cmp::Ordering> {
        unsafe { self.gref().partial_cmp(other) }
    }
}

impl<T: PartialOrd> PartialOrd for R<T> {
    /// Compares the pointer.
    /// 
    /// # Safety
    /// 
    /// The pointer isn't checked for validity.
    #[inline(always)]
    fn partial_cmp(&self, other: &R<T>) -> Option<core::cmp::Ordering> {
        unsafe { self.gref().partial_cmp(other.gref()) }
    }
}

impl<T: Ord> Ord for R<T> {
    /// Compares the pointer.
    /// 
    /// # Safety
    /// 
    /// The pointer isn't checked for validity.
    #[inline(always)]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        unsafe { self.gref().cmp(other.gref()) }
    }
}

impl<T: core::hash::Hash> core::hash::Hash for R<T> {
    /// Hashes the pointer.
    /// 
    /// # Safety
    /// 
    /// The pointer isn't checked for validity.
    #[inline(always)]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        unsafe { self.gref().hash(state) }
    }
}

impl<T: core::fmt::Debug> core::fmt::Debug for R<T> {
    /// Formats the pointer.
    /// 
    /// Returns: `@{pointer} -> {debugged_value}`
    /// 
    /// # Safety
    /// 
    /// The pointer isn't checked for validity.
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "@{:p} -> {:?}", &self.0, &*self)
    }
}

impl<T: core::fmt::Display> core::fmt::Display for R<T> {
    /// Formats the pointer.
    /// 
    /// Returns: `{displayed_value}`
    /// 
    /// # Safety
    /// 
    /// The pointer isn't checked for validity.
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", &*self)
    }
}

impl<T> core::fmt::Pointer for R<T> {
    /// Formats the pointer.
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Pointer::fmt(&self.0, f)
    }
}

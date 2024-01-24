#![no_std]

#![feature(core_intrinsics)]
#![allow(internal_features)]

/// Unsafely assumes the value of an expression to be `true`.
///
/// The compiler is sometimes able to use this to optimize better, but it often backfires.
#[inline(always)]
pub fn assume(b: bool) {
    unsafe { core::intrinsics::assume(b) }
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
pub fn unreach() -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}

/// Assumes a closure will not panic.
///
/// Calls to `core::panicking` functions will still be generated;
/// however, this function is nounwind and panics will cause UB.
#[inline(always)]
pub unsafe extern "C" fn assume_nopanic<F: FnOnce() -> T, T>(f: F) -> T {
    struct NoPanic;
    impl Drop for NoPanic {
        #[inline(always)]
        fn drop(&mut self) { unreach(); }
    }
    let no_panic = NoPanic;
    let r = f();
    core::mem::forget(no_panic);
    r
}

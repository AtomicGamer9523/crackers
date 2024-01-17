pub(crate) struct UnsafeMultithreadedRC<T>(T);

impl<T> core::ops::Deref for UnsafeMultithreadedRC<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> UnsafeMultithreadedRC<T> {
    #[inline]
    pub fn new(value: T) -> Self {
        Self(value)
    }

    pub fn thread_shared_ref(&self) -> ThreadSharedRef<T> {
        ThreadSharedRef(&self.0 as *const T)
    }
}

pub(crate) struct ThreadSharedRef<T>(*const T);
impl<T> Clone for ThreadSharedRef<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T> Copy for ThreadSharedRef<T> {}

impl<T> core::ops::Deref for ThreadSharedRef<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

unsafe impl<T> Send for ThreadSharedRef<T> {}
unsafe impl<T> Sync for ThreadSharedRef<T> {}

pub(crate) trait UnsafeMultithreadedRcExt<T> {
    fn as_unsafe_multithreaded_rc(self) -> UnsafeMultithreadedRC<T>;
}
impl<T> UnsafeMultithreadedRcExt<T> for T {
    #[inline]
    fn as_unsafe_multithreaded_rc(self) -> UnsafeMultithreadedRC<T> {
        UnsafeMultithreadedRC::new(self)
    }
}

unsafe impl<T> Send for UnsafeMultithreadedRC<T> {}
unsafe impl<T> Sync for UnsafeMultithreadedRC<T> {}

use core::fmt::{self, Debug, Display};
use core::marker::PhantomData;
use core::mem;
use core::ops::{Deref, DerefMut};
use core::pin::Pin;

pub trait UniquePtrTarget
where
    Self: Sized,
{
    unsafe fn __drop(this: &mut UniquePtr<Self>);
}

/// Binding to C++ `std::unique_ptr<T, std::default_delete<T>>`.
#[repr(C)]
pub struct UniquePtr<T: UniquePtrTarget> {
    pub ptr: *mut T,
    pd: PhantomData<T>,
}

impl<T: UniquePtrTarget> UniquePtr<T> {
    pub fn get_ptr(&self) -> *const T {
        self.ptr as *const T
    }

    pub fn null() -> Self {
        UniquePtr {
            ptr: std::ptr::null_mut(),
            pd: PhantomData,
        }
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    pub fn as_ref(&self) -> Option<&T> {
        unsafe { (self.ptr as *const T).as_ref() }
    }

    pub fn as_mut(&mut self) -> Option<&mut T>
    where
        T: Unpin,
    {
        unsafe { (self.ptr as *mut T).as_mut() }
    }

    pub fn as_pin_mut(&mut self) -> Option<Pin<&mut T>> {
        unsafe {
            let p = (self.ptr as *mut T).as_mut()?;
            Some(Pin::new_unchecked(p))
        }
    }

    pub fn pin_mut(&mut self) -> Pin<&mut T> {
        match self.as_pin_mut() {
            Some(target) => target,
            None => panic!(
                "called pin_mut on a null UniquePtr<{}>",
                std::any::type_name::<T>(),
            ),
        }
    }

    pub unsafe fn from_raw(raw: *mut T) -> Self {
        UniquePtr {
            ptr: raw,
            pd: PhantomData,
        }
    }

    pub fn into_raw(self) -> *mut T {
        let ptr = self.ptr;
        mem::forget(self);
        ptr
    }
}

unsafe impl<T> Send for UniquePtr<T> where T: Send + UniquePtrTarget {}
unsafe impl<T> Sync for UniquePtr<T> where T: Sync + UniquePtrTarget {}

impl<T: UniquePtrTarget> Drop for UniquePtr<T> {
    fn drop(&mut self) {
        unsafe { T::__drop(self) }
    }
}

impl<T: UniquePtrTarget> Deref for UniquePtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self.as_ref() {
            Some(target) => target,
            None => panic!(
                "called deref on a null UniquePtr<{}>",
                std::any::type_name::<T>(),
            ),
        }
    }
}

impl<T: UniquePtrTarget + Unpin> DerefMut for UniquePtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self.as_mut() {
            Some(target) => target,
            None => panic!(
                "called deref_mut on a null UniquePtr<{}>",
                std::any::type_name::<T>(),
            ),
        }
    }
}

impl<T: Debug + UniquePtrTarget> Debug for UniquePtr<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.as_ref() {
            None => formatter.write_str("nullptr"),
            Some(value) => Debug::fmt(value, formatter),
        }
    }
}

impl<T: Display + UniquePtrTarget> Display for UniquePtr<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.as_ref() {
            None => formatter.write_str("nullptr"),
            Some(value) => Display::fmt(value, formatter),
        }
    }
}

#[macro_export]
macro_rules! genrs_unique_ptr {
    ($link_name:ident, $tp:ty) => {
        paste::paste! {
            impl $crate::UniquePtrTarget for $tp {
        unsafe fn __drop(this: &mut UniquePtr<$tp>) {
                    extern "C" {
            #[link_name=stringify!([<$link_name _destroy>])]
            fn func(this: &mut UniquePtr<$tp>);
                    }
                    func(this);
        }
            }
        }
    };
}

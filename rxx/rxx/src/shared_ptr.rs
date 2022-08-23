use crate::weak_ptr::{WeakPtr, WeakPtrTarget};
use core::fmt::{self, Debug, Display};
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use core::pin::Pin;
use std::mem::MaybeUninit;

pub trait SharedPtrTarget where Self: Sized {
    unsafe fn __drop(this: &mut SharedPtr<Self>);
    unsafe fn __clone(this: &SharedPtr<Self>, out: *mut SharedPtr<Self>);
}

#[repr(C)]
pub struct SharedPtr<T: SharedPtrTarget> {
    ptr: *mut T,
    ctrl: *mut T,
    pd: PhantomData<T>,
}

impl<T: SharedPtrTarget> SharedPtr<T> {
    pub fn get_ptr(&self) -> *const T {
        self.ptr as *const T
    }

    pub fn null() -> Self {
        SharedPtr {
            ptr: std::ptr::null_mut(),
            ctrl: std::ptr::null_mut(),
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

    pub fn downgrade(&self) -> WeakPtr<T>
    where
        T: WeakPtrTarget,
    {
        let mut out = MaybeUninit::<WeakPtr<T>>::uninit();
        unsafe {
            T::__downgrade(self,out.as_mut_ptr());
            out.assume_init()
        }
    }
}

unsafe impl<T> Send for SharedPtr<T> where T: Send + SharedPtrTarget {}
unsafe impl<T> Sync for SharedPtr<T> where T: Sync + SharedPtrTarget {}

impl<T: SharedPtrTarget> Clone for SharedPtr<T> {
    fn clone(&self) -> Self {
        let mut out = MaybeUninit::<Self>::uninit();
        unsafe {
            T::__clone(
		self,
                out.as_mut_ptr(),
            );
            out.assume_init()
        }
    }
}

impl<T: SharedPtrTarget> Drop for SharedPtr<T> {
    fn drop(&mut self) {
        unsafe {
            T::__drop(self)
        }
    }
}

impl<T: SharedPtrTarget> Deref for SharedPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self.as_ref() {
            Some(target) => target,
            None => panic!(
                "called deref on a null SharedPtr<{}>",
                std::any::type_name::<T>(),
            ),
        }
    }
}

impl<T: SharedPtrTarget + Unpin> DerefMut for SharedPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self.as_mut() {
            Some(target) => target,
            None => panic!(
                "called deref_mut on a null SharedPtr<{}>",
                std::any::type_name::<T>(),
            ),
        }
    }
}

impl<T: Debug + SharedPtrTarget> Debug for SharedPtr<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.as_ref() {
            None => formatter.write_str("nullptr"),
            Some(value) => Debug::fmt(value, formatter),
        }
    }
}

impl<T: Display + SharedPtrTarget> Display for SharedPtr<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.as_ref() {
            None => formatter.write_str("nullptr"),
            Some(value) => Display::fmt(value, formatter),
        }
    }
}

#[macro_export]
macro_rules! genrs_shared_ptr {
    ($link_name:ident, $tp:ty) => {
        paste::paste! {
            impl $crate::SharedPtrTarget for $tp {
            unsafe fn __drop(this: &mut SharedPtr<$tp>) {
                extern "C" {
                #[link_name=stringify!([<$link_name _delete>])]
                fn func(this: &mut SharedPtr<$tp>);
                }
                func(this);
            }

            unsafe fn __clone(this: &SharedPtr<$tp>, out: *mut SharedPtr<Self>) {
                extern "C" {
                    #[link_name=stringify!([<$link_name _clone>])]
                    fn func(this: &SharedPtr<$tp>, out: *mut SharedPtr<$tp>);
                }
                func(this, out);
            }
            }
        }
    };
}

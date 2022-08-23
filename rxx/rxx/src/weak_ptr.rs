use crate::shared_ptr::{SharedPtr, SharedPtrTarget};
use core::ffi::c_void;
use core::fmt::{self, Debug};
use core::marker::PhantomData;
use core::mem::MaybeUninit;

pub trait WeakPtrTarget {
    unsafe fn __drop(this: *mut c_void);
    unsafe fn __clone(this: *const c_void, new: *mut c_void);
    unsafe fn __downgrade(shared: *const c_void, weak: *mut c_void);
    unsafe fn __upgrade(weak: *const c_void, shared: *mut c_void);
}

#[repr(C)]
pub struct WeakPtr<T: WeakPtrTarget> {
    repr: [*mut c_void; 2],
    pd: PhantomData<T>,
}

impl<T: WeakPtrTarget> WeakPtr<T> {
    pub fn null() -> Self {
        WeakPtr {
            repr: [std::ptr::null_mut(); 2],
            pd: PhantomData,
        }
    }

    pub fn is_null(&self) -> bool {
        self.repr[0].is_null()
    }

    pub fn upgrade(&self) -> SharedPtr<T>
    where
        T: SharedPtrTarget,
    {
        let mut out = MaybeUninit::<SharedPtr<T>>::uninit();
        unsafe {
            T::__upgrade(
                self as *const Self as *const c_void,
                out.as_mut_ptr().cast(),
            );
            out.assume_init()
        }
    }
}

unsafe impl<T> Send for WeakPtr<T> where T: Send + Sync + WeakPtrTarget {}
unsafe impl<T> Sync for WeakPtr<T> where T: Send + Sync + WeakPtrTarget {}

impl<T: WeakPtrTarget> Clone for WeakPtr<T> {
    fn clone(&self) -> Self {
        let mut out = MaybeUninit::<Self>::uninit();
        unsafe {
            T::__clone(
                self as *const Self as *const c_void,
                out.as_mut_ptr().cast(),
            );
            out.assume_init()
        }
    }
}

impl<T: WeakPtrTarget> Drop for WeakPtr<T> {
    fn drop(&mut self) {
        unsafe {
            T::__drop(self as *mut Self as *mut c_void);
        }
    }
}

impl<T> Debug for WeakPtr<T>
where
    T: Debug + WeakPtrTarget + SharedPtrTarget,
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.upgrade(), formatter)
    }
}

#[macro_export]
macro_rules! genrs_weak_ptr {
    ($link_name:ident, $tp:ty) => {
        paste::paste! {
            impl $crate::WeakPtrTarget for $tp {
            unsafe fn __drop(this: *mut core::ffi::c_void) {
                extern "C" {
                #[link_name=stringify!([<$link_name _delete>])]
                fn func(this: *mut core::ffi::c_void);
                }
                func(this);
            }

            unsafe fn __clone(this: *const core::ffi::c_void, out: *mut core::ffi::c_void) {
                extern "C" {
                #[link_name=stringify!([<$link_name _clone>])]
                fn func(this: *const core::ffi::c_void, out: *mut core::ffi::c_void);
                }
                func(this, out);
            }

            unsafe fn __downgrade(shared: *const core::ffi::c_void, weak: *mut core::ffi::c_void) {
                extern "C" {
                #[link_name=stringify!([<$link_name _downgrade>])]
                fn func(shared: *const core::ffi::c_void, weak: *mut core::ffi::c_void);
                }
                func(shared, weak);
            }

            unsafe fn __upgrade(weak: *const core::ffi::c_void, shared: *mut core::ffi::c_void) {
                extern "C" {
                #[link_name=stringify!([<$link_name _upgrade>])]
                fn func(weak: *const core::ffi::c_void, shared: *mut core::ffi::c_void);
                }
                func(weak, shared);
            }
            }
        }
    };
}

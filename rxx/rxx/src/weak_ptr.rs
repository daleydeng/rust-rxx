use crate::shared_ptr::{SharedPtr, SharedPtrTarget};
use core::ffi::c_void;
use core::fmt::{self, Debug};
use core::marker::PhantomData;
use core::mem::MaybeUninit;

pub trait WeakPtrTarget where Self: Sized + SharedPtrTarget {
    unsafe fn __drop(this: &mut WeakPtr<Self>);
    unsafe fn __clone(this: &WeakPtr<Self>, new: *mut WeakPtr<Self>);
    unsafe fn __downgrade(shared: &SharedPtr<Self>, weak: *mut WeakPtr<Self>);
    unsafe fn __upgrade(weak: &WeakPtr<Self>, shared: *mut SharedPtr<Self>);
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
            T::__upgrade(self, out.as_mut_ptr());
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
            WeakPtrTarget::__clone(self, out.as_mut_ptr());
            out.assume_init()
        }
    }
}

impl<T: WeakPtrTarget> Drop for WeakPtr<T> {
    fn drop(&mut self) {
        unsafe {
            WeakPtrTarget::__drop(self);
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
		unsafe fn __drop(this: &mut WeakPtr<Self>) {
                    extern "C" {
			#[link_name=stringify!([<$link_name _destroy>])]
			fn func(this: &mut WeakPtr<$tp>);
                    }
                    func(this);
		}

		unsafe fn __clone(this: &WeakPtr<Self>, out: *mut WeakPtr<Self>) {
                    extern "C" {
			#[link_name=stringify!([<$link_name _clone>])]
			fn func(this: &WeakPtr<$tp>, out: *mut WeakPtr<$tp>);
                    }
                    func(this, out);
		}

		unsafe fn __downgrade(shared: &SharedPtr<Self>, weak: *mut WeakPtr<Self>) {
                    extern "C" {
			#[link_name=stringify!([<$link_name _downgrade>])]
			fn func(shared: &SharedPtr<$tp>, weak: *mut WeakPtr<$tp>);
                    }
                    func(shared, weak);
		}

		unsafe fn __upgrade(weak: &WeakPtr<Self>, shared: *mut SharedPtr<Self>) {
                    extern "C" {
			#[link_name=stringify!([<$link_name _upgrade>])]
			fn func(weak: &WeakPtr<$tp>, shared: *mut SharedPtr<$tp>);
                    }
                    func(weak, shared);
		}
            }
        }
    };
}

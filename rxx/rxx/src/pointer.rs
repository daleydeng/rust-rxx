use core::fmt::{self, Debug, Display};
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use core::pin::Pin;

pub trait PointerDrop {
    unsafe fn __drop(&mut self) {}
}

#[repr(C)]
pub struct Pointer<T: PointerDrop> {
    pub ptr: *mut T,
    pub _pd: PhantomData<T>,
}

impl<T: PointerDrop> Default for Pointer<T> {
    fn default() -> Self {
	Self {
	    ptr: std::ptr::null_mut(),
	    _pd: Default::default(),
	}
    }
}
impl<T: PointerDrop> Pointer<T> {
    pub fn get_ptr(&self) -> *const T {
        self.ptr as *const T
    }

    pub fn null() -> Self {
        Pointer {
            ptr: std::ptr::null_mut(),
	    ..Default::default()
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
                "called pin_mut on a null Pointer<{}>",
                std::any::type_name::<T>(),
            ),
        }
    }

    pub unsafe fn from_raw(raw: *mut T) -> Self {
        Self {
            ptr: raw,
            _pd: PhantomData,
        }
    }

    pub fn into_raw(self) -> *mut T {
        let ptr = self.ptr;
        std::mem::forget(self);
        ptr
    }

}

unsafe impl<T> Send for Pointer<T> where T: Send + PointerDrop {}
unsafe impl<T> Sync for Pointer<T> where T: Sync + PointerDrop {}

impl<T: PointerDrop> Deref for Pointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self.as_ref() {
            Some(target) => target,
            None => panic!(
                "called deref on a null Pointer<{}>",
                std::any::type_name::<T>(),
            ),
        }
    }
}

impl<T: Unpin + PointerDrop> DerefMut for Pointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self.as_mut() {
            Some(target) => target,
            None => panic!(
                "called deref_mut on a null Pointer<{}>",
                std::any::type_name::<T>(),
            ),
        }
    }
}

impl<T: PointerDrop + Debug> Debug for Pointer<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.as_ref() {
            None => formatter.write_str("nullptr"),
            Some(value) => Debug::fmt(value, formatter),
        }
    }
}

impl<T: PointerDrop + Display> Display for Pointer<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.as_ref() {
            None => formatter.write_str("nullptr"),
            Some(value) => Display::fmt(value, formatter),
        }
    }
}

impl<T: PointerDrop> Drop for Pointer<T> {
    fn drop(&mut self) {
	unsafe { T::__drop(&mut *self.ptr) }
    }
}

#[macro_export]
macro_rules! genrs_pointer_drop {
    ($link_name:ident, $tp:ty) => {
        impl PointerDrop for $tp {
	    unsafe fn __drop(&mut self) {
                extern "C" {
		    #[link_name=stringify!($link_name)]
		    fn func(this: *mut $tp);
                }
                func(self);
	    }
	}
    };
}

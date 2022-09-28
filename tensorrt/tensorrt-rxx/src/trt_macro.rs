#[macro_export]
macro_rules! impl_wrapper_lt {
    ($name:ident, $ffi_ty:ty) => {
        paste::paste! {
            #[repr(C)]
            pub struct $name<'a> {
		_obj: $ffi_ty,
		_pd: std::marker::PhantomData<&'a ()>
            }
        }
    };

    ($name:ident) => {
        impl_wrapper_lt!($name, ffi::$name);
    };
}

#[macro_export]
macro_rules! impl_raii_lt {
    ($name:ident, $ffi_ty:ty) => {
        paste::paste! {
	    impl_wrapper_lt!($name, $ffi_ty);
            genrs_pointer_drop!([<tensorrt_rxx_ $name _delete>], $name<'_>);
        }
    };

    ($name:ident) => {
        impl_raii_lt!($name, ffi::$name);
    };
}

#[macro_export]
macro_rules! impl_wrapper {
    ($name:ident, $ffi_ty:ty) => {
        paste::paste! {
            #[repr(C)]
            pub struct $name {
		_obj: $ffi_ty,
            }
        }
    };

    ($name:ident) => {
        impl_wrapper!($name, ffi::$name);
    };
}

#[macro_export]
macro_rules! impl_raii {
    ($name:ident, $ffi_ty:ty) => {
        paste::paste! {
	    impl_wrapper!($name, $ffi_ty);
            genrs_pointer_drop!([<tensorrt_rxx_ $name _delete>], $name);
        }
    };

    ($name:ident) => {
        impl_raii!($name, ffi::$name);
    };
}

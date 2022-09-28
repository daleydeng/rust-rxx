#[macro_export]
macro_rules! impl_wrapper {
    ($name:ident, $ffi_ty:ty) => {
        paste::paste! {
            #[repr(transparent)]
            pub struct $name<'a> {
		_obj: $ffi_ty,
		_pd: std::marker::PhantomData<&'a ()>
            }
        }
    };

    ($name:ident) => {
        impl_wrapper!($name, ffi::$name);
    };
}

#[macro_export]
macro_rules! impl_del {
    ($name:ident, $ffi_ty:ty) => {
        paste::paste! {
	    impl_wrapper!($name, $ffi_ty);
            genrs_pointer_drop!([<tensorrt_rxx_ $name _delete>], $name<'_>);
        }
    };

    ($name:ident) => {
        impl_del!($name, ffi::$name);
    };
}

#[macro_export]
macro_rules! impl_interface {
    ($name:ident, $ffi_ty:ty) => {
        paste::paste! {
            #[repr(transparent)]
            pub struct $name<'a> {
            _obj: $ffi_ty,
            _pd: std::marker::PhantomData<&'a ()>
            }
            genrs_pointer_drop!([<tensorrt_rxx_ $name _delete>], $name<'_>);
        }
    };

    ($name:ident) => {
        impl_interface!($name, ffi::$name);
    };
}

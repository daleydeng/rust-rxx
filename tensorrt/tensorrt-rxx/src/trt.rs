use std::marker::PhantomData;
use crate::ILogger;
use rxx::*;
use rxx_macro::*;
use tensorrt_sys::nvinfer1 as ffi;

macro_rules! impl_interface {
    ($name:ident, $ffi_ty:ty) => {
	paste::paste! {
	    #[repr(transparent)]
	    pub struct $name<'a> {
		_obj: $ffi_ty,
		_pd: PhantomData<&'a ()>
	    }
	    genrs_pointer_drop!([<tensorrt_rxx_ $name _delete>], $name<'_>);
	}
    };

    ($name:ident) => {
	impl_interface!($name, ffi::$name);
    };
}

impl_interface!(INetworkDefinition);
impl_interface!(IBuilderConfig);
impl_interface!(IBuilder);

genrs_fn!(
    #[ffi(link_name="tensorrt_rxx_createInferBuilder", new_ptr)]
    pub fn create_infer_builder<'a>(logger: &mut ILogger<'a>) -> CxxPointer<IBuilder<'a>> {}

    impl<'a> IBuilder<'a> {
	#[ffi(link_name="tensorrt_rxx_IBuilder_createNetworkV2", new_ptr)]
	pub fn create_network_v2(&mut self, flags: ffi::NetworkDefinitionCreationFlag) -> CxxPointer<INetworkDefinition<'a>> {}

    }
);

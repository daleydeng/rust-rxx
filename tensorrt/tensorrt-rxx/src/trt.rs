use std::marker::PhantomData;
use crate::ILogger;
use rxx::*;
use rxx_macro::*;
use tensorrt_sys::nvinfer1 as ffi;

#[repr(transparent)]
pub struct IBuilder<'a> {
    _obj: ffi::IBuilder,
    _pd: PhantomData<&'a ()>
}

genrs_pointer_drop!(tensorrt_rxx_IBuilder_delete, IBuilder<'_>);

genrs_fn!(
    #[ffi(link_name="tensorrt_rxx_createInferBuilder", new_ptr)]
    pub fn create_infer_builder<'a>(logger: &mut ILogger<'a>) -> Pointer<IBuilder<'a>> {}
);

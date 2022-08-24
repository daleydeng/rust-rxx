use rxx::*;
use rxx_macro::*;
use tensorrt_sys::nvinfer1 as ffi;
use crate::ILogger;

impl_interface!(INetworkDefinition);
impl_interface!(IBuilderConfig);
impl_interface!(IBuilder);

genrs_fn!(
    #[ffi(link_name="tensorrt_rxx_createInferBuilder", new_ptr)]
    pub fn create_infer_builder<'a>(logger: &mut ILogger<'a>) -> CxxPointer<IBuilder<'a>> {}

    impl<'a> IBuilder<'a> {
	#[ffi(link_name="tensorrt_rxx_IBuilder_createNetworkV2", new_ptr)]
	pub fn create_network_v2(&mut self, flags: ffi::NetworkDefinitionCreationFlags) -> CxxPointer<INetworkDefinition<'a>> {}

	#[ffi(link_name="tensorrt_rxx_IBuilder_createBuilderConfig", new_ptr)]
	pub fn create_builder_config(&mut self) -> CxxPointer<IBuilderConfig<'a>> {}
    }
);

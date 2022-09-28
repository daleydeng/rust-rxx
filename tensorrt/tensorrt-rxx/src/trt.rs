use crate::ILogger;
use rxx::*;
use rxx_macro::*;
use tensorrt_sys::nvinfer1 as ffi;

impl_del!(INetworkDefinition);
impl_del!(IBuilderConfig);
impl_del!(IBuilder);
impl_wrapper!(ILayer);
impl_wrapper!(ITensor);

genrs_fn!(
    #[ffi(link_name="tensorrt_rxx_createInferBuilder", new_ptr)]
    pub fn create_infer_builder<'a>(logger: &mut ILogger<'a>) -> CxxPointer<IBuilder<'a>> {}

    #[ffi(link_prefix="tensorrt_rxx_IBuilderConfig_")]
    impl IBuilderConfig<'_> {

	pub fn clear_flag(&mut self, flag: ffi::BuilderFlag) {}
	pub fn set_flag(&mut self, flag: ffi::BuilderFlag) {}
	#[ffi(atomic)]
	pub fn get_flag(&self, flag: ffi::BuilderFlag) -> bool {}

	pub fn set_dla_core(&mut self, v: i32) {}
	#[ffi(atomic)]
	pub fn get_dla_core(&self) -> i32 {}

	pub fn set_default_device_type(&mut self, v: ffi::DeviceType) {}
	#[ffi(atomic)]
	pub fn get_default_device_type(&self) -> ffi::DeviceType {}

    }

    #[ffi(link_prefix="tensorrt_rxx_IBuilder_")]
    impl<'a> IBuilder<'a> {

	#[ffi(new_ptr)]
	pub fn create_network_v2(&mut self, flags: ffi::NetworkDefinitionCreationFlags) -> CxxPointer<INetworkDefinition<'a>> {}

	#[ffi(new_ptr)]
	pub fn create_builder_config(&mut self) -> CxxPointer<IBuilderConfig<'a>> {}

	#[ffi(atomic)]
	pub fn get_nb_dla_cores(&self) -> i32 {}

    }
    #[ffi(link_prefix="tensorrt_rxx_INetworkDefinition_")]
    impl<'a> INetworkDefinition<'a> {
	#[ffi(atomic)]
	pub fn get_nb_layers(&self) -> i32 {}

	#[ffi(atomic, link_name="{LP}get_layer")]
	pub unsafe fn __get_layer(&self, index: i32) -> *mut ILayer<'a> {}
	pub fn get_layer(&self, index: i32) -> Option<&ILayer<'a>> {
	    unsafe {self.__get_layer(index).as_ref() }
	}
	pub fn get_layer_mut(&mut self, index: i32) -> Option<&mut ILayer<'a>> {
	    unsafe {self.__get_layer(index).as_mut() }
	}

    }

    #[ffi(link_prefix="tensorrt_rxx_ILayer_")]
    impl<'a> ILayer<'a> {
	#[ffi(atomic)]
	pub fn get_type(&self) -> ffi::LayerType {}

	#[ffi(atomic)]
	pub fn get_nb_outputs(&self) -> i32 {}

	#[ffi(atomic)]
	pub fn get_nb_inputs(&self) -> i32 {}

	#[ffi(atomic, link_name="{LP}get_input")]
	pub unsafe fn __get_input(&self, index: i32) -> *mut ITensor<'a> {}

	pub fn get_input(&self, index: i32) -> Option<&ITensor<'a>> {
	    unsafe {self.__get_input(index).as_ref() }
	}
	pub fn get_input_mut(&mut self, index: i32) -> Option<&mut ITensor<'a>> {
	    unsafe {self.__get_input(index).as_mut() }
	}

	#[ffi(atomic, link_name="{LP}get_output")]
	pub unsafe fn __get_output(&self, index: i32) -> *mut ITensor<'a> {}
	pub fn get_output(&self, index: i32) -> Option<&ITensor<'a>> {
	    unsafe {self.__get_output(index).as_ref() }
	}
	pub fn get_output_mut(&mut self, index: i32) -> Option<&mut ITensor<'a>> {
	    unsafe {self.__get_output(index).as_mut() }
	}


    }

    #[ffi(link_prefix="tensorrt_rxx_ITensor_")]
    impl<'a> ITensor<'a> {
	#[ffi(atomic)]
	pub fn dynamic_range_is_set(&self) -> bool {}

	#[ffi(atomic)]
	pub fn set_dynamic_range(&mut self, min: f32, max: f32) -> bool {}
    }
);

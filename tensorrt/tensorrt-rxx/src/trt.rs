use crate::{ILogger, TrtError};
use cuda_rs as cuda_ffi;
use rxx::*;
use rxx_macro::*;
use std::ffi::{c_char, c_void, CStr, CString};
use tensorrt_sys::nvinfer1 as ffi;

impl_raii!(IHostMemory);
impl_raii_lt!(INetworkDefinition);
impl_raii_lt!(IBuilderConfig);
impl_raii_lt!(IBuilder);
impl_wrapper_lt!(ILayer);
impl_wrapper_lt!(ITensor);

impl_raii_lt!(IRuntime);
impl_raii_lt!(ICudaEngine);
impl_raii_lt!(IExecutionContext);

genrs_fn!(
    #[ffi(link_prefix = "tensorrt_rxx_IHostMemory_")]
    impl IHostMemory {
        #[ffi(atomic)]
        fn data(&self) -> *mut () {}
        #[ffi(atomic)]
        pub fn size(&self) -> usize {}
        #[ffi(atomic, link_name = "{LP}type")]
        pub fn type_(&self) -> ffi::DataType {}
    }

    #[ffi(link_prefix = "tensorrt_rxx_", new_ptr)]
    pub fn create_infer_builder<'a>(logger: &mut ILogger<'a>) -> CxxPointer<IBuilder<'a>> {}

    #[ffi(link_prefix = "tensorrt_rxx_", new_ptr)]
    pub fn create_infer_runtime<'a>(logger: &mut ILogger<'a>) -> CxxPointer<IRuntime<'a>> {}

    #[ffi(link_prefix = "tensorrt_rxx_IBuilderConfig_")]
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

        pub fn set_profile_stream(&mut self, v: &cuda_ffi::Stream) {}

        // lifetime is questioned
        #[ffi(atomic)]
        pub fn get_profile_stream(&self) -> *mut cuda_ffi::Stream {}
    }

    #[ffi(link_prefix = "tensorrt_rxx_IBuilder_")]
    impl<'a> IBuilder<'a> {
        #[ffi(new_ptr)]
        pub fn create_network_v2(
            &mut self,
            flags: ffi::NetworkDefinitionCreationFlags,
        ) -> CxxPointer<INetworkDefinition<'a>> {
        }

        #[ffi(new_ptr)]
        pub fn create_builder_config(&mut self) -> CxxPointer<IBuilderConfig<'a>> {}

        #[ffi(atomic)]
        pub fn get_nb_dla_cores(&self) -> i32 {}

        #[ffi(atomic)]
        pub fn build_serialized_network(
            &mut self,
            network: &mut INetworkDefinition,
            config: &mut IBuilderConfig,
        ) -> CxxPointer<IHostMemory> {
        }
    }

    #[ffi(link_prefix = "tensorrt_rxx_INetworkDefinition_")]
    impl<'a> INetworkDefinition<'a> {
        #[ffi(atomic)]
        pub fn get_nb_layers(&self) -> i32 {}

        #[ffi(atomic, link_name = "{LP}get_layer")]
        pub unsafe fn __get_layer(&self, index: i32) -> *mut ILayer<'a> {}
        pub fn get_layer(&self, index: i32) -> Option<&ILayer<'a>> {
            unsafe { self.__get_layer(index).as_ref() }
        }
        pub fn get_layer_mut(&mut self, index: i32) -> Option<&mut ILayer<'a>> {
            unsafe { self.__get_layer(index).as_mut() }
        }

        #[ffi(atomic)]
        pub fn get_nb_outputs(&self) -> i32 {}
        #[ffi(atomic)]
        pub fn get_nb_inputs(&self) -> i32 {}

        #[ffi(atomic, link_name = "{LP}get_input")]
        pub unsafe fn __get_input(&self, index: i32) -> *mut ITensor<'a> {}

        pub fn get_input(&self, index: i32) -> Option<&ITensor<'a>> {
            unsafe { self.__get_input(index).as_ref() }
        }
        pub fn get_input_mut(&mut self, index: i32) -> Option<&mut ITensor<'a>> {
            unsafe { self.__get_input(index).as_mut() }
        }

        #[ffi(atomic, link_name = "{LP}get_output")]
        pub unsafe fn __get_output(&self, index: i32) -> *mut ITensor<'a> {}
        pub fn get_output(&self, index: i32) -> Option<&ITensor<'a>> {
            unsafe { self.__get_output(index).as_ref() }
        }
        pub fn get_output_mut(&mut self, index: i32) -> Option<&mut ITensor<'a>> {
            unsafe { self.__get_output(index).as_mut() }
        }
    }

    #[ffi(link_prefix = "tensorrt_rxx_ILayer_")]
    impl<'a> ILayer<'a> {
        #[ffi(atomic)]
        pub fn get_type(&self) -> ffi::LayerType {}

        #[ffi(atomic)]
        pub fn get_nb_outputs(&self) -> i32 {}

        #[ffi(atomic)]
        pub fn get_nb_inputs(&self) -> i32 {}

        #[ffi(atomic, link_name = "{LP}get_input")]
        pub unsafe fn __get_input(&self, index: i32) -> *mut ITensor<'a> {}

        pub fn get_input(&self, index: i32) -> Option<&ITensor<'a>> {
            unsafe { self.__get_input(index).as_ref() }
        }
        pub fn get_input_mut(&mut self, index: i32) -> Option<&mut ITensor<'a>> {
            unsafe { self.__get_input(index).as_mut() }
        }

        #[ffi(atomic, link_name = "{LP}get_output")]
        pub unsafe fn __get_output(&self, index: i32) -> *mut ITensor<'a> {}
        pub fn get_output(&self, index: i32) -> Option<&ITensor<'a>> {
            unsafe { self.__get_output(index).as_ref() }
        }
        pub fn get_output_mut(&mut self, index: i32) -> Option<&mut ITensor<'a>> {
            unsafe { self.__get_output(index).as_mut() }
        }
    }

    #[ffi(link_prefix = "tensorrt_rxx_ITensor_")]
    impl<'a> ITensor<'a> {
        #[ffi(atomic)]
        pub fn dynamic_range_is_set(&self) -> bool {}

        #[ffi(atomic)]
        pub fn set_dynamic_range(&mut self, min: f32, max: f32) -> bool {}

        #[ffi(atomic)]
        pub fn get_dimensions(&self) -> ffi::Dims32 {}
        pub fn set_dimensions(&mut self, dims: ffi::Dims32) {}
    }

    #[ffi(link_prefix = "tensorrt_rxx_IRuntime_")]
    impl<'a> IRuntime<'a> {
        #[ffi(atomic)]
        pub unsafe fn deserialize_cuda_engine(
            &mut self,
            blob: *const (),
            size: usize,
        ) -> CxxPointer<ICudaEngine<'a>> {
        }

        pub fn deserialize_cuda_engine_host_memory(
            &mut self,
            blob: &IHostMemory,
        ) -> CxxPointer<ICudaEngine<'a>> {
            unsafe { self.deserialize_cuda_engine(blob.data(), blob.size()) }
        }
    }

    #[ffi(link_prefix = "tensorrt_rxx_ICudaEngine_")]
    impl<'a> ICudaEngine<'a> {
        #[ffi(atomic)]
        pub fn create_execution_context(&mut self) -> CxxPointer<IExecutionContext<'a>> {}

        #[ffi(atomic)]
        pub fn has_implicit_batch_dimension(&self) -> bool {}

        #[ffi(atomic)]
        pub fn get_nb_bindings(&self) -> i32 {}

        #[ffi(atomic, link_name = "{LP}get_binding_index")]
        pub fn __get_binding_index(&self, name: *const c_char) -> i32 {}
        pub fn get_binding_index(&self, name: &str) -> i32 {
            let name = CString::new(name).unwrap();
            self.__get_binding_index(name.as_ptr())
        }

        #[ffi(atomic, link_name = "{LP}get_binding_name")]
        pub fn __get_binding_name(&self, index: i32) -> *const c_char {}
        pub fn get_binding_name(&self, index: i32) -> Option<String> {
            let name = self.__get_binding_name(index);
            if name.is_null() {
                None
            } else {
                unsafe { Some(CStr::from_ptr(name).to_str().unwrap().to_owned()) }
            }
        }

        #[ffi(atomic)]
        pub fn get_binding_dimensions(&self, index: i32) -> ffi::Dims {}

        #[ffi(atomic)]
        pub fn get_binding_data_type(&self, index: i32) -> ffi::DataType {}

        #[ffi(atomic)]
        pub fn get_binding_vectorized_dim(&self, index: i32) -> i32 {}

        #[ffi(atomic)]
        pub fn get_binding_components_per_element(&self, index: i32) -> i32 {}
    }

    #[ffi(link_prefix = "tensorrt_rxx_IExecutionContext_")]
    impl<'a> IExecutionContext<'a> {
        #[ffi(atomic)]
        pub fn get_binding_dimensions(&self, index: i32) -> ffi::Dims {}

        #[ffi(atomic, link_name = "{LP}execute_v2")]
        pub unsafe fn __execute_v2(&mut self, bindings: *const *mut c_void) -> bool {}
        pub fn execute_v2(
            &mut self,
            bindings: &mut Vec<cuda_ffi::DeviceMemory>,
        ) -> Result<(), TrtError> {
            unsafe {
                let bindings = bindings.iter().map(|i| i.as_ptr()).collect::<Vec<_>>();
                if self.__execute_v2(bindings.as_ptr()) {
                    Ok(())
                } else {
                    Err(TrtError::ExecutionFailed)
                }
            }
        }
    }
);

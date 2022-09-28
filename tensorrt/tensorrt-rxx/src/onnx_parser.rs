use crate::{ILogger, INetworkDefinition, Severity, TensorrtError};
use libc::c_char;
use rxx::*;
use rxx_macro::*;
use std::ffi::CString;
use std::path::Path;

use tensorrt_sys::nvonnxparser as onnx_parser;

impl_del!(OnnxIParser, onnx_parser::IParser);

#[repr(transparent)]
pub struct IParserError<'a> {
    _obj: onnx_parser::IParserError,
    _pd: std::marker::PhantomData<&'a ()>,
}

genrs_fn!(
    #[ffi(link_name="tensorrt_rxx_createOnnxParser", new_ptr)]
    pub fn create_onnx_parser<'a>(network: &mut INetworkDefinition<'a>, logger: &mut ILogger<'a>) -> CxxPointer<OnnxIParser<'a>> {}

    impl OnnxIParser<'_> {
    #[ffi(link_name="tensorrt_rxx_OnnxIParser_getNbErrors", atomic)]
    pub fn get_error_nr(&self) -> i32 {}

    #[ffi(link_name="tensorrt_rxx_OnnxIParser_clearErrors")]
    pub fn clear_errors(&mut self) {}

    pub fn parse_from_file(&mut self, fname: &Path, verbosity: Severity) -> Result<(), TensorrtError> {
        extern "C" {
        #[link_name="tensorrt_rxx_OnnxIParser_parseFromFile"]
        fn __func(this: &mut OnnxIParser<'_>, fname: *const c_char, verbosity: Severity) -> bool;
        }
        unsafe {
        let s = CString::new(fname.to_str().unwrap()).unwrap();
        let ret = __func(self, s.as_ptr(), verbosity);
        if ret {
            return Ok(())
        }
        Err(TensorrtError::OnnxParseFail)
        }
    }
    }
);

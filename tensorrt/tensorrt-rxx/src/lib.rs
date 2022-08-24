#![feature(default_free_fn, absolute_path)]
pub use tensorrt_sys::nvinfer1 as ffi;

pub mod wrapper;
pub use wrapper::*;

pub mod logger;
pub use logger::*;

#[macro_use]
pub mod trt_macro;
pub use trt_macro::*;

pub mod trt;
pub use trt::*;

pub mod onnx_parser;
pub use onnx_parser::*;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TensorrtError {
    #[error("onnx parse failed")]
    OnnxParseFail,
}

#[cfg(test)]
mod tests {
    use std::path::{self, Path};
    use super::*;

    #[test]
    fn test_callback() {
	let msg = "hello world";
	let mut rust_logger = Default::default();
	// rust cant infer here, need drop logger first, don't know why
	{
	    let mut logger = create_rust_logger(&mut rust_logger, RustLogger::log_callback);

	    log(&mut logger, Severity::kINFO, msg);
	}
	assert_eq!(rust_logger.last_msg, msg);
    }

    #[test]
    fn test_builder() {
	let mut rust_logger = Default::default();
	let mut logger = create_rust_logger(&mut rust_logger, RustLogger::log_callback);
	let mut builder = create_infer_builder(&mut logger);
	let mut network = builder.create_network_v2(1u32 << ffi::NetworkDefinitionCreationFlag::kEXPLICIT_BATCH as i32);
	let config = builder.create_builder_config();
	let mut parser = create_onnx_parser(&mut network, &mut logger);
	let p = Path::new("data/mnist.onnx");

	eprintln!("path {:?}", path::absolute(p).unwrap());
	parser.parse_from_file(p, Severity::kVERBOSE).expect("parse error");
    }

}

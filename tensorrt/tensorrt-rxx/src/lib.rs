#![feature(default_free_fn)]
pub use tensorrt_sys::nvinfer1 as ffi;

pub mod logger;
pub use logger::*;

pub mod trt;
pub use trt::*;

#[cfg(test)]
mod tests {
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
	let network = builder.create_network_v2(ffi::NetworkDefinitionCreationFlag::kEXPLICIT_BATCH);
    }

}

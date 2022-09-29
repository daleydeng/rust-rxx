#![feature(default_free_fn, absolute_path)]
pub use tensorrt_sys::nvinfer1 as ffi;

pub mod common;
pub use common::*;

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
pub enum TrtError {
    #[error("onnx parse failed")]
    OnnxParseFail,
    #[error("execution failed")]
    ExecutionFailed,
}

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
}

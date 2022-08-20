use rxx::*;
use tensorrt_sys::nvinfer1::ILogger;

#[repr(C)]
pub struct RustLogger{}

pub type LogFnType = extern "C" fn(obj: *mut libc::c_void, msg: *const libc::c_char);

genrs_fn!(pub fn create_rust_logger(obj: *mut libc::c_void, log_fn: &LogFnType) -> *mut ILogger, cret=atomic, ln=tensorrt_rxx_RustLogger_create);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_callback() {
    }
}

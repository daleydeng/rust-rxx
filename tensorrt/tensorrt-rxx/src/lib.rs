use rxx::*;
use tensorrt_sys::nvinfer1::ILogger;

#[repr(C)]
pub struct RustLogger{}

pub type LogFnType = extern "C" fn(obj: *mut libc::c_void, msg: *const libc::c_char);

// ILogger delete

rxx_macro::genrs_fn!(
    #[ffi(link_name="tensorrt_rxx_RustLogger_create", new_ptr)]
    pub fn create_rust_logger(obj: *mut libc::c_void, log_fn: &LogFnType) -> Box<ILogger> {}
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_callback() {
    }
}

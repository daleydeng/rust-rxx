use std::marker::PhantomData;
use std::ffi::{CStr, CString};
use libc::c_char;
use rxx::*;
use rxx_macro::*;
use tensorrt_sys::nvinfer1 as ffi;

pub type Severity = ffi::ILogger_Severity;

#[repr(transparent)]
pub struct ILogger<'a> {
    _obj: ffi::ILogger,
    _pd: PhantomData<&'a ()>
}

genrs_pointer_drop!(tensorrt_rxx_ILogger_delete, ILogger<'_>);

#[repr(C)]
pub struct RustLogger {
    pub severity: Severity,
    pub last_msg: String,
}

impl Default for RustLogger {
    fn default() -> Self {
	Self {
	    severity: Severity::kINFO,
	    last_msg: String::new(),
	}
    }
}

impl RustLogger {
    pub extern "C" fn log(&mut self, severity: Severity, msg: *const c_char) {
	if severity as i32 > self.severity as i32 {
	    return;
	}
	self.last_msg = unsafe { CStr::from_ptr(msg) }.to_string_lossy().into_owned();
    }
}

pub type LogFnType = extern "C" fn(logger: &mut RustLogger, severity: Severity, msg: *const c_char);

pub fn create_rust_logger<'a>(
    logger: &'a mut RustLogger,
    log_fn: LogFnType,
) -> Pointer<ILogger<'a>> {
    extern "C" {
        #[link_name = "tensorrt_rxx_RustLogger_create"]
        fn __func<'a>(logger: *mut (), log_fn: *const ()) -> *mut ILogger<'a>;
    }
    unsafe {
	let ptr = __func(logger as *mut RustLogger as *mut(), log_fn as *const ());
	Pointer::from_raw(ptr)
    }
}


genrs_fn!(
    #[ffi(link_name="tensorrt_rxx_log")]
    pub fn c_log(logger: &mut ILogger, severity: Severity, msg: *const c_char) {}
);

pub fn log(logger: &mut ILogger, severity: Severity, msg: &str) {
    let msg = CString::new(msg).unwrap();
    c_log(logger, severity, msg.as_ptr());
}
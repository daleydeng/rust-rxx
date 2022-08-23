use std::marker::PhantomData;
use std::ffi::{CStr, CString};
use libc::{c_char, c_void};
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
    severity: Severity,
    last_msg: *mut c_char,
}

impl Default for RustLogger {
    fn default() -> Self {
	Self {
	    severity: Severity::kINFO,
	    last_msg: std::ptr::null_mut(),
	}
    }
}

impl Drop for RustLogger {
    fn drop(&mut self) {
	self.free()
    }
}

impl RustLogger {

    pub fn free(&mut self) {
	unsafe {
	    if !self.last_msg.is_null() {
		libc::free(self.last_msg as *mut c_void);
		self.last_msg = std::ptr::null_mut();
	    }
	}
    }

    pub fn set_msg(&mut self, msg: *const c_char) {
	unsafe {
	    self.free();
	    self.last_msg = libc::strdup(msg);
	};
    }

    pub fn get_msg(&self) -> &str {
	if self.last_msg.is_null() {
	    return "";
	}
	unsafe {CStr::from_ptr(self.last_msg)}.to_str().unwrap()
    }

    pub extern "C" fn log(&mut self, severity: Severity, msg: *const c_char) {
	if severity as i32 > self.severity as i32 {
	    return;
	}
	self.set_msg(msg);
    }
}

pub type LogFnType = extern "C" fn(logger: &mut RustLogger, severity: Severity, msg: *const c_char);

genrs_fn!(
    #[ffi(link_name="tensorrt_rxx_RustLogger_create", new_ptr)]
    pub fn create_rust_logger<'a>(logger: &'a mut RustLogger, log_fn: LogFnType) -> Pointer<ILogger<'a>> {}
);

genrs_fn!(
    #[ffi(link_name="tensorrt_rxx_log")]
    pub fn c_log(logger: &mut ILogger, severity: Severity, msg: *const c_char) {}
);

pub fn log(logger: &mut ILogger, severity: Severity, msg: &str) {
    let msg = CString::new(msg).unwrap();
    c_log(logger, severity, msg.as_ptr());
}

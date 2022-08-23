#![feature(default_free_fn)]
mod logger;
pub use logger::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_callback() {
	let msg = "hello world";
	let mut rust_logger = Default::default();
	// rust cant infer here, need drop logger first, don't know why
	{
	    let mut logger = create_rust_logger(&mut rust_logger, RustLogger::log);

	    log(&mut logger, Severity::kINFO, msg);
	}
	assert_eq!(rust_logger.get_msg(), msg);
    }
}

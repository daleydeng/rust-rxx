#[macro_export]
macro_rules! genrs_drop {
    ($link_name:ident, $tp:ty) => {
	impl Drop for $tp {
	    fn drop(&mut self) {
		extern "C" {
		    #[link_name=stringify!($link_name)]
		    fn __func(this: &mut $tp);
		}
		unsafe {
		    __func(self);
		}
	    }
	}
    }
}

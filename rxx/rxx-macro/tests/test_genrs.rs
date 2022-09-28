use rxx_macro::genrs_fn;

pub struct ILayer<'a> {
    _pd: std::marker::PhantomData<&'a ()>,
}

struct IHostMemory {}

#[test]
fn test_genrs() {
    genrs_fn!(
	// #[ffi(link_prefix="tensorrt_rxx_ILayer_")]
	// impl<'a> ILayer<'a> {
        //     #[ffi(link_name="{LP}getNbOutputs", atomic)]
        //     pub fn get_nb_outputs(&self) -> i32 {}

        //     pub fn get_nb_inputs(&self) -> i32 {}
	// }

	#[ffi(link_prefix="aaa_")]
	pub fn hello() {}

	#[ffi(link_name="tensorrt_rxx_IHostMemory_")]
	impl IHostMemory {
            #[ffi(atomic)]
            pub fn data(&self) -> *mut () {}
	}
    );
}

fn main() {}

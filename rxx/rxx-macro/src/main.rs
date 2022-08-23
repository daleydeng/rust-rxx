// struct Dummy<'a> {}
use std::ops::Mul;

#[repr(C)]
struct SO2d {}

rxx_macro::genrs_fn!(
    // #[ffi(link_name="rxx_dummy_cpp_new_vector_i64")]
    // pub fn rxx_dummy_cpp_new_vector_i64(a: i32) -> Vec<i64> {}
    // pub fn rxx_dummy_cpp_add_vector_i64(a: &mut i64, i: i32) {}
    // #[ffi(atomic, test)]
    // pub fn rxx_dummy_cpp_addret_vector_i64(a: &mut i64, i: i32) -> i64 {}
    // #[ffi(atomic)]
    // pub fn rxx_dummy_cpp_getref_vector_i64<'a>(a: &'a i64, i: i32) -> &'a i64 {}
    // impl Vec<i64> {
    // 	// #[ffi(link_name="rxx_dummy_cpp_add_vector_i64")]
    // 	// pub fn add(&mut self, a: i32) {}

    // 	#[ffi(link_name="rxx_dummy_cpp_get_vector_i64")]
    // 	pub fn get1(&self) -> i64 {}
    // }

    // impl<'a> Dummy<'a> {
    // 	#[ffi(link_name="rxx_Dummy_get", atomic)]
    //     pub fn get(&self, idx: usize) -> i64 {}
    // }

    #[ffi(link_name="rxx_dummy_cpp_get_vector_i64", new_ptr)]
    pub fn get1() -> Box<i64> {}

    // impl SO2d {
    //     type Output = Self;
    //     #[ffi(link_name="mul", atomic)]
    //     fn mul(&self, other: &mut Self::Output) -> &Self::Output {}
    // }

    impl Mul<Self> for &SO2d {
	type Output = SO2d;
        #[ffi(link_name="sophus_rxx_SO2d_mul")]
        fn mul(self, rhs: Self) -> Self::Output {}
    }
);

fn main() {}

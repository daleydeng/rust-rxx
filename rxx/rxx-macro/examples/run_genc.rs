#![allow(unused)]
// struct Dummy<'a> {}
use rxx_build::*;
use std::{marker::PhantomData, ops::Mul};

struct Pointer<T> {
    pub ptr: *mut T,
    _pd: PhantomData<T>,
}

impl<T> Default for Pointer<T> {
    fn default() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
            _pd: Default::default(),
        }
    }
}

#[repr(C)]
struct SO2d {}

// rxx_macro::genrs_fn!(
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

// #[ffi(link_name = "rxx_dummy_i64_new", new_ptr)]
// fn dummy_i64_new() -> Pointer<i64> {};

// #[ffi(link_name="rxx_dummy_i64_new", new_ptr)]
// pub fn dummy_i64_new() -> Pointer<i64> {}

// impl SO2d {
//     type Output = Self;
//     #[ffi(link_name="mul", atomic)]
//     fn mul(&self, other: &mut Self::Output) -> &Self::Output {}
// }

// impl Mul<Self> for &SO2d {
// 	type Output = SO2d;
//     #[ffi(link_name="sophus_rxx_SO2d_mul")]
//     fn mul(self, rhs: Self) -> Self::Output {}
// }
// );
fn main() {
    let lp = "rxx_Dummy";

    // let a = rxx_macro::genc_fn!(
    //     #[ffi(link_name = "MapMut_Matrix3d_new")]
    // 	fn MapMut_fixed_new<Matrix3d, double>(data: *mut double) -> Eigen::Map<Matrix3d> {}

    // 	#[ffi(link_name="{lp}_cpp_new_vector_i64")]
    // 	fn dummy_cpp_new_vector_i64(a: int) -> std::vector<int64_t> {}

    // 	#[ffi(link_name="{lp}_cpp_add_vector_i64")]
    // 	fn dummy_cpp_add_vector_i64(val: &mut std::vector<int64_t>, n: int) {}

    // 	#[ffi(link_name="{lp}_cpp_addret_vector_i64", atomic)]
    // 	fn dummy_cpp_addret_vector_i64(val: &mut std::vector<int64_t>, n: int) -> int64_t {}

    // 	#[ffi(link_name="{lp}_cpp_get_vector_i64")]
    // 	fn dummy_cpp_get_vector_i64(val: &std::vector<int64_t>) -> int64_t {}

    // 	#[ffi(link_name="{lp}_cpp_getvoid_vector_i64")]
    // 	fn dummy_cpp_getvoid_vector_i64(val: &std::vector<int64_t>, a: int) {}

    // 	#[ffi(link_name="{lp}_cpp_getref_vector_i64", atomic)]
    // 	fn dummy_cpp_getref_vector_i64(val: &std::vector<int64_t>, idx: int) -> &int64_t {}

    // 	#[ffi(link_name="{lp}_i64_delete", ns="rxx")]
    // 	fn delete_pointer(ptr: *mut int64_t) {}

    // 	#[ffi(link_prefix="rxx_Dummy")]
    // 	impl Dummy {
    // 	    #[ffi(atomic, link_name="rxx_Dummy_get")]
    // 	    fn get(&self, idx: size_t) -> int64_t {}

    // 	    #[ffi(atomic)]
    // 	    fn get_mut(&mut self, idx: size_t) -> &mut int64_t {}

    // 	    fn add(&mut self, val: int64_t) {}
    // 	}
    // );
}

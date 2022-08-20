rxx_macro::genrs_fn!(
    #[ffi(link_name="rxx_dummy_cpp_new_vector_i64")]
    pub fn rxx_dummy_cpp_new_vector_i64(a: i32) -> Vec<i64> {}
);
rxx_macro::genrs_fn!(pub fn rxx_dummy_cpp_add_vector_i64(a: &mut Vec<i64>, i: i32) {});

rxx_macro::genrs_fn!(
    #[ffi(atomic, test)]
    pub fn rxx_dummy_cpp_addret_vector_i64(a: &mut Vec<i64>, i: i32) -> i64 {}
); // match with build.rs

fn main() {}

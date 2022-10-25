use std::collections::HashMap;

#[test]
fn test_genc() {
    let s = rxx_macro::genc_fn!(
    let tpl_vars = HashMap::from([
        ("lp".to_owned(), "rxx"),
    ]);

        #[ffi(link_name = "{lp}_MapMut_Matrix3d_new")]
        fn MapMut_fixed_new<Matrix3d, double>(data: *mut double) -> Eigen::Map<Matrix3d> {}

        #[ffi(link_name = "{lp}_Matrix3d_print")]
        fn Matrix3d_print(self: &Matrix3d) {}

        #[ffi(link_name = "{lp}_dummy_cpp_getref_vector_i64", atomic)]
        fn dummy_cpp_getref_vector_i64(val: &std::vector<int64_t>, idx: int) -> &int64_t {}

        #[ffi(link_name = "{lp}_dummy_i64_delete", ns = "rxx")]
        fn delete_pointer<int64_t>(ptr: *mut int64_t) {}

        #[ffi(link_prefix = "{lp}_Dummy")]
        impl Dummy {
            #[ffi(atomic)]
            fn get(&self, idx: size_t) -> int64_t {}

            #[ffi(atomic)]
            fn get_mut(&mut self, idx: size_t) -> &mut int64_ {}
        }
    );

    assert_eq!(
        s[0],
        r#"
extern "C" void rxx_MapMut_Matrix3d_new(double * data, Eigen :: Map < Matrix3d > *__ret) noexcept {
    Eigen :: Map < Matrix3d > (*__func)(double * data) = MapMut_fixed_new< Matrix3d, double >;
    new (__ret) (Eigen :: Map < Matrix3d >)(__func(data));
}
"#
        .trim_start()
    );

    assert_eq!(
        s[1],
        r#"
extern "C" void rxx_Matrix3d_print(Matrix3d const & self) noexcept {
    void (*__func)(Matrix3d const & self) = Matrix3d_print;
    __func(self);
}
"#
        .trim_start()
    );
}

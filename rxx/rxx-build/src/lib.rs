#![feature(default_free_fn)]
use anyhow::Result;
use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;
use std::collections::HashSet;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Once;

pub mod genc;
pub use genc::*;

const NAME: &str = "rxx";

const C_HDR: &str = include_str!("../include/wrapper.hh");
const C_SRC: &str = include_str!("../csrc/wrapper.cc");

pub fn dump_file_once(fname: &Path, source: &str, once: &Once) {
    let inc_dir = fname.parent().unwrap();
    fs::create_dir_all(inc_dir).unwrap();
    once.call_once(|| {
        let mut file = fs::File::create(fname).unwrap();
        file.write_all(source.as_bytes()).unwrap();
    });
}

pub fn dump_headers_rxx(inc_dir: &Path) -> Result<HashSet<PathBuf>> {
    static ONCE: Once = Once::new();

    let inc_dir = inc_dir.join(NAME);

    let wrapper_f = inc_dir.join("wrapper.hh");
    dump_file_once(&wrapper_f, C_HDR, &ONCE);
    Ok(HashSet::from([inc_dir]))
}

pub fn render_c_template(tpl: &str, items: impl Serialize) -> Result<String> {
    let hb = Handlebars::new();

    Ok(hb.render_template(
        tpl,
        &json!({
            "items": items,
        }),
    )?)
}

pub fn genc_file_rxx(items: impl Serialize) -> Result<String> {
    let tpl = r#"
#include <rxx/wrapper.hh>

{{#each items}}
{{{this}}}
{{/each}}
"#
    .trim_start();
    render_c_template(tpl, items)
}

pub fn dump_sources_rxx(src_dir: &Path) -> Result<HashSet<PathBuf>> {
    static ONCE_SRC: Once = Once::new();
    static ONCE_FFI: Once = Once::new();

    let src_dir = src_dir.join(NAME);
    let wrapper_f = src_dir.join("wrapper.cc");
    dump_file_once(&wrapper_f, C_SRC, &ONCE_SRC);

    let ffi_f = src_dir.join("ffi.cc");
    let ffi_code = genc_file_rxx(&[
        &genc_unique_ptr("rxx_unique_string", "std::unique_ptr<std::string>"),
        &genc_shared_ptr("rxx_shared_string", "std::shared_ptr<std::string>"),
        &genc_weak_ptr(
            "rxx_weak_string",
            "std::weak_ptr<std::string>",
            "std::shared_ptr<std::string>",
        ),
    ])
    .unwrap();

    dump_file_once(&ffi_f, &ffi_code, &ONCE_FFI);
    Ok(HashSet::from([wrapper_f, ffi_f]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::default::default;

    #[test]
    fn test_fn() {
        let s = genc_fn(
            "MapMut_Matrix3d_new",
            FnSig {
                c_fn: "MapMut_fixed_new<Matrix3d, double>",
                ret_type: ReturnType::Object("Eigen::Map<Matrix3d>"),
                args: &[("double *", "data")],
                ..default()
            },
        );

        assert_eq!(
            s,
            r#"
extern "C" void MapMut_Matrix3d_new(double * data, Eigen::Map<Matrix3d> *__ret) noexcept {
    Eigen::Map<Matrix3d> (*__func)(double * data) = MapMut_fixed_new<Matrix3d, double>;
    new (__ret) (Eigen::Map<Matrix3d>)(__func(data));
}
"#
            .trim_start()
        );

        let s = genc_fn(
            "rxx_Matrix3d_print",
            FnSig {
                c_fn: "Matrix3d_print",
                args: &[("Matrix3d const &", "self")],
                ..default()
            },
        );

        assert_eq!(
            s,
            r#"
extern "C" void rxx_Matrix3d_print(Matrix3d const & self) noexcept {
    void (*__func)(Matrix3d const & self) = Matrix3d_print;
    __func(self);
}
"#
            .trim_start()
        );
    }

    #[test]
    fn test_std() {
        let s = genc_unique_ptr("rxx_unique_string", "std::unique_ptr<std::string>");
        assert_eq!(
            s,
            r#"
extern "C" void rxx_unique_string_destroy(std::unique_ptr<std::string> &self) noexcept {
    rxx::destroy(self);
}
"#
            .trim_start()
        );

        let s = genc_shared_ptr("rxx_shared_string", "std::shared_ptr<std::string>");
        assert_eq!(s, r#"
extern "C" void rxx_shared_string_destroy(std::shared_ptr<std::string> &self) noexcept {
    rxx::destroy(self);
}

extern "C" void rxx_shared_string_clone(const std::shared_ptr<std::string> &self, std::shared_ptr<std::string> *out) noexcept {
    rxx::shared_ptr_clone(self, out);
}
"#.trim_start());

        let s = genc_weak_ptr(
            "rxx_weak_string",
            "std::weak_ptr<std::string>",
            "std::shared_ptr<std::string>",
        );
        assert_eq!(s, r#"
extern "C" void rxx_weak_string_destroy(std::weak_ptr<std::string> &self) noexcept {
    rxx::destroy(self);
}

extern "C" void rxx_weak_string_clone(const std::weak_ptr<std::string> &self, std::weak_ptr<std::string> *out) noexcept {
    rxx::weak_ptr_clone(self, out);
}

extern "C" void rxx_weak_string_upgrade(const std::weak_ptr<std::string> &self, std::shared_ptr<std::string> *out) {
    rxx::weak_ptr_upgrade(self, out);
}

extern "C"  void rxx_weak_string_downgrade(const std::shared_ptr<std::string> &self, std::weak_ptr<std::string> *out) {
    rxx::weak_ptr_downgrade(self, out);
}
"#.trim_start());

        let s = genc_vector(
            "rxx_vector_string",
            "std::vector<std::string>",
            "std::string",
        );
        assert_eq!(s,  r#"
extern "C" void rxx_vector_string_destroy(const std::vector<std::string> &self) {
    rxx::destroy(self);
}

extern "C" std::size_t rxx_vector_string_size(const std::vector<std::string> &self) {
    return rxx::vector_size(self);
}

extern "C" const std::string& rxx_vector_string_get(const std::vector<std::string> &self, size_t pos) {
    return rxx::vector_get(self, pos);
}

extern "C" std::string& rxx_vector_string_get_mut(std::vector<std::string> &self, size_t pos) {
    return rxx::vector_get_mut(self, pos);
}

extern "C" void rxx_vector_string_push_back(std::vector<std::string> &self, std::string &val) {
    return rxx::vector_push_back(self, val);
}

extern "C" void rxx_vector_string_pop_back(std::vector<std::string> &self, std::string *out) {
    rxx::vector_pop_back(self, out);
}
"#.trim_start());

        let s = genc_get_val("get_global", ReturnType::Atomic("int"), "test");
        assert_eq!(
            s,
            r#"
extern "C" int get_global() noexcept {
    return test;
}
"#
            .trim_start()
        );
    }
}

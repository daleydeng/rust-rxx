use std::path::{Path, PathBuf};
use std::io::Write;
use std::fs;
use std::sync::Once;
use std::collections::HashSet;
use anyhow::Result;
use handlebars::Handlebars;
use serde_json::json;
pub mod genc;
pub use genc::*;

const NAME: &str = "rxx";

const C_HDR: &str = include_str!("../include/wrapper.hh");
const C_SRC: &str = include_str!("../csrc/wrapper.cc");

pub fn dump_headers_rxx(inc_dir: &Path) -> Result<HashSet<PathBuf>> {
    static START: Once = Once::new();

    let inc_dir = inc_dir.join(NAME);
    fs::create_dir_all(&inc_dir)?;

    let wrapper_f = inc_dir.join("wrapper.hh");
    START.call_once(|| {
	let mut file = fs::File::create(&wrapper_f).unwrap();
	file.write_all(C_HDR.as_bytes()).unwrap();
    });
    Ok(HashSet::from([inc_dir]))
}

pub fn genc_file_rxx(gen_types: &[&str]) -> String {
    let tpl = r#"
#include <rxx/wrapper.hh>

{{#each code}}
{{{this}}}
{{/each}}
"#
    .trim_start();

    let hb = Handlebars::new();

    hb.render_template(
        tpl,
        &json!({
            "code": gen_types,
        }),
    )
    .unwrap()
}

pub fn dump_sources_rxx(src_dir: &Path) -> Result<HashSet<PathBuf>> {
    static START: Once = Once::new();

    let src_dir = src_dir.join(NAME);
    fs::create_dir_all(&src_dir)?;

    let wrapper_f = src_dir.join("wrapper.cc");
    let ffi_f = src_dir.join("ffi.cc");

    START.call_once(|| {
	let mut file = fs::File::create(&wrapper_f).unwrap();
	file.write_all(C_SRC.as_bytes()).unwrap();

	let mut file = fs::File::create(&ffi_f).unwrap();
	file.write_all(genc_file_rxx(&[
            &genc_unique_ptr("rxx_unique_string", "std::unique_ptr<std::string>"),
            &genc_shared_ptr("rxx_shared_string", "std::shared_ptr<std::string>"),
            &genc_weak_ptr(
                "rxx_weak_string",
                "std::weak_ptr<std::string>",
                "std::shared_ptr<std::string>",
            ),
	]).as_bytes()).unwrap();
    });

    Ok(HashSet::from([wrapper_f, ffi_f]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fn() {
        let s = genc_fn(
            "MapMut_Matrix3d_new",
            FnSig {
                c_fn: "MapMut_fixed_new<Matrix3d, double>",
                ret_type: ReturnType::Object("Eigen::Map<Matrix3d>"),
                args: &[("double *", "data")],
                ..FnSig::default()
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
                ..FnSig::default()
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
extern "C" void rxx_unique_string_delete(std::unique_ptr<std::string> &self) noexcept {
    rxx::destroy(&self);
}
"#
            .trim_start()
        );

        let s = genc_shared_ptr("rxx_shared_string", "std::shared_ptr<std::string>");
        assert_eq!(s, r#"
extern "C" void rxx_shared_string_delete(std::shared_ptr<std::string> &self) noexcept {
    rxx::destroy(&self);
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
extern "C" void rxx_weak_string_delete(std::weak_ptr<std::string> &self) noexcept {
    rxx::destroy(&self);
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
extern "C" void rxx_vector_string_delete(const std::vector<std::string> &self) {
    rxx::destroy(&self);
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

	let s = genc_get_val("get_global", "int", "test");
	assert_eq!(s, r#"
extern "C" int get_global() noexcept {
    return test;
}
"#.trim_start());
    }
}
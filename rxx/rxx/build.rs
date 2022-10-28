#![feature(default_free_fn)]
use anyhow::Result;
use rxx_build::{self, *};
use std::collections::{HashMap, HashSet};
use std::io::prelude::*;
use std::path::PathBuf;
use std::{env, fs, fs::File};

fn main() -> Result<()> {
    let prefix = PathBuf::from(env::var("CONDA_PREFIX")?);
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let mut inc_dirs = HashSet::from([out_dir.join("include"), prefix.join("include")]);
    let mut src_files = HashSet::new();

    let genc_dir = out_dir.join("gen");
    dump_headers_rxx(&out_dir.join("include"))?;
    src_files.extend(dump_sources_rxx(&genc_dir)?);

    let genc_file_test;

    let profile = env::var("PROFILE").unwrap();
    if profile == "debug" {
        inc_dirs.insert("csrc".into());
        src_files.insert("csrc/test.cc".into());

        for i in ["csrc/test.hh", "csrc/test.cc"] {
            println!("cargo:rerun-if-changed={i}");
        }

        genc_file_test = genc_dir.join("ffi_test.cc");

        fs::create_dir_all(&genc_dir)?;
        let mut file = File::create(genc_dir.join(&genc_file_test))?;
        file.write_all("#include <test.hh>\n".as_bytes())?;

        let mut c_fns = vec![];
        c_fns.extend(rxx_macro::genc_fn!(
            let mut tpl_vars = HashMap::from([
            ("lp".to_owned(), "rxx_dummy"),
            ]);

            #[ffi(link_name="{lp}_cpp_new_vector_i64")]
            fn dummy_cpp_new_vector_i64(a: int) -> std::vector<int64_t> {}

            #[ffi(link_name="{lp}_cpp_add_vector_i64")]
            fn dummy_cpp_add_vector_i64(val: &mut std::vector<int64_t>, n: int) {}

            #[ffi(link_name="{lp}_cpp_addret_vector_i64", atomic)]
            fn dummy_cpp_addret_vector_i64(val: &mut std::vector<int64_t>, n: int) -> int64_t {}

            #[ffi(link_name="{lp}_cpp_get_vector_i64")]
            fn dummy_cpp_get_vector_i64(val: &std::vector<int64_t>) -> int64_t {}

            #[ffi(link_name="{lp}_cpp_getvoid_vector_i64")]
            fn dummy_cpp_getvoid_vector_i64(val: &std::vector<int64_t>, a: int) {}

            #[ffi(link_name="{lp}_cpp_getref_vector_i64", atomic)]
            fn dummy_cpp_getref_vector_i64(val: &std::vector<int64_t>, idx: int) -> &int64_t {}

            #[ffi(link_name="{lp}_i64_delete", ns="rxx")]
            fn delete_pointer(ptr: *mut int64_t) {}

            tpl_vars.insert("lp".to_owned(), "rxx");

            #[ffi(link_prefix="{lp}_Dummy")]
            impl Dummy {
            #[ffi(atomic)]
            fn get(&self, idx: size_t) -> int64_t {}

            #[ffi(atomic)]
            fn get_mut(&mut self, idx: size_t) -> &mut int64_t {}

            fn add(&mut self, val: int64_t) {}
            }
        ));

        c_fns.extend([
            genc_unique_ptr("rxx_unique_i64", "std::unique_ptr<int64_t>"),
            genc_shared_ptr("rxx_shared_i64", "std::shared_ptr<int64_t>"),
            genc_weak_ptr(
                "rxx_weak_i64",
                "std::weak_ptr<int64_t>",
                "std::shared_ptr<int64_t>",
            ),
            genc_vector("rxx_vector_i64", "std::vector<int64_t>", "int64_t"),
        ]);

        file.write_all(genc_file_rxx(&c_fns).unwrap().as_bytes())?;

        src_files.insert(genc_file_test);
    }

    cc::Build::new()
        .files(&src_files)
        .cpp(true)
        .flag_if_supported("-std=c++14")
        .includes(&inc_dirs)
        .compile("rxx");

    Ok(())
}

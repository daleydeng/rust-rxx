use anyhow::Result;
use rxx_build::{self, *};
use std::io::prelude::*;
use std::path::PathBuf;
use std::collections::HashSet;
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

    if cfg!(feature = "test") {
        inc_dirs.insert("csrc".into());
        src_files.insert("csrc/test.cc".into());

        for i in ["csrc/test.hh", "csrc/test.cc"] {
            println!("cargo:rerun-if-changed={}", i);
        }

        genc_file_test = genc_dir.join("ffi_test.cc");

        fs::create_dir_all(&genc_dir)?;
        let mut file = File::create(genc_dir.join(&genc_file_test))?;
        file.write_all("#include <test.hh>\n".as_bytes())?;

        file.write_all(
            genc_file_rxx(&[
                &genc_fn(
                    "rxx_dummy_cpp_new_vector_i64",
                    FnSig {
                        c_fn: "dummy_cpp_new_vector_i64",
                        ret_type: ReturnType::Object("std::vector<int64_t>"),
                        args: &[("int", "a")],
                        ..FnSig::default()
                    },
                ),
                &genc_fn(
                    "rxx_dummy_cpp_add_vector_i64",
                    FnSig {
                        c_fn: "dummy_cpp_add_vector_i64",
                        args: &[
			    ("std::vector<int64_t>&", "val"),
			    ("int", "n")
			],
                        ..FnSig::default()
                    },
                ),
                &genc_fn(
                    "rxx_dummy_cpp_addret_vector_i64",
                    FnSig {
                        c_fn: "dummy_cpp_addret_vector_i64",
                        ret_type: ReturnType::Atomic("int64_t"),
                        args: &[("std::vector<int64_t>&", "val"), ("int", "n")],
                        ..FnSig::default()
                    },
                ),
                &genc_fn(
                    "rxx_dummy_cpp_get_vector_i64",
                    FnSig {
                        c_fn: "dummy_cpp_get_vector_i64",
                        ret_type: ReturnType::Object("int64_t"),
                        args: &[("std::vector<int64_t>const&", "val")],
                        ..FnSig::default()
                    },
                ),
                &genc_fn(
                    "rxx_dummy_cpp_getvoid_vector_i64",
                    FnSig {
                        c_fn: "dummy_cpp_getvoid_vector_i64",
                        args: &[("std::vector<int64_t>const&", "val"), ("int", "a")],
                        ..FnSig::default()
                    },
                ),
                &genc_fn(
                    "rxx_dummy_cpp_getref_vector_i64",
                    FnSig {
                        c_fn: "dummy_cpp_getref_vector_i64",
                        ret_type: ReturnType::Atomic("int64_t const &"),
                        args: &[("std::vector<int64_t>const&", "val"), ("int", "idx")],
                        ..FnSig::default()
                    },
                ),
                &genc_unique_ptr("rxx_unique_i64", "std::unique_ptr<int64_t>"),
                &genc_shared_ptr("rxx_shared_i64", "std::shared_ptr<int64_t>"),
                &genc_weak_ptr(
                    "rxx_weak_i64",
                    "std::weak_ptr<int64_t>",
                    "std::shared_ptr<int64_t>",
                ),
                &genc_vector("rxx_vector_i64", "std::vector<int64_t>", "int64_t"),
                &genc_fn(
                    "rxx_Dummy_get",
                    FnSig {
                        cls: Some("Dummy"),
                        c_fn: "&$C::get",
                        ret_type: ReturnType::Atomic("int64_t"),
                        args: &[("size_t", "idx")],
                        ..FnSig::default()
                    },
                ),
                &genc_fn(
                    "rxx_Dummy_get_mut",
                    FnSig {
                        cls: Some("Dummy"),
                        c_fn: "&$C::get_mut",
                        is_mut: true,
                        ret_type: ReturnType::Atomic("int64_t&"),
                        args: &[("size_t", "idx")],
                    },
                ),
                &genc_fn(
                    "rxx_Dummy_add",
                    FnSig {
                        cls: Some("Dummy"),
                        c_fn: "&$C::add",
                        is_mut: true,
                        args: &[("int64_t", "val")],
                        ..FnSig::default()
                    },
                ),
            ])
            .as_bytes(),
        )?;

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

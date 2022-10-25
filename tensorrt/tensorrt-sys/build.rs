use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::PathBuf;

fn extract_enum_names(fname: &str) -> Result<HashSet<String>> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^enum class (\w+)\s*:").expect("regex creation failed!");
    }
    let mut out = HashSet::new();
    let file = File::open(fname)?;
    for l in io::BufReader::new(file).lines() {
        let l = l?;
        let cap = RE.captures(&l);
        let Some(cap) = cap else {
	    continue;
	};
        out.insert(cap[1].into());
    }
    Ok(out)
}

fn main() -> Result<()> {
    let prefix = env::var("PREFIX").or_else(|_| env::var("CONDA_PREFIX"));
    let s_prefix = prefix?;
    env::set_var("LIBCLANG_PATH", format!("{s_prefix}/lib"));
    let prefix = PathBuf::from(s_prefix);

    let inc_dir = prefix.join("include");

    let mut builder = bindgen::builder()
        .header("wrapper.h")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .enable_cxx_namespaces()
        .disable_untagged_union()
        .clang_args(&[
            &format!("-I{}", inc_dir.to_str().expect("to_str failed")),
            "-x",
            "c++",
        ]);

    for f in ["NvInferVersion.h", "NvInferLegacyDims.h"] {
        builder = builder.allowlist_file(inc_dir.join(f).to_str().expect("to_str failed"));
    }

    for f in [
        "NvInfer.h",
        "NvInferRuntime.h",
        "NvInferRuntimeCommon.h",
        "NvOnnxParser.h",
    ] {
        for n in extract_enum_names(inc_dir.join(f).to_str().expect("to_str failed"))? {
            builder = builder.allowlist_type(format!("nvinfer1::{n}"));
        }
    }

    for f in ["NvOnnxParser.h"] {
        for n in extract_enum_names(inc_dir.join(f).to_str().expect("to_str failed"))? {
            builder = builder.allowlist_type(format!("nvonnxparser::{n}"));
        }
    }

    for t in [
        "nvinfer1::ILogger",
        "nvinfer1::IBuilder",
        "nvinfer1::IRuntime",
        "nvonnxparser::IParser",
    ] {
        builder = builder.allowlist_type(t);
    }

    let bindings = builder.generate()?;
    let out_path = PathBuf::from(env::var("OUT_DIR").expect("ENV::OUT_DIR not exists!"));
    let bindings_f = out_path.join("bindings.rs");
    bindings
        .write_to_file(&bindings_f)
        .expect("Couldn't write bindings!");

    fs::create_dir_all("gen")?;
    fs::copy(bindings_f, "gen/bindings.rs")?;

    println!("cargo:rustc-link-lib=nvinfer");
    println!("cargo:rustc-link-lib=nvonnxparser");

    Ok(())
}

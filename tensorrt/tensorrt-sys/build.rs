// we need 2.17 conda sysroot due to libMvCameraControl need 2.15 glibc

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::PathBuf;
use anyhow::Result;
use std::collections::HashSet;
use lazy_static::lazy_static;
use regex::Regex;

fn extract_enum_names(fname: &str) -> Result<HashSet<String>>{
    lazy_static!{
	static ref RE: Regex = Regex::new(r"^enum class (\w+)\s*:").unwrap();
    }
    let mut out = HashSet::new();
    let file = File::open(fname)?;
    for l in io::BufReader::new(file).lines() {
	let l = l.unwrap();
	let cap = RE.captures(&l);
	if cap.is_none() {
	    continue;
	}
	let cap = cap.unwrap();
	out.insert(cap[1].into());
    }
    Ok(out)
}

fn main() -> Result<()> {
    let prefix =
        PathBuf::from(env::var("PREFIX").unwrap_or_else(|_| env::var("CONDA_PREFIX").unwrap()));

    let inc_dir = prefix.join("include");

    let mut builder = bindgen::builder()
        .header("wrapper.h")
        .default_enum_style(bindgen::EnumVariation::Rust{
	    non_exhaustive: false
	})
        .enable_cxx_namespaces()
        .clang_args(&[&format!("-I{}", inc_dir.to_str().unwrap()), "-x", "c++"])
	;

    for f in [
	"NvInferVersion.h",
	"NvInferLegacyDims.h"
    ] {
	builder = builder.allowlist_file(inc_dir.join(f).to_str().unwrap());
    }

    for f in ["NvInfer.h", "NvInferRuntime.h", "NvInferRuntimeCommon.h", "NvOnnxParser.h"] {
	for n in extract_enum_names(inc_dir.join(f).to_str().unwrap())? {
	    builder = builder.allowlist_type(format!("nvinfer1::{n}"));
	}
    }

    for f in ["NvOnnxParser.h"] {
	for n in extract_enum_names(inc_dir.join(f).to_str().unwrap())? {
	    builder = builder.allowlist_type(format!("nvonnxparser::{n}"));
	}
    }

    for t in [
	"nvinfer1::ILogger",
	"nvinfer1::IBuilder",
	"nvonnxparser::IParser",
    ] {
	builder = builder.allowlist_type(t);
    }

    let bindings = builder.generate()?;
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_f = out_path.join("bindings.rs");
    bindings
        .write_to_file(&bindings_f)
        .expect("Couldn't write bindings!");

    fs::create_dir_all("gen")?;
    fs::copy(bindings_f, "gen/bindings.rs")?;

    Ok(())
}

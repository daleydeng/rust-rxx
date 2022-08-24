use std::env;
use std::path::PathBuf;
use std::collections::HashSet;
use anyhow::Result;
use tensorrt_rxx_build::*;

fn main() -> Result<()> {
    let prefix = PathBuf::from(env::var("CONDA_PREFIX")?);
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    let inc_dirs = HashSet::from([
        out_dir.join("include"),
	prefix.join("include"),
    ]);

    let mut src_files = HashSet::new();

    let genc_dir = out_dir.join("gen");
    dump_headers_tensorrt(&out_dir.join("include"))?;
    src_files.extend(dump_sources_tensorrt(&genc_dir)?);

    cc::Build::new()
        .files(&src_files)
        .cpp(true)
        .flag_if_supported("-std=c++14")
	.flag("-Wno-deprecated-declarations")
	.flag("-Wno-unused-parameter")
        .includes(&inc_dirs)
        .compile("tensorrt_rxx");

    println!("cargo:rustc-link-lib=nvinfer");
    println!("cargo:rustc-link-lib=nvonnxparser");
    Ok(())
}

use anyhow::Result;
use sophus_rxx_build::*;
use std::collections::HashSet;
use std::env;
use std::path::PathBuf;

fn main() -> Result<()> {
    let prefix = PathBuf::from(env::var("CONDA_PREFIX")?);
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    let inc_dirs = HashSet::from([
        out_dir.join("include"),
        prefix.join("include"),
        prefix.join("include").join("eigen3"),
    ]);

    let mut src_files = HashSet::new();

    let genc_dir = out_dir.join("gen");
    dump_headers_sophus(&out_dir.join("include"))?;
    src_files.extend(dump_sources_sophus(&genc_dir)?);

    cc::Build::new()
        .files(&src_files)
        .cpp(true)
        .flag_if_supported("-std=c++14")
        .includes(&inc_dirs)
        .compile("sophus_rxx");

    println!("cargo:rustc-link-lib=fmt");
    Ok(())
}

#![feature(default_free_fn)]
use std::default::default;
use std::path::{Path, PathBuf};
use std::sync::Once;
use std::collections::HashSet;
use anyhow::{bail, Result};
use rxx_build::*;

const C_HDR: &str = include_str!("../include/wrapper.hh");
const NAME: &str = "tensorrt_rxx";

pub fn genc_fns() -> Vec<String> {
    let mut out = vec![];
    let lp = NAME;

    let cls = "ILogger";
    out.push(genc_delete(&format!("{lp}_{cls}_delete"), cls));

    out.push(genc_fn(
	&format!("{lp}_RustLogger_create"),
	FnSig {
	    c_fn: "RustLogger::create",
	    ret_type: ReturnType::Atomic("ILogger*"),
	    args: &[
		("void*", "obj"),
		("log_fn_t", "log_fn"),
	    ],
	    ..default()
	}
    ));
    out
}

pub fn dump_headers_tensorrt(inc_dir: &Path) -> Result<HashSet<PathBuf>> {
    static ONCE: Once = Once::new();

    let mut out = dump_headers_rxx(inc_dir)?;
    let inc_dir = inc_dir.join(NAME);
    let wrapper_f = inc_dir.join("wrapper.hh");
    dump_file_once(&wrapper_f, C_HDR, &ONCE);
    out.insert(inc_dir);
    Ok(out)
}

pub fn genc_file_tensorrt(fns: &[&str]) -> Result<String> {
    if fns.is_empty() {
        bail!("empty gen types!");
    }

    let tpl = r#"
#include <tensorrt_rxx/wrapper.hh>

using namespace tensorrt_rxx;

{{#each items}}
{{{this}}}
{{/each}}
"#
    .trim_start();
    render_c_template(tpl, fns)
}

pub fn dump_sources_tensorrt(src_dir: &Path) -> Result<HashSet<PathBuf>> {
    static ONCE: Once = Once::new();

    let mut out = dump_sources_rxx(src_dir)?;
    let src_dir = src_dir.join(NAME);
    let ffi_f = src_dir.join("ffi.cc");
    let fn_codes = genc_fns();
    let fn_codes: Vec<&str> = fn_codes.iter().map(AsRef::as_ref).collect();
    let ffi_code = genc_file_tensorrt(&fn_codes).unwrap();
    dump_file_once(&ffi_f, &ffi_code, &ONCE);
    out.insert(ffi_f);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
	let out = genc_fns();
	println!("{}", out[0]);
    }
}

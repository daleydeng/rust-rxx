use std::path::{Path, PathBuf};
use std::fs;
use std::io::Write;
use std::collections::HashSet;
use std::sync::Once;
use anyhow::{bail, Result};
use handlebars::Handlebars;
use serde_json::json;
use rxx_build::*;
use eigen_rxx_build::*;

const C_HDR: &str = include_str!("../include/wrapper.hh");
const NAME: &str = "sophus_rxx";

pub fn genc_fns() -> Vec<String> {
    let mut out = vec![];

    let lp = NAME;

    for (tp, t) in [("double", "d"), ("float", "f")] {
	let cls = &format!("SO2{t}");
	for (val_tp, val) in [("int","DoF"), ("int", "num_parameters"), ("int", "N"), ("int", "Dim")] {
	    out.push(genc_get_val(
		&format!("{lp}_get_{cls}_{val}"),
		ReturnType::Atomic(val_tp),
		&format!("{cls}::{val}"),
	    ));
	}

	out.push(genc_fn(&format!("{lp}_{cls}_Adj"), FnSig {
	    cls: Some(cls),
	    c_fn: "&$C::Adj",
	    ret_type: ReturnType::Atomic(tp),
	    ..FnSig::default()
	}));

	// cast, data
	out.push(genc_fn(&format!("{lp}_{cls}_inverse"), FnSig {
	    cls: Some(cls),
	    c_fn: "&$C::inverse",
	    ret_type: ReturnType::Object(cls),
	    ..FnSig::default()
	}));

	out.push(genc_fn(&format!("{lp}_{cls}_log"), FnSig {
	    cls: Some(cls),
	    c_fn: "&$C::log",
	    ret_type: ReturnType::Atomic(tp),
	    ..FnSig::default()
	}));

	out.push(genc_fn(&format!("{lp}_{cls}_normalize"), FnSig {
	    cls: Some(cls),
	    is_mut: true,
	    c_fn: "&$C::normalize",
	    ..FnSig::default()
	}));

	out.push(genc_fn(&format!("{lp}_{cls}_matrix"), FnSig {
	    cls: Some(cls),
	    c_fn: "&$C::matrix",
	    ret_type: ReturnType::Object("$C::Transformation"),
	    ..FnSig::default()
	}));

	// operator=
	// operator*
	out.push(genc_fn(&format!("{lp}_{cls}_mul"), FnSig {
	    c_fn: "op_mul",
	    ret_type: ReturnType::Object(cls),
	    args: &[
		(&format!("{cls} const&"), "self"),
		(&format!("{cls} const&"), "other"),
	    ],
	    ..FnSig::default()
	}));

	out.push(genc_fn(&format!("{lp}_{cls}_mul_point"), FnSig {
	    c_fn: "op_mul",
	    ret_type: ReturnType::Object(&format!("{cls}::Point")),
	    args: &[
		(&format!("{cls} const&"), "self"),
		(&format!("{cls}::Point const&"), "other"),
	    ],
	    ..FnSig::default()
	}));

	out.push(genc_fn(&format!("{lp}_{cls}_mul_hpoint"), FnSig {
	    c_fn: "op_mul",
	    ret_type: ReturnType::Object(&format!("{cls}::HomogeneousPoint")),
	    args: &[
		(&format!("{cls} const&"), "self"),
		(&format!("{cls}::HomogeneousPoint const&"), "other"),
	    ],
	    ..FnSig::default()
	}));
	// end operator*

	// Dx_this_mul_exp_x_at_0
	out.push(genc_fn(&format!("{lp}_{cls}_params"), FnSig {
	    cls: Some(cls),
	    c_fn: "&$C::params",
	    ret_type: ReturnType::Object("Matrix<$C::Scalar, $C::num_parameters, 1>"),
	    ..FnSig::default()
	}));

	// Dx_log_this_inv_by_x_at_this
	out.push(genc_fn(&format!("{lp}_{cls}_setComplex"), FnSig {
	    cls: Some(cls),
	    c_fn: "&$C::setComplex",
	    is_mut: true,
	    args: &[
		("$C::Point const&", "v"),
	    ],
	    ..FnSig::default()
	}));

	out.push(genc_fn(&format!("{lp}_{cls}_unit_complex"), FnSig {
	    cls: Some(cls),
	    c_fn: "&$C::unit_complex",
	    ret_type: ReturnType::Atomic("$C::ComplexT const &"),
	    ..FnSig::default()
	}));

    }

    out
}

pub fn genc_file_sophus(fns: &[&str]) -> Result<String> {
    if fns.is_empty() {
        bail!("empty gen types!");
    }

    let tpl = r#"
#include <sophus_rxx/wrapper.hh>

using namespace sophus_rxx;

{{#each fns}}
{{{this}}}
{{/each}}
"#
    .trim_start();

    let hb = Handlebars::new();

    Ok(hb.render_template(
        tpl,
        &json!({
        "fns": fns,
        }),
    )?)
}

pub fn dump_headers_sophus(inc_dir: &Path) -> Result<HashSet<PathBuf>> {
    static START: Once = Once::new();

    let mut out = dump_headers_eigen(inc_dir)?;

    let inc_dir = inc_dir.join(NAME);
    fs::create_dir_all(&inc_dir)?;

    let wrapper_f = inc_dir.join("wrapper.hh");
    START.call_once(|| {
	let mut file = fs::File::create(&wrapper_f).unwrap();
	file.write_all(C_HDR.as_bytes()).unwrap();
    });

    out.insert(inc_dir);
    Ok(out)
}

pub fn dump_sources_sophus(src_dir: &Path) -> Result<HashSet<PathBuf>> {
    static START: Once = Once::new();

    let mut out = dump_sources_eigen(src_dir)?;

    let src_dir = src_dir.join(NAME);
    fs::create_dir_all(&src_dir)?;

    let ffi_f = src_dir.join("ffi.cc");
    START.call_once(|| {
        let mut file = fs::File::create(&ffi_f).unwrap();

	let fn_codes = genc_fns();
        let fn_codes: Vec<&str> = fn_codes.iter().map(AsRef::as_ref).collect();
        file.write_all(genc_file_sophus(&fn_codes).unwrap().as_bytes()).unwrap();
    });
    out.insert(ffi_f);
    Ok(out)
}

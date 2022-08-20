use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Once;
use anyhow::{bail, Result};
use rxx_build::*;

pub fn genc_file_eigen(fns: &[&str]) -> Result<String> {
    if fns.is_empty() {
        bail!("empty gen types!");
    }

    let tpl = r#"
#include <eigen_rxx/wrapper.hh>

using namespace eigen_rxx;

{{#each items}}
{{{this}}}
{{/each}}
"#
    .trim_start();

    render_c_template(tpl, fns)
}

#[allow(non_snake_case)]
fn add_fns_Map(lp: &str, cls: &str, out: &mut Vec<String>) {
    out.push(genc_fn(
        &format!("{lp}_MapMut_{cls}_new"),
        FnSig {
            c_fn: &format!("MapMut_fixed_new<{cls}, {cls}::Scalar>"),
            ret_type: ReturnType::Object(&format!("Eigen::Map<{cls}>")),
            args: &[(&format!("{cls}::Scalar*"), "data")],
            ..Default::default()
        },
    ));

    out.push(genc_fn(
        &format!("{lp}_Map_{cls}_new"),
        FnSig {
            c_fn: &format!("Map_fixed_new<{cls}, {cls}::Scalar>"),
            ret_type: ReturnType::Object(&format!("Eigen::Map<{cls} const>")),
            args: &[(&format!("{cls}::Scalar const*"), "data")],
            ..Default::default()
        },
    ));
}

fn genc_fns() -> Vec<String> {
    let mut out = vec![];
    let lp = "eigen_rxx"; // link_prefix

    let mut all_types = vec![];

    for (_tp, t) in [("double", "d"), ("float", "f"), ("int", "i")] {
        for (row, col) in [(2, 2), (3, 3), (4, 4), (2, 3), (2, 1), (3, 1), (4, 1)] {
            let cls;
            if col == 1 {
                cls = format!("Vector{row}{t}");
            } else if row == col {
                cls = format!("Matrix{row}{t}");
            } else {
                cls = format!("Matrix{row}x{col}{t}");
            }

            add_fns_Map(lp, &cls, &mut out);

            all_types.extend([cls.clone(), format!("MapMut_{cls}"), format!("Map_{cls}")]);
        }
    }

    for (_tp, t) in [("double", "d"), ("float", "f")] {
        let cls = &format!("Quaternion{t}");

        all_types.push(cls.clone());

        out.push(genc_fn(
            &format!("{lp}_{cls}_normalized"),
            FnSig {
                cls: Some(cls),
                c_fn: "&$C::normalized",
                ret_type: ReturnType::Object("$C"),
                ..Default::default()
            },
        ));

        out.push(genc_fn(
            &format!("{lp}_{cls}_normalize"),
            FnSig {
                cls: Some(cls),
                is_mut: true,
                c_fn: "&$C::normalize",
                ..Default::default()
            },
        ));

        out.push(genc_fn(
            &format!("{lp}_{cls}_inverse"),
            FnSig {
                cls: Some(cls),
                c_fn: "&$C::inverse",
                ret_type: ReturnType::Object("$C"),
                ..Default::default()
            },
        ));

        out.push(genc_fn(
            &format!("{lp}_{cls}_mul"),
            FnSig {
                c_fn: "op_mul",
                ret_type: ReturnType::Object(cls),
                args: &[
                    (&format!("{cls} const &"), "self"),
                    (&format!("{cls} const &"), "other"),
                ],
                ..Default::default()
            },
        ));

        out.push(genc_fn(
            &format!("{lp}_{cls}_toRotationMatrix"),
            FnSig {
                cls: Some(cls),
                c_fn: "&$C::toRotationMatrix",
                ret_type: ReturnType::Object(&format!("Matrix3{t}")),
                ..Default::default()
            },
        ));

        add_fns_Map(lp, cls, &mut out);

        let quat_cls = cls;
        let cls = &format!("AngleAxis{t}");

        out.push(genc_fn(
            &format!("{lp}_{cls}_inverse"),
            FnSig {
                cls: Some(cls),
                c_fn: "&$C::inverse",
                ret_type: ReturnType::Object("$C"),
                ..Default::default()
            },
        ));

        out.push(genc_fn(
            &format!("{lp}_{cls}_mul"),
            FnSig {
                c_fn: "op_mul",
                ret_type: ReturnType::Object(quat_cls),
                args: &[
                    (&format!("{cls} const &"), "self"),
                    (&format!("{cls} const &"), "other"),
                ],
                ..Default::default()
            },
        ));

        out.push(genc_fn(
            &format!("{lp}_{cls}_toRotationMatrix"),
            FnSig {
                cls: Some(cls),
                c_fn: "&$C::toRotationMatrix",
                ret_type: ReturnType::Object(&format!("Matrix3{t}")),
                ..Default::default()
            },
        ));
    }

    for cls in all_types {
        out.push(genc_fn(
            &format!("{lp}_{cls}_to_string"),
            FnSig {
                c_fn: &format!("to_string<{cls}>"),
                ret_type: ReturnType::Object("std::unique_ptr<std::string>"),
                args: &[(&format!("{cls} const &"), "self")],
                ..Default::default()
            },
        ));
    }

    out
}

const C_HDR: &str = include_str!("../include/wrapper.hh");
const NAME: &str = "eigen_rxx";

pub fn dump_headers_eigen(inc_dir: &Path) -> Result<HashSet<PathBuf>> {
    static ONCE: Once = Once::new();

    let mut out = dump_headers_rxx(inc_dir)?;
    let inc_dir = inc_dir.join(NAME);
    let wrapper_f = inc_dir.join("wrapper.hh");
    dump_file_once(&wrapper_f, C_HDR, &ONCE);
    out.insert(inc_dir);
    Ok(out)
}

pub fn dump_sources_eigen(src_dir: &Path) -> Result<HashSet<PathBuf>> {
    static ONCE: Once = Once::new();
    let mut out = dump_sources_rxx(src_dir)?;

    let src_dir = src_dir.join(NAME);
    let ffi_f = src_dir.join("ffi.cc");

    let fn_codes = genc_fns();
    let fn_codes: Vec<&str> = fn_codes.iter().map(AsRef::as_ref).collect();
    let ffi_code = genc_file_eigen(&fn_codes).unwrap();
    dump_file_once(&ffi_f, &ffi_code, &ONCE);

    out.insert(ffi_f);
    Ok(out)
}

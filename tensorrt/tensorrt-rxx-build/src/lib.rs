#![feature(default_free_fn)]
use std::default::default;
use std::path::{Path, PathBuf};
use std::sync::Once;
use std::collections::HashSet;
use anyhow::{bail, Result};
use rxx_build::*;

const C_HDR: &str = include_str!("../include/wrapper.hh");
const C_SRC: &str = include_str!("../csrc/wrapper.cc");

const NAME: &str = "tensorrt_rxx";
const LP: &str = NAME;

fn gen_builder_config(out: &mut Vec<String>) {
    let cls = "IBuilderConfig";
    out.push(genc_fn(
	&format!("{LP}_{cls}_clearFlag"),
	FnSig {
	    cls: Some(cls),
	    c_fn: "&$C::clearFlag",
	    is_mut: true,
	    args: &[
		("BuilderFlag", "flag"),
	    ],
	    ..default()
	}
    ));

    out.push(genc_fn(
	&format!("{LP}_{cls}_setFlag"),
	FnSig {
	    cls: Some(cls),
	    c_fn: "&$C::setFlag",
	    is_mut: true,
	    args: &[
		("BuilderFlag", "flag"),
	    ],
	    ..default()
	}
    ));

    out.push(genc_fn(
	&format!("{LP}_{cls}_getFlag"),
	FnSig {
	    cls: Some(cls),
	    c_fn: "&$C::getFlag",
	    ret_type: ReturnType::Atomic("bool"),
	    args: &[
		("BuilderFlag", "flag"),
	    ],
	    ..default()
	}
    ));
}

fn gen_tensor(out: &mut Vec<String>) {
    let cls = "ITensor";

    out.push(genc_fn(
	&format!("{LP}_{cls}_dynamicRangeIsSet"),
	FnSig {
	    cls: Some(cls),
	    c_fn: "&$C::dynamicRangeIsSet",
	    ret_type: ReturnType::Atomic("bool"),
	    ..default()
	}
    ));

    out.push(genc_fn(
	&format!("{LP}_{cls}_setDynamicRange"),
	FnSig {
	    cls: Some(cls),
	    c_fn: "&$C::setDynamicRange",
	    is_mut: true,
	    ret_type: ReturnType::Atomic("bool"),
	    args: &[
		("float", "min"),
		("float", "max"),
	    ],
	}
    ));
}

fn gen_layer(out: &mut Vec<String>) {
    let cls = "ILayer";

    out.push(genc_fn(
	&format!("{LP}_{cls}_getNbInputs"),
	FnSig {
	    cls: Some(cls),
	    c_fn: "&$C::getNbInputs",
	    ret_type: ReturnType::Atomic("int32_t"),
	    ..default()
	}
    ));

    out.push(genc_fn(
	&format!("{LP}_{cls}_getInput"),
	FnSig {
	    cls: Some(cls),
	    c_fn: "&$C::getInput",
	    ret_type: ReturnType::Atomic("ITensor*"),
	    args: &[
		("int32_t", "index"),
	    ],
	    ..default()
	}
    ));

    out.push(genc_fn(
	&format!("{LP}_{cls}_getNbOutputs"),
	FnSig {
	    cls: Some(cls),
	    c_fn: "&$C::getNbOutputs",
	    ret_type: ReturnType::Atomic("int32_t"),
	    ..default()
	}
    ));

    out.push(genc_fn(
	&format!("{LP}_{cls}_getOutput"),
	FnSig {
	    cls: Some(cls),
	    c_fn: "&$C::getOutput",
	    ret_type: ReturnType::Atomic("ITensor*"),
	    args: &[
		("int32_t", "index"),
	    ],
	    ..default()
	}
    ));

}

fn gen_network_definition(out: &mut Vec<String>) {
    let cls = "INetworkDefinition";
    out.push(genc_fn(
	&format!("{LP}_{cls}_getNbLayers"),
	FnSig {
	    cls: Some(cls),
	    c_fn: "&$C::getNbLayers",
	    ret_type: ReturnType::Atomic("int32_t"),
	    ..default()
	}
    ));

    out.push(genc_fn(
	&format!("{LP}_{cls}_getLayer"),
	FnSig {
	    cls: Some(cls),
	    c_fn: "&$C::getLayer",
	    ret_type: ReturnType::Atomic("ILayer*"),
	    args: &[
		("int32_t", "index"),
	    ],
	    ..default()
	}
    ));

    out.push(genc_fn(
	&format!("{LP}_{cls}_getNbInputs"),
	FnSig {
	    cls: Some(cls),
	    c_fn: "&$C::getNbInputs",
	    ret_type: ReturnType::Atomic("int32_t"),
	    ..default()
	}
    ));

}

pub fn genc_fns() -> Vec<String> {
    let mut out = vec![];

    for cls in ["ILogger", "IBuilder", "INetworkDefinition", "IBuilderConfig", "OnnxIParser"] {
	out.push(genc_delete(&format!("{LP}_{cls}_delete"), cls));
    }

    gen_builder_config(&mut out);
    gen_network_definition(&mut out);
    gen_layer(&mut out);
    gen_tensor(&mut out);

    out.push(genc_fn(
	&format!("{LP}_RustLogger_create"),
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

    out.push(genc_fn(
	&format!("{LP}_log"),
	FnSig {
	    c_fn: "log",
	    args: &[
		("ILogger*", "self"),
		("ILogger::Severity", "severity"),
		("const char*", "msg"),
	    ],
	    ..default()
	}
    ));

    out.push(genc_fn(
	&format!("{LP}_createOnnxParser"),
	FnSig {
	    c_fn: "nvonnxparser::createParser",
	    ret_type: ReturnType::Atomic("nvonnxparser::IParser*"),
	    args: &[
		("INetworkDefinition&", "network"),
		("ILogger&", "logger"),
	    ],
	    ..default()
	}
    ));

    out.push(genc_fn(
	&format!("{LP}_createInferBuilder"),
	FnSig {
	    c_fn: "createInferBuilder",
	    ret_type: ReturnType::Atomic("IBuilder*"),
	    args: &[
		("ILogger&", "logger"),
	    ],
	    ..default()
	}
    ));

    let cls = "IBuilder";
    out.push(genc_fn(
	&format!("{LP}_{cls}_createNetworkV2"),
	FnSig {
	    cls: Some(cls),
	    c_fn: "&$C::createNetworkV2",
	    is_mut: true,
	    ret_type: ReturnType::Atomic("INetworkDefinition*"),
	    args: &[
		("NetworkDefinitionCreationFlags", "flags"),
	    ],
	}
    ));


    let cls = "IBuilder";
    out.push(genc_fn(
	&format!("{LP}_{cls}_createBuilderConfig"),
	FnSig {
	    cls: Some(cls),
	    c_fn: "&$C::createBuilderConfig",
	    is_mut: true,
	    ret_type: ReturnType::Atomic("IBuilderConfig*"),
	    ..default()
	}
    ));

    let cls = "OnnxIParser";
    out.push(genc_fn(
	&format!("{LP}_{cls}_parseFromFile"),
	FnSig {
	    cls: Some(cls),
	    c_fn: "&$C::parseFromFile",
	    is_mut: true,
	    ret_type: ReturnType::Atomic("bool"),
	    args: &[
		("const char*", "fname"),
		("int", "verbosity"),
	    ],
	}
    ));

    out.push(genc_fn(
	&format!("{LP}_{cls}_clearErrors"),
	FnSig {
	    cls: Some(cls),
	    c_fn: "&$C::clearErrors",
	    is_mut: true,
	    ..default()
	}
    ));

    out.push(genc_fn(
	&format!("{LP}_{cls}_getNbErrors"),
	FnSig {
	    cls: Some(cls),
	    c_fn: "&$C::getNbErrors",
	    ret_type: ReturnType::Atomic("int"),
	    ..default()
	}
    ));

    out.push(genc_fn(
	&format!("{LP}_{cls}_getError"),
	FnSig {
	    cls: Some(cls),
	    c_fn: "&$C::getError",
	    ret_type: ReturnType::Atomic("OnnxIParserError const*"),
	    args: &[
		("int", "index"),
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
    static ONCE_WRAPPER: Once = Once::new();
    static ONCE_FFI: Once = Once::new();

    let mut out = dump_sources_rxx(src_dir)?;
    let src_dir = src_dir.join(NAME);

    let wrapper_f = src_dir.join("wrapper.cc");
    dump_file_once(&wrapper_f, C_SRC, &ONCE_WRAPPER);
    out.insert(wrapper_f);

    let ffi_f = src_dir.join("ffi.cc");
    let fn_codes = genc_fns();
    let fn_codes: Vec<&str> = fn_codes.iter().map(AsRef::as_ref).collect();
    let ffi_code = genc_file_tensorrt(&fn_codes).unwrap();
    dump_file_once(&ffi_f, &ffi_code, &ONCE_FFI);
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

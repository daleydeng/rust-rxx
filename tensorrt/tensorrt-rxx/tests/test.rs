#![feature(slice_range)]
use rstest::rstest;
use std::{
    path::{self, Path},
    slice::range,
};
use tensorrt_rxx::ffi::BuilderFlag::*;
use tensorrt_rxx::*;

fn set_all_dynamic_ranges(network: &mut INetworkDefinition, in_range: f32, out_range: f32)
{
    let layer_nr = network.get_nb_layers();
    for i in 0..layer_nr {
	let layer = network.get_layer_mut(i).unwrap();

	let inp_nr = layer.get_nb_inputs();
	for j in 0..inp_nr {
	    let input = layer.get_input_mut(j).unwrap();
	    if !input.dynamic_range_is_set() {
		assert!(input.set_dynamic_range(-in_range, in_range));
	    }
	}

	let cur_out_range = if layer.get_type() == ffi::LayerType::kPOOLING {in_range} else {out_range};

	let oup_nr = layer.get_nb_outputs();
	for j in 0..oup_nr {
	    let output = layer.get_output_mut(j).unwrap();

	    if !output.dynamic_range_is_set() {
		assert!(output.set_dynamic_range(-cur_out_range, cur_out_range));
	    }
	}
    }


}

fn enable_dla(builder: &IBuilder, cfg: &mut IBuilderConfig, use_dla_core: i32, allow_gpu_fallback: bool)
{
    if use_dla_core < 0 {
	return
    }

    let dla_core_nr = builder.get_nb_dla_cores();
    if dla_core_nr <= 0 {
	println!("no dla core");
	return;
    }

    if allow_gpu_fallback {
	cfg.set_flag(kGPU_FALLBACK);
    }

    if !cfg.get_flag(kINT8) {
	cfg.set_flag(kFP16);
    }

    cfg.set_default_device_type(ffi::DeviceType::kDLA);
    cfg.set_dla_core(use_dla_core);
}

#[rstest]
#[case(false, true)]
// #[case(false, false)]
// #[case(false, true)]
// #[case(true, false)]
// #[case(true, true)]
fn test_builder(#[case] use_fp16: bool, #[case] use_int8: bool) {
    let mut rust_logger = Default::default();
    let mut logger = create_rust_logger(&mut rust_logger, RustLogger::log_callback);
    let mut builder = create_infer_builder(&mut logger);
    let mut network = builder
        .create_network_v2(1u32 << ffi::NetworkDefinitionCreationFlag::kEXPLICIT_BATCH as i32);
    let mut config = builder.create_builder_config();
    let mut parser = create_onnx_parser(&mut network, &mut logger);
    let p = Path::new("data/mnist.onnx");
    let s_p = p.to_str().unwrap();

    parser
        .parse_from_file(p, Severity::kVERBOSE)
        .expect(&format!("parse error {s_p}"));

    if use_fp16 {
        config.set_flag(kFP16);
    }

    if use_int8 {
        config.set_flag(kINT8);

	set_all_dynamic_ranges(&mut network, 127.0f32, 127.0f32)
    }

    let use_dla_core = 1;
    enable_dla(&builder, &mut config, use_dla_core, true);
}

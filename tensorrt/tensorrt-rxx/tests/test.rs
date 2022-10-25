#![feature(slice_range)]
use crate::type_size;
use cuda_rs::*;
use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageBuffer};
use nalgebra::SVector;
use num_traits::zero;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rstest::rstest;
use std::io::Cursor;
use std::path::Path;
use tensorrt_rxx::ffi::BuilderFlag::*;
use tensorrt_rxx::*;

fn div_up(x: i32, n: i32) -> i32 {
    (x + n - 1) / n
}

fn set_all_dynamic_ranges(network: &mut INetworkDefinition, in_range: f32, out_range: f32) {
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

        let cur_out_range = if layer.get_type() == ffi::LayerType::kPOOLING {
            in_range
        } else {
            out_range
        };

        let oup_nr = layer.get_nb_outputs();
        for j in 0..oup_nr {
            let output = layer.get_output_mut(j).unwrap();

            if !output.dynamic_range_is_set() {
                assert!(output.set_dynamic_range(-cur_out_range, cur_out_range));
            }
        }
    }
}

fn enable_dla(
    builder: &IBuilder,
    cfg: &mut IBuilderConfig,
    use_dla_core: i32,
    allow_gpu_fallback: bool,
) {
    if use_dla_core < 0 {
        return;
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

    let profile_stream = StreamPtr::create_with_flags(true);
    config.set_profile_stream(&profile_stream);

    let plan = builder.build_serialized_network(&mut network, &mut config);

    let mut runtime = create_infer_runtime(&mut logger);

    let mut engine = runtime.deserialize_cuda_engine_host_memory(&plan);

    let mut dev_mems = Vec::new();
    for i in 0..engine.get_nb_bindings() {
        let mut dims = engine.get_binding_dimensions(i);
        let mut vol = 1usize;
        let type_ = engine.get_binding_data_type(i);
        let vec_dim = engine.get_binding_vectorized_dim(i);
        if vec_dim != -1 {
            let vec_dim = vec_dim as usize;
            let scalars_per_vec = engine.get_binding_components_per_element(i);
            dims.d[vec_dim] = div_up(dims.d[vec_dim], scalars_per_vec);
            vol *= scalars_per_vec as usize;
        }
        vol *= dims.d[..dims.nbDims as usize].iter().product::<i32>() as usize;
        println!("vol {vol}");
        println!("dims {:?}", dims);
        let binding_name = engine.get_binding_name(i).unwrap();

        println!("type {:?}", engine.get_binding_data_type(i));
        let dev_mem = DeviceMemory::malloc(vol * type_size(engine.get_binding_data_type(i)));
        dev_mems.push(dev_mem);
    }

    assert_eq!(network.get_nb_inputs(), 1);
    let input_dims = network.get_input(0).unwrap().get_dimensions();
    assert_eq!(input_dims.nbDims, 4);

    assert_eq!(network.get_nb_outputs(), 1);
    let output_dims = network.get_output(0).unwrap().get_dimensions();
    assert_eq!(output_dims.nbDims, 2);

    let mut context = engine.create_execution_context();

    let mut ids: Vec<i32> = (0..10).collect();
    ids.shuffle(&mut thread_rng());

    let mut correct = 0;
    for i in &ids {
        let img = ImageReader::open(format!("data/images/{i}.pgm"))
            .unwrap()
            .decode()
            .unwrap();
        let DynamicImage::ImageLuma8(img) = img else {
	    panic!("image format wrong");
	};

        // let w = img.width();
        // let h = img.height();
        let buf = img
            .pixels()
            .map(|p| 1.0 - p.0[0] as f32 / 255.0)
            .collect::<Vec<_>>();

        dev_mems[0].copy_from_slice(&buf);
        context.execute_v2(&mut dev_mems).unwrap();

        let mut ret = SVector::<f32, 10>::zeros();
        dev_mems[1].copy_to_slice(ret.as_mut_slice());

        let pred = ret.argmax().0 as i32;
        correct += (pred == *i) as usize;
    }

    let total = ids.len();
    println!("correct/total {correct}/{total}");
    assert_eq!(correct, total);
}

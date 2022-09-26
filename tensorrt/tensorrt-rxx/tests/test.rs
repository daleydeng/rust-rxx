use rstest::rstest;
use std::path::{self, Path};
use tensorrt_rxx::*;

#[rstest]
#[case(false, false)]
#[case(false, true)]
#[case(true, false)]
#[case(true, true)]
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
        config.set_flag(ffi::BuilderFlag::kFP16);
    }

    if use_int8 {
        config.set_flag(ffi::BuilderFlag::kINT8);
    }
    //
    // println!("fp16 {}", config.get_flag(ffi::BuilderFlag::kFP16));
}

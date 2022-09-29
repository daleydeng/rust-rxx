use tensorrt_sys::nvinfer1 as ffi;

pub const fn type_size(t: ffi::DataType) -> usize {
    use ffi::DataType::*;
    match t {
	kFLOAT => 4,
	kHALF => 2,
	kINT8 => 1,
	kINT32 => 4,
	kBOOL => 1,
    }
}

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub use root::*;

#[cfg(test)]
mod tests {
    use super::nvinfer1::*;

    #[test]
    fn assert_enum() {
        assert_eq!(LayerType::kCONVOLUTION as i32, 0);
    }
}

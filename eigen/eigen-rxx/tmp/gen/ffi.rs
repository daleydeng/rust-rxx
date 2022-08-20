extern crate nalgebra as na;

use na::Dim;
use std::ops::Mul;

pub trait FixedSize {
    type Scalar;
}

#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct Matrix<SCALAR, const ROWS: usize, const COLS: usize> where [(); ROWS * COLS]: {
    pub data: [SCALAR; ROWS * COLS]
}

impl<SCALAR, const ROWS: usize, const COLS: usize> FixedSize for Matrix<SCALAR, ROWS, COLS> where [(); ROWS * COLS]: {
    type Scalar = SCALAR;
}

#[repr(C)]
#[derive(Debug)]
pub struct Map<'a, T: FixedSize> {
    pub data: *const T::Scalar,
    _pd: std::marker::PhantomData<&'a ()>,
}

#[repr(C)]
#[derive(Debug)]
pub struct MapMut<'a, T: FixedSize> {
    pub data: *mut T::Scalar,
    _pd: std::marker::PhantomData<&'a ()>,
}

pub type Matrix2d = Matrix<f64, 2, 2>;
pub type Matrix3d = Matrix<f64, 3, 3>;
pub type Matrix4d = Matrix<f64, 4, 4>;
pub type Matrix2x3d = Matrix<f64, 2, 3>;
pub type Vector2d = Matrix<f64, 2, 1>;
pub type Vector3d = Matrix<f64, 3, 1>;
pub type Vector4d = Matrix<f64, 4, 1>;
pub type Matrix2f = Matrix<f32, 2, 2>;
pub type Matrix3f = Matrix<f32, 3, 3>;
pub type Matrix4f = Matrix<f32, 4, 4>;
pub type Matrix2x3f = Matrix<f32, 2, 3>;
pub type Vector2f = Matrix<f32, 2, 1>;
pub type Vector3f = Matrix<f32, 3, 1>;
pub type Vector4f = Matrix<f32, 4, 1>;
pub type Matrix2i = Matrix<i32, 2, 2>;
pub type Matrix3i = Matrix<i32, 3, 3>;
pub type Matrix4i = Matrix<i32, 4, 4>;
pub type Matrix2x3i = Matrix<i32, 2, 3>;
pub type Vector2i = Matrix<i32, 2, 1>;
pub type Vector3i = Matrix<i32, 3, 1>;
pub type Vector4i = Matrix<i32, 4, 1>;


#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct Quaterniond {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl FixedSize for Quaterniond {
    type Scalar = f64;
}
#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct Quaternionf {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl FixedSize for Quaternionf {
    type Scalar = f32;
}


#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct AngleAxisd {
    pub axis: Vector3d,
    pub angle: f64,
}#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct AngleAxisf {
    pub axis: Vector3f,
    pub angle: f32,
}

pub fn MapMut_Matrix2d_new<'a>(data: *mut f64) -> MapMut<'a, Matrix2d> {
    extern "C" {
        #[link_name = "eigen_rxx$MapMut_Matrix2d_new"]
        fn __func<'a>(_: *mut f64, __return: *mut MapMut<'a, Matrix2d>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<MapMut<'a, Matrix2d>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn Map_Matrix2d_new<'a>(data: *const f64) -> Map<'a, Matrix2d> {
    extern "C" {
        #[link_name = "eigen_rxx$Map_Matrix2d_new"]
        fn __func<'a>(_: *const f64, __return: *mut Map<'a, Matrix2d>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<Map<'a, Matrix2d>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn MapMut_Matrix3d_new<'a>(data: *mut f64) -> MapMut<'a, Matrix3d> {
    extern "C" {
        #[link_name = "eigen_rxx$MapMut_Matrix3d_new"]
        fn __func<'a>(_: *mut f64, __return: *mut MapMut<'a, Matrix3d>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<MapMut<'a, Matrix3d>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn Map_Matrix3d_new<'a>(data: *const f64) -> Map<'a, Matrix3d> {
    extern "C" {
        #[link_name = "eigen_rxx$Map_Matrix3d_new"]
        fn __func<'a>(_: *const f64, __return: *mut Map<'a, Matrix3d>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<Map<'a, Matrix3d>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn MapMut_Matrix4d_new<'a>(data: *mut f64) -> MapMut<'a, Matrix4d> {
    extern "C" {
        #[link_name = "eigen_rxx$MapMut_Matrix4d_new"]
        fn __func<'a>(_: *mut f64, __return: *mut MapMut<'a, Matrix4d>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<MapMut<'a, Matrix4d>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn Map_Matrix4d_new<'a>(data: *const f64) -> Map<'a, Matrix4d> {
    extern "C" {
        #[link_name = "eigen_rxx$Map_Matrix4d_new"]
        fn __func<'a>(_: *const f64, __return: *mut Map<'a, Matrix4d>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<Map<'a, Matrix4d>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn MapMut_Matrix2x3d_new<'a>(data: *mut f64) -> MapMut<'a, Matrix2x3d> {
    extern "C" {
        #[link_name = "eigen_rxx$MapMut_Matrix2x3d_new"]
        fn __func<'a>(_: *mut f64, __return: *mut MapMut<'a, Matrix2x3d>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<MapMut<'a, Matrix2x3d>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn Map_Matrix2x3d_new<'a>(data: *const f64) -> Map<'a, Matrix2x3d> {
    extern "C" {
        #[link_name = "eigen_rxx$Map_Matrix2x3d_new"]
        fn __func<'a>(_: *const f64, __return: *mut Map<'a, Matrix2x3d>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<Map<'a, Matrix2x3d>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn MapMut_Vector2d_new<'a>(data: *mut f64) -> MapMut<'a, Vector2d> {
    extern "C" {
        #[link_name = "eigen_rxx$MapMut_Vector2d_new"]
        fn __func<'a>(_: *mut f64, __return: *mut MapMut<'a, Vector2d>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<MapMut<'a, Vector2d>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn Map_Vector2d_new<'a>(data: *const f64) -> Map<'a, Vector2d> {
    extern "C" {
        #[link_name = "eigen_rxx$Map_Vector2d_new"]
        fn __func<'a>(_: *const f64, __return: *mut Map<'a, Vector2d>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<Map<'a, Vector2d>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn MapMut_Vector3d_new<'a>(data: *mut f64) -> MapMut<'a, Vector3d> {
    extern "C" {
        #[link_name = "eigen_rxx$MapMut_Vector3d_new"]
        fn __func<'a>(_: *mut f64, __return: *mut MapMut<'a, Vector3d>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<MapMut<'a, Vector3d>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn Map_Vector3d_new<'a>(data: *const f64) -> Map<'a, Vector3d> {
    extern "C" {
        #[link_name = "eigen_rxx$Map_Vector3d_new"]
        fn __func<'a>(_: *const f64, __return: *mut Map<'a, Vector3d>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<Map<'a, Vector3d>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn MapMut_Vector4d_new<'a>(data: *mut f64) -> MapMut<'a, Vector4d> {
    extern "C" {
        #[link_name = "eigen_rxx$MapMut_Vector4d_new"]
        fn __func<'a>(_: *mut f64, __return: *mut MapMut<'a, Vector4d>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<MapMut<'a, Vector4d>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn Map_Vector4d_new<'a>(data: *const f64) -> Map<'a, Vector4d> {
    extern "C" {
        #[link_name = "eigen_rxx$Map_Vector4d_new"]
        fn __func<'a>(_: *const f64, __return: *mut Map<'a, Vector4d>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<Map<'a, Vector4d>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn MapMut_Matrix2f_new<'a>(data: *mut f32) -> MapMut<'a, Matrix2f> {
    extern "C" {
        #[link_name = "eigen_rxx$MapMut_Matrix2f_new"]
        fn __func<'a>(_: *mut f32, __return: *mut MapMut<'a, Matrix2f>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<MapMut<'a, Matrix2f>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn Map_Matrix2f_new<'a>(data: *const f32) -> Map<'a, Matrix2f> {
    extern "C" {
        #[link_name = "eigen_rxx$Map_Matrix2f_new"]
        fn __func<'a>(_: *const f32, __return: *mut Map<'a, Matrix2f>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<Map<'a, Matrix2f>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn MapMut_Matrix3f_new<'a>(data: *mut f32) -> MapMut<'a, Matrix3f> {
    extern "C" {
        #[link_name = "eigen_rxx$MapMut_Matrix3f_new"]
        fn __func<'a>(_: *mut f32, __return: *mut MapMut<'a, Matrix3f>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<MapMut<'a, Matrix3f>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn Map_Matrix3f_new<'a>(data: *const f32) -> Map<'a, Matrix3f> {
    extern "C" {
        #[link_name = "eigen_rxx$Map_Matrix3f_new"]
        fn __func<'a>(_: *const f32, __return: *mut Map<'a, Matrix3f>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<Map<'a, Matrix3f>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn MapMut_Matrix4f_new<'a>(data: *mut f32) -> MapMut<'a, Matrix4f> {
    extern "C" {
        #[link_name = "eigen_rxx$MapMut_Matrix4f_new"]
        fn __func<'a>(_: *mut f32, __return: *mut MapMut<'a, Matrix4f>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<MapMut<'a, Matrix4f>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn Map_Matrix4f_new<'a>(data: *const f32) -> Map<'a, Matrix4f> {
    extern "C" {
        #[link_name = "eigen_rxx$Map_Matrix4f_new"]
        fn __func<'a>(_: *const f32, __return: *mut Map<'a, Matrix4f>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<Map<'a, Matrix4f>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn MapMut_Matrix2x3f_new<'a>(data: *mut f32) -> MapMut<'a, Matrix2x3f> {
    extern "C" {
        #[link_name = "eigen_rxx$MapMut_Matrix2x3f_new"]
        fn __func<'a>(_: *mut f32, __return: *mut MapMut<'a, Matrix2x3f>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<MapMut<'a, Matrix2x3f>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn Map_Matrix2x3f_new<'a>(data: *const f32) -> Map<'a, Matrix2x3f> {
    extern "C" {
        #[link_name = "eigen_rxx$Map_Matrix2x3f_new"]
        fn __func<'a>(_: *const f32, __return: *mut Map<'a, Matrix2x3f>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<Map<'a, Matrix2x3f>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn MapMut_Vector2f_new<'a>(data: *mut f32) -> MapMut<'a, Vector2f> {
    extern "C" {
        #[link_name = "eigen_rxx$MapMut_Vector2f_new"]
        fn __func<'a>(_: *mut f32, __return: *mut MapMut<'a, Vector2f>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<MapMut<'a, Vector2f>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn Map_Vector2f_new<'a>(data: *const f32) -> Map<'a, Vector2f> {
    extern "C" {
        #[link_name = "eigen_rxx$Map_Vector2f_new"]
        fn __func<'a>(_: *const f32, __return: *mut Map<'a, Vector2f>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<Map<'a, Vector2f>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn MapMut_Vector3f_new<'a>(data: *mut f32) -> MapMut<'a, Vector3f> {
    extern "C" {
        #[link_name = "eigen_rxx$MapMut_Vector3f_new"]
        fn __func<'a>(_: *mut f32, __return: *mut MapMut<'a, Vector3f>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<MapMut<'a, Vector3f>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn Map_Vector3f_new<'a>(data: *const f32) -> Map<'a, Vector3f> {
    extern "C" {
        #[link_name = "eigen_rxx$Map_Vector3f_new"]
        fn __func<'a>(_: *const f32, __return: *mut Map<'a, Vector3f>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<Map<'a, Vector3f>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn MapMut_Vector4f_new<'a>(data: *mut f32) -> MapMut<'a, Vector4f> {
    extern "C" {
        #[link_name = "eigen_rxx$MapMut_Vector4f_new"]
        fn __func<'a>(_: *mut f32, __return: *mut MapMut<'a, Vector4f>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<MapMut<'a, Vector4f>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn Map_Vector4f_new<'a>(data: *const f32) -> Map<'a, Vector4f> {
    extern "C" {
        #[link_name = "eigen_rxx$Map_Vector4f_new"]
        fn __func<'a>(_: *const f32, __return: *mut Map<'a, Vector4f>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<Map<'a, Vector4f>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn MapMut_Matrix2i_new<'a>(data: *mut i32) -> MapMut<'a, Matrix2i> {
    extern "C" {
        #[link_name = "eigen_rxx$MapMut_Matrix2i_new"]
        fn __func<'a>(_: *mut i32, __return: *mut MapMut<'a, Matrix2i>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<MapMut<'a, Matrix2i>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn Map_Matrix2i_new<'a>(data: *const i32) -> Map<'a, Matrix2i> {
    extern "C" {
        #[link_name = "eigen_rxx$Map_Matrix2i_new"]
        fn __func<'a>(_: *const i32, __return: *mut Map<'a, Matrix2i>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<Map<'a, Matrix2i>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn MapMut_Matrix3i_new<'a>(data: *mut i32) -> MapMut<'a, Matrix3i> {
    extern "C" {
        #[link_name = "eigen_rxx$MapMut_Matrix3i_new"]
        fn __func<'a>(_: *mut i32, __return: *mut MapMut<'a, Matrix3i>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<MapMut<'a, Matrix3i>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn Map_Matrix3i_new<'a>(data: *const i32) -> Map<'a, Matrix3i> {
    extern "C" {
        #[link_name = "eigen_rxx$Map_Matrix3i_new"]
        fn __func<'a>(_: *const i32, __return: *mut Map<'a, Matrix3i>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<Map<'a, Matrix3i>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn MapMut_Matrix4i_new<'a>(data: *mut i32) -> MapMut<'a, Matrix4i> {
    extern "C" {
        #[link_name = "eigen_rxx$MapMut_Matrix4i_new"]
        fn __func<'a>(_: *mut i32, __return: *mut MapMut<'a, Matrix4i>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<MapMut<'a, Matrix4i>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn Map_Matrix4i_new<'a>(data: *const i32) -> Map<'a, Matrix4i> {
    extern "C" {
        #[link_name = "eigen_rxx$Map_Matrix4i_new"]
        fn __func<'a>(_: *const i32, __return: *mut Map<'a, Matrix4i>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<Map<'a, Matrix4i>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn MapMut_Matrix2x3i_new<'a>(data: *mut i32) -> MapMut<'a, Matrix2x3i> {
    extern "C" {
        #[link_name = "eigen_rxx$MapMut_Matrix2x3i_new"]
        fn __func<'a>(_: *mut i32, __return: *mut MapMut<'a, Matrix2x3i>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<MapMut<'a, Matrix2x3i>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn Map_Matrix2x3i_new<'a>(data: *const i32) -> Map<'a, Matrix2x3i> {
    extern "C" {
        #[link_name = "eigen_rxx$Map_Matrix2x3i_new"]
        fn __func<'a>(_: *const i32, __return: *mut Map<'a, Matrix2x3i>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<Map<'a, Matrix2x3i>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn MapMut_Vector2i_new<'a>(data: *mut i32) -> MapMut<'a, Vector2i> {
    extern "C" {
        #[link_name = "eigen_rxx$MapMut_Vector2i_new"]
        fn __func<'a>(_: *mut i32, __return: *mut MapMut<'a, Vector2i>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<MapMut<'a, Vector2i>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn Map_Vector2i_new<'a>(data: *const i32) -> Map<'a, Vector2i> {
    extern "C" {
        #[link_name = "eigen_rxx$Map_Vector2i_new"]
        fn __func<'a>(_: *const i32, __return: *mut Map<'a, Vector2i>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<Map<'a, Vector2i>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn MapMut_Vector3i_new<'a>(data: *mut i32) -> MapMut<'a, Vector3i> {
    extern "C" {
        #[link_name = "eigen_rxx$MapMut_Vector3i_new"]
        fn __func<'a>(_: *mut i32, __return: *mut MapMut<'a, Vector3i>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<MapMut<'a, Vector3i>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn Map_Vector3i_new<'a>(data: *const i32) -> Map<'a, Vector3i> {
    extern "C" {
        #[link_name = "eigen_rxx$Map_Vector3i_new"]
        fn __func<'a>(_: *const i32, __return: *mut Map<'a, Vector3i>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<Map<'a, Vector3i>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn MapMut_Vector4i_new<'a>(data: *mut i32) -> MapMut<'a, Vector4i> {
    extern "C" {
        #[link_name = "eigen_rxx$MapMut_Vector4i_new"]
        fn __func<'a>(_: *mut i32, __return: *mut MapMut<'a, Vector4i>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<MapMut<'a, Vector4i>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn Map_Vector4i_new<'a>(data: *const i32) -> Map<'a, Vector4i> {
    extern "C" {
        #[link_name = "eigen_rxx$Map_Vector4i_new"]
        fn __func<'a>(_: *const i32, __return: *mut Map<'a, Vector4i>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<Map<'a, Vector4i>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
impl Quaterniond {
    pub fn normalized(&self) -> Quaterniond {
        extern "C" {
            #[link_name = "eigen_rxx$Quaterniond$normalized"]
            fn __func(_: &Quaterniond, __return: *mut Quaterniond);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<Quaterniond>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl Quaterniond {
    pub fn normalize(self: &mut Self) {
        extern "C" {
            #[link_name = "eigen_rxx$Quaterniond$normalize"]
            fn __func(_: &mut Quaterniond);
        }
        unsafe {
            __func(self);
        }
    }
}
impl Quaterniond {
    pub fn inverse(&self) -> Quaterniond {
        extern "C" {
            #[link_name = "eigen_rxx$Quaterniond$inverse"]
            fn __func(_: &Quaterniond, __return: *mut Quaterniond);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<Quaterniond>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl Quaterniond {
    pub fn mul(&self, other: &Quaterniond) -> Quaterniond {
        extern "C" {
            #[link_name = "eigen_rxx$Quaterniond$mul"]
            fn __func(_: &Quaterniond, other: &Quaterniond, __return: *mut Quaterniond);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<Quaterniond>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, other, __return);
            ret.assume_init()
        }
    }
}

pub fn MapMut_Quaterniond_new<'a>(data: *mut f64) -> MapMut<'a, Quaterniond> {
    extern "C" {
        #[link_name = "eigen_rxx$MapMut_Quaterniond_new"]
        fn __func<'a>(_: *mut f64, __return: *mut MapMut<'a, Quaterniond>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<MapMut<'a, Quaterniond>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn Map_Quaterniond_new<'a>(data: *const f64) -> Map<'a, Quaterniond> {
    extern "C" {
        #[link_name = "eigen_rxx$Map_Quaterniond_new"]
        fn __func<'a>(_: *const f64, __return: *mut Map<'a, Quaterniond>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<Map<'a, Quaterniond>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
impl AngleAxisd {
    pub fn inverse(&self) -> AngleAxisd {
        extern "C" {
            #[link_name = "eigen_rxx$AngleAxisd$inverse"]
            fn __func(_: &AngleAxisd, __return: *mut AngleAxisd);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<AngleAxisd>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl AngleAxisd {
    pub fn mul(&self, other: &AngleAxisd) -> Quaterniond {
        extern "C" {
            #[link_name = "eigen_rxx$AngleAxisd$mul"]
            fn __func(_: &AngleAxisd, other: &AngleAxisd, __return: *mut Quaterniond);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<Quaterniond>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, other, __return);
            ret.assume_init()
        }
    }
}

impl Quaternionf {
    pub fn normalized(&self) -> Quaternionf {
        extern "C" {
            #[link_name = "eigen_rxx$Quaternionf$normalized"]
            fn __func(_: &Quaternionf, __return: *mut Quaternionf);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<Quaternionf>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl Quaternionf {
    pub fn normalize(self: &mut Self) {
        extern "C" {
            #[link_name = "eigen_rxx$Quaternionf$normalize"]
            fn __func(_: &mut Quaternionf);
        }
        unsafe {
            __func(self);
        }
    }
}
impl Quaternionf {
    pub fn inverse(&self) -> Quaternionf {
        extern "C" {
            #[link_name = "eigen_rxx$Quaternionf$inverse"]
            fn __func(_: &Quaternionf, __return: *mut Quaternionf);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<Quaternionf>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl Quaternionf {
    pub fn mul(&self, other: &Quaternionf) -> Quaternionf {
        extern "C" {
            #[link_name = "eigen_rxx$Quaternionf$mul"]
            fn __func(_: &Quaternionf, other: &Quaternionf, __return: *mut Quaternionf);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<Quaternionf>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, other, __return);
            ret.assume_init()
        }
    }
}

pub fn MapMut_Quaternionf_new<'a>(data: *mut f32) -> MapMut<'a, Quaternionf> {
    extern "C" {
        #[link_name = "eigen_rxx$MapMut_Quaternionf_new"]
        fn __func<'a>(_: *mut f32, __return: *mut MapMut<'a, Quaternionf>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<MapMut<'a, Quaternionf>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
pub fn Map_Quaternionf_new<'a>(data: *const f32) -> Map<'a, Quaternionf> {
    extern "C" {
        #[link_name = "eigen_rxx$Map_Quaternionf_new"]
        fn __func<'a>(_: *const f32, __return: *mut Map<'a, Quaternionf>);
    }
    unsafe {
        let mut ret = std::mem::MaybeUninit::<Map<'a, Quaternionf>>::uninit();
        let mut __return = ret.as_mut_ptr();
        __func(data, __return);
        ret.assume_init()
    }
}
impl AngleAxisf {
    pub fn inverse(&self) -> AngleAxisf {
        extern "C" {
            #[link_name = "eigen_rxx$AngleAxisf$inverse"]
            fn __func(_: &AngleAxisf, __return: *mut AngleAxisf);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<AngleAxisf>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl AngleAxisf {
    pub fn mul(&self, other: &AngleAxisf) -> Quaternionf {
        extern "C" {
            #[link_name = "eigen_rxx$AngleAxisf$mul"]
            fn __func(_: &AngleAxisf, other: &AngleAxisf, __return: *mut Quaternionf);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<Quaternionf>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, other, __return);
            ret.assume_init()
        }
    }
}

impl Matrix2d {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Matrix2d"]
            fn __func(_: &Matrix2d, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> MapMut<'a, Matrix2d> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_MapMut_Matrix2d"]
            fn __func<'a>(_: &MapMut<'a, Matrix2d>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> Map<'a, Matrix2d> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Map_Matrix2d"]
            fn __func<'a>(_: &Map<'a, Matrix2d>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl Matrix3d {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Matrix3d"]
            fn __func(_: &Matrix3d, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> MapMut<'a, Matrix3d> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_MapMut_Matrix3d"]
            fn __func<'a>(_: &MapMut<'a, Matrix3d>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> Map<'a, Matrix3d> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Map_Matrix3d"]
            fn __func<'a>(_: &Map<'a, Matrix3d>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl Matrix4d {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Matrix4d"]
            fn __func(_: &Matrix4d, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> MapMut<'a, Matrix4d> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_MapMut_Matrix4d"]
            fn __func<'a>(_: &MapMut<'a, Matrix4d>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> Map<'a, Matrix4d> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Map_Matrix4d"]
            fn __func<'a>(_: &Map<'a, Matrix4d>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl Matrix2x3d {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Matrix2x3d"]
            fn __func(_: &Matrix2x3d, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> MapMut<'a, Matrix2x3d> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_MapMut_Matrix2x3d"]
            fn __func<'a>(_: &MapMut<'a, Matrix2x3d>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> Map<'a, Matrix2x3d> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Map_Matrix2x3d"]
            fn __func<'a>(_: &Map<'a, Matrix2x3d>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl Vector2d {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Vector2d"]
            fn __func(_: &Vector2d, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> MapMut<'a, Vector2d> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_MapMut_Vector2d"]
            fn __func<'a>(_: &MapMut<'a, Vector2d>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> Map<'a, Vector2d> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Map_Vector2d"]
            fn __func<'a>(_: &Map<'a, Vector2d>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl Vector3d {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Vector3d"]
            fn __func(_: &Vector3d, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> MapMut<'a, Vector3d> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_MapMut_Vector3d"]
            fn __func<'a>(_: &MapMut<'a, Vector3d>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> Map<'a, Vector3d> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Map_Vector3d"]
            fn __func<'a>(_: &Map<'a, Vector3d>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl Vector4d {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Vector4d"]
            fn __func(_: &Vector4d, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> MapMut<'a, Vector4d> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_MapMut_Vector4d"]
            fn __func<'a>(_: &MapMut<'a, Vector4d>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> Map<'a, Vector4d> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Map_Vector4d"]
            fn __func<'a>(_: &Map<'a, Vector4d>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl Matrix2f {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Matrix2f"]
            fn __func(_: &Matrix2f, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> MapMut<'a, Matrix2f> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_MapMut_Matrix2f"]
            fn __func<'a>(_: &MapMut<'a, Matrix2f>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> Map<'a, Matrix2f> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Map_Matrix2f"]
            fn __func<'a>(_: &Map<'a, Matrix2f>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl Matrix3f {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Matrix3f"]
            fn __func(_: &Matrix3f, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> MapMut<'a, Matrix3f> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_MapMut_Matrix3f"]
            fn __func<'a>(_: &MapMut<'a, Matrix3f>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> Map<'a, Matrix3f> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Map_Matrix3f"]
            fn __func<'a>(_: &Map<'a, Matrix3f>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl Matrix4f {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Matrix4f"]
            fn __func(_: &Matrix4f, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> MapMut<'a, Matrix4f> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_MapMut_Matrix4f"]
            fn __func<'a>(_: &MapMut<'a, Matrix4f>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> Map<'a, Matrix4f> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Map_Matrix4f"]
            fn __func<'a>(_: &Map<'a, Matrix4f>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl Matrix2x3f {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Matrix2x3f"]
            fn __func(_: &Matrix2x3f, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> MapMut<'a, Matrix2x3f> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_MapMut_Matrix2x3f"]
            fn __func<'a>(_: &MapMut<'a, Matrix2x3f>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> Map<'a, Matrix2x3f> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Map_Matrix2x3f"]
            fn __func<'a>(_: &Map<'a, Matrix2x3f>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl Vector2f {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Vector2f"]
            fn __func(_: &Vector2f, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> MapMut<'a, Vector2f> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_MapMut_Vector2f"]
            fn __func<'a>(_: &MapMut<'a, Vector2f>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> Map<'a, Vector2f> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Map_Vector2f"]
            fn __func<'a>(_: &Map<'a, Vector2f>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl Vector3f {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Vector3f"]
            fn __func(_: &Vector3f, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> MapMut<'a, Vector3f> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_MapMut_Vector3f"]
            fn __func<'a>(_: &MapMut<'a, Vector3f>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> Map<'a, Vector3f> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Map_Vector3f"]
            fn __func<'a>(_: &Map<'a, Vector3f>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl Vector4f {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Vector4f"]
            fn __func(_: &Vector4f, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> MapMut<'a, Vector4f> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_MapMut_Vector4f"]
            fn __func<'a>(_: &MapMut<'a, Vector4f>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> Map<'a, Vector4f> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Map_Vector4f"]
            fn __func<'a>(_: &Map<'a, Vector4f>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl Matrix2i {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Matrix2i"]
            fn __func(_: &Matrix2i, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> MapMut<'a, Matrix2i> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_MapMut_Matrix2i"]
            fn __func<'a>(_: &MapMut<'a, Matrix2i>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> Map<'a, Matrix2i> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Map_Matrix2i"]
            fn __func<'a>(_: &Map<'a, Matrix2i>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl Matrix3i {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Matrix3i"]
            fn __func(_: &Matrix3i, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> MapMut<'a, Matrix3i> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_MapMut_Matrix3i"]
            fn __func<'a>(_: &MapMut<'a, Matrix3i>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> Map<'a, Matrix3i> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Map_Matrix3i"]
            fn __func<'a>(_: &Map<'a, Matrix3i>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl Matrix4i {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Matrix4i"]
            fn __func(_: &Matrix4i, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> MapMut<'a, Matrix4i> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_MapMut_Matrix4i"]
            fn __func<'a>(_: &MapMut<'a, Matrix4i>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> Map<'a, Matrix4i> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Map_Matrix4i"]
            fn __func<'a>(_: &Map<'a, Matrix4i>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl Matrix2x3i {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Matrix2x3i"]
            fn __func(_: &Matrix2x3i, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> MapMut<'a, Matrix2x3i> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_MapMut_Matrix2x3i"]
            fn __func<'a>(_: &MapMut<'a, Matrix2x3i>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> Map<'a, Matrix2x3i> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Map_Matrix2x3i"]
            fn __func<'a>(_: &Map<'a, Matrix2x3i>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl Vector2i {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Vector2i"]
            fn __func(_: &Vector2i, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> MapMut<'a, Vector2i> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_MapMut_Vector2i"]
            fn __func<'a>(_: &MapMut<'a, Vector2i>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> Map<'a, Vector2i> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Map_Vector2i"]
            fn __func<'a>(_: &Map<'a, Vector2i>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl Vector3i {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Vector3i"]
            fn __func(_: &Vector3i, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> MapMut<'a, Vector3i> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_MapMut_Vector3i"]
            fn __func<'a>(_: &MapMut<'a, Vector3i>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> Map<'a, Vector3i> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Map_Vector3i"]
            fn __func<'a>(_: &Map<'a, Vector3i>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl Vector4i {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Vector4i"]
            fn __func(_: &Vector4i, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> MapMut<'a, Vector4i> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_MapMut_Vector4i"]
            fn __func<'a>(_: &MapMut<'a, Vector4i>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl<'a> Map<'a, Vector4i> {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Map_Vector4i"]
            fn __func<'a>(_: &Map<'a, Vector4i>, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl Quaterniond {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Quaterniond"]
            fn __func(_: &Quaterniond, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}
impl Quaternionf {
    pub fn to_cxx_string(&self) -> rxx::UniquePtr<rxx::CxxString> {
        extern "C" {
            #[link_name = "to_string_Quaternionf"]
            fn __func(_: &Quaternionf, __return: *mut rxx::UniquePtr<rxx::CxxString>);
        }
        unsafe {
            let mut ret = std::mem::MaybeUninit::<rxx::UniquePtr<rxx::CxxString>>::uninit();
            let mut __return = ret.as_mut_ptr();
            __func(self, __return);
            ret.assume_init()
        }
    }
}


impl<'a> MapMut<'a, Matrix2d> {
    pub fn new(data: &'a mut [f64]) -> Self {
        MapMut_Matrix2d_new::<'a>(data.as_mut_ptr())
    }
}

impl<'a> Map<'a, Matrix2d> {
    pub fn new(data: &'a [f64]) -> Self {
        Map_Matrix2d_new::<'a>(data.as_ptr())
    }
}

impl From<na::Matrix2<f64>> for Matrix2d {
    fn from(v: na::Matrix2<f64>) -> Self {
        Self {
            data: v.as_slice().try_into().unwrap()
        }
    }
}

impl From<Matrix2d> for na::Matrix2<f64> {
    fn from(v: Matrix2d) -> Self {
        let d = unsafe {
            std::mem::transmute::<[f64; 4], [[f64; 2]; 2]>(v.data)
        };
        Self::from_data(na::ArrayStorage(d))
    }
}
impl<'a> From<Map<'a, Matrix2d>> for na::MatrixSlice2<'a, f64> {
    fn from(v: Map<'a, Matrix2d>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorage::from_raw_parts(v.data, (Dim::from_usize(2), Dim::from_usize(2)), (Dim::from_usize(1), Dim::from_usize(2)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::MatrixSlice2<'a, f64>> for Map<'a, Matrix2d> {
    fn from(v: na::MatrixSlice2<'a, f64>) -> Self {
        Self::new(v.data.into_slice())
    }
}
impl<'a> From<MapMut<'a, Matrix2d>> for na::MatrixSliceMut2<'a, f64> {
    fn from(v: MapMut<'a, Matrix2d>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorageMut::from_raw_parts(v.data, (Dim::from_usize(2), Dim::from_usize(2)), (Dim::from_usize(1), Dim::from_usize(2)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::MatrixSliceMut2<'a, f64>> for MapMut<'a, Matrix2d> {
    fn from(v: na::MatrixSliceMut2<'a, f64>) -> Self {
        Self::new(v.data.into_slice_mut())
    }
}impl<'a> MapMut<'a, Matrix3d> {
    pub fn new(data: &'a mut [f64]) -> Self {
        MapMut_Matrix3d_new::<'a>(data.as_mut_ptr())
    }
}

impl<'a> Map<'a, Matrix3d> {
    pub fn new(data: &'a [f64]) -> Self {
        Map_Matrix3d_new::<'a>(data.as_ptr())
    }
}

impl From<na::Matrix3<f64>> for Matrix3d {
    fn from(v: na::Matrix3<f64>) -> Self {
        Self {
            data: v.as_slice().try_into().unwrap()
        }
    }
}

impl From<Matrix3d> for na::Matrix3<f64> {
    fn from(v: Matrix3d) -> Self {
        let d = unsafe {
            std::mem::transmute::<[f64; 9], [[f64; 3]; 3]>(v.data)
        };
        Self::from_data(na::ArrayStorage(d))
    }
}
impl<'a> From<Map<'a, Matrix3d>> for na::MatrixSlice3<'a, f64> {
    fn from(v: Map<'a, Matrix3d>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorage::from_raw_parts(v.data, (Dim::from_usize(3), Dim::from_usize(3)), (Dim::from_usize(1), Dim::from_usize(3)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::MatrixSlice3<'a, f64>> for Map<'a, Matrix3d> {
    fn from(v: na::MatrixSlice3<'a, f64>) -> Self {
        Self::new(v.data.into_slice())
    }
}
impl<'a> From<MapMut<'a, Matrix3d>> for na::MatrixSliceMut3<'a, f64> {
    fn from(v: MapMut<'a, Matrix3d>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorageMut::from_raw_parts(v.data, (Dim::from_usize(3), Dim::from_usize(3)), (Dim::from_usize(1), Dim::from_usize(3)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::MatrixSliceMut3<'a, f64>> for MapMut<'a, Matrix3d> {
    fn from(v: na::MatrixSliceMut3<'a, f64>) -> Self {
        Self::new(v.data.into_slice_mut())
    }
}impl<'a> MapMut<'a, Matrix4d> {
    pub fn new(data: &'a mut [f64]) -> Self {
        MapMut_Matrix4d_new::<'a>(data.as_mut_ptr())
    }
}

impl<'a> Map<'a, Matrix4d> {
    pub fn new(data: &'a [f64]) -> Self {
        Map_Matrix4d_new::<'a>(data.as_ptr())
    }
}

impl From<na::Matrix4<f64>> for Matrix4d {
    fn from(v: na::Matrix4<f64>) -> Self {
        Self {
            data: v.as_slice().try_into().unwrap()
        }
    }
}

impl From<Matrix4d> for na::Matrix4<f64> {
    fn from(v: Matrix4d) -> Self {
        let d = unsafe {
            std::mem::transmute::<[f64; 16], [[f64; 4]; 4]>(v.data)
        };
        Self::from_data(na::ArrayStorage(d))
    }
}
impl<'a> From<Map<'a, Matrix4d>> for na::MatrixSlice4<'a, f64> {
    fn from(v: Map<'a, Matrix4d>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorage::from_raw_parts(v.data, (Dim::from_usize(4), Dim::from_usize(4)), (Dim::from_usize(1), Dim::from_usize(4)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::MatrixSlice4<'a, f64>> for Map<'a, Matrix4d> {
    fn from(v: na::MatrixSlice4<'a, f64>) -> Self {
        Self::new(v.data.into_slice())
    }
}
impl<'a> From<MapMut<'a, Matrix4d>> for na::MatrixSliceMut4<'a, f64> {
    fn from(v: MapMut<'a, Matrix4d>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorageMut::from_raw_parts(v.data, (Dim::from_usize(4), Dim::from_usize(4)), (Dim::from_usize(1), Dim::from_usize(4)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::MatrixSliceMut4<'a, f64>> for MapMut<'a, Matrix4d> {
    fn from(v: na::MatrixSliceMut4<'a, f64>) -> Self {
        Self::new(v.data.into_slice_mut())
    }
}impl<'a> MapMut<'a, Matrix2x3d> {
    pub fn new(data: &'a mut [f64]) -> Self {
        MapMut_Matrix2x3d_new::<'a>(data.as_mut_ptr())
    }
}

impl<'a> Map<'a, Matrix2x3d> {
    pub fn new(data: &'a [f64]) -> Self {
        Map_Matrix2x3d_new::<'a>(data.as_ptr())
    }
}

impl From<na::Matrix2x3<f64>> for Matrix2x3d {
    fn from(v: na::Matrix2x3<f64>) -> Self {
        Self {
            data: v.as_slice().try_into().unwrap()
        }
    }
}

impl From<Matrix2x3d> for na::Matrix2x3<f64> {
    fn from(v: Matrix2x3d) -> Self {
        let d = unsafe {
            std::mem::transmute::<[f64; 6], [[f64; 2]; 3]>(v.data)
        };
        Self::from_data(na::ArrayStorage(d))
    }
}
impl<'a> From<Map<'a, Matrix2x3d>> for na::MatrixSlice2x3<'a, f64> {
    fn from(v: Map<'a, Matrix2x3d>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorage::from_raw_parts(v.data, (Dim::from_usize(2), Dim::from_usize(3)), (Dim::from_usize(1), Dim::from_usize(3)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::MatrixSlice2x3<'a, f64>> for Map<'a, Matrix2x3d> {
    fn from(v: na::MatrixSlice2x3<'a, f64>) -> Self {
        Self::new(v.data.into_slice())
    }
}
impl<'a> From<MapMut<'a, Matrix2x3d>> for na::MatrixSliceMut2x3<'a, f64> {
    fn from(v: MapMut<'a, Matrix2x3d>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorageMut::from_raw_parts(v.data, (Dim::from_usize(2), Dim::from_usize(3)), (Dim::from_usize(1), Dim::from_usize(3)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::MatrixSliceMut2x3<'a, f64>> for MapMut<'a, Matrix2x3d> {
    fn from(v: na::MatrixSliceMut2x3<'a, f64>) -> Self {
        Self::new(v.data.into_slice_mut())
    }
}impl<'a> MapMut<'a, Vector2d> {
    pub fn new(data: &'a mut [f64]) -> Self {
        MapMut_Vector2d_new::<'a>(data.as_mut_ptr())
    }
}

impl<'a> Map<'a, Vector2d> {
    pub fn new(data: &'a [f64]) -> Self {
        Map_Vector2d_new::<'a>(data.as_ptr())
    }
}

impl From<na::Vector2<f64>> for Vector2d {
    fn from(v: na::Vector2<f64>) -> Self {
        Self {
            data: v.as_slice().try_into().unwrap()
        }
    }
}

impl From<Vector2d> for na::Vector2<f64> {
    fn from(v: Vector2d) -> Self {
        let d = unsafe {
            std::mem::transmute::<[f64; 2], [[f64; 2]; 1]>(v.data)
        };
        Self::from_data(na::ArrayStorage(d))
    }
}
impl<'a> From<Map<'a, Vector2d>> for na::VectorSlice2<'a, f64> {
    fn from(v: Map<'a, Vector2d>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorage::from_raw_parts(v.data, (Dim::from_usize(2), Dim::from_usize(1)), (Dim::from_usize(1), Dim::from_usize(1)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::VectorSlice2<'a, f64>> for Map<'a, Vector2d> {
    fn from(v: na::VectorSlice2<'a, f64>) -> Self {
        Self::new(v.data.into_slice())
    }
}
impl<'a> From<MapMut<'a, Vector2d>> for na::VectorSliceMut2<'a, f64> {
    fn from(v: MapMut<'a, Vector2d>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorageMut::from_raw_parts(v.data, (Dim::from_usize(2), Dim::from_usize(1)), (Dim::from_usize(1), Dim::from_usize(1)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::VectorSliceMut2<'a, f64>> for MapMut<'a, Vector2d> {
    fn from(v: na::VectorSliceMut2<'a, f64>) -> Self {
        Self::new(v.data.into_slice_mut())
    }
}impl<'a> MapMut<'a, Vector3d> {
    pub fn new(data: &'a mut [f64]) -> Self {
        MapMut_Vector3d_new::<'a>(data.as_mut_ptr())
    }
}

impl<'a> Map<'a, Vector3d> {
    pub fn new(data: &'a [f64]) -> Self {
        Map_Vector3d_new::<'a>(data.as_ptr())
    }
}

impl From<na::Vector3<f64>> for Vector3d {
    fn from(v: na::Vector3<f64>) -> Self {
        Self {
            data: v.as_slice().try_into().unwrap()
        }
    }
}

impl From<Vector3d> for na::Vector3<f64> {
    fn from(v: Vector3d) -> Self {
        let d = unsafe {
            std::mem::transmute::<[f64; 3], [[f64; 3]; 1]>(v.data)
        };
        Self::from_data(na::ArrayStorage(d))
    }
}
impl<'a> From<Map<'a, Vector3d>> for na::VectorSlice3<'a, f64> {
    fn from(v: Map<'a, Vector3d>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorage::from_raw_parts(v.data, (Dim::from_usize(3), Dim::from_usize(1)), (Dim::from_usize(1), Dim::from_usize(1)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::VectorSlice3<'a, f64>> for Map<'a, Vector3d> {
    fn from(v: na::VectorSlice3<'a, f64>) -> Self {
        Self::new(v.data.into_slice())
    }
}
impl<'a> From<MapMut<'a, Vector3d>> for na::VectorSliceMut3<'a, f64> {
    fn from(v: MapMut<'a, Vector3d>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorageMut::from_raw_parts(v.data, (Dim::from_usize(3), Dim::from_usize(1)), (Dim::from_usize(1), Dim::from_usize(1)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::VectorSliceMut3<'a, f64>> for MapMut<'a, Vector3d> {
    fn from(v: na::VectorSliceMut3<'a, f64>) -> Self {
        Self::new(v.data.into_slice_mut())
    }
}impl<'a> MapMut<'a, Vector4d> {
    pub fn new(data: &'a mut [f64]) -> Self {
        MapMut_Vector4d_new::<'a>(data.as_mut_ptr())
    }
}

impl<'a> Map<'a, Vector4d> {
    pub fn new(data: &'a [f64]) -> Self {
        Map_Vector4d_new::<'a>(data.as_ptr())
    }
}

impl From<na::Vector4<f64>> for Vector4d {
    fn from(v: na::Vector4<f64>) -> Self {
        Self {
            data: v.as_slice().try_into().unwrap()
        }
    }
}

impl From<Vector4d> for na::Vector4<f64> {
    fn from(v: Vector4d) -> Self {
        let d = unsafe {
            std::mem::transmute::<[f64; 4], [[f64; 4]; 1]>(v.data)
        };
        Self::from_data(na::ArrayStorage(d))
    }
}
impl<'a> From<Map<'a, Vector4d>> for na::VectorSlice4<'a, f64> {
    fn from(v: Map<'a, Vector4d>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorage::from_raw_parts(v.data, (Dim::from_usize(4), Dim::from_usize(1)), (Dim::from_usize(1), Dim::from_usize(1)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::VectorSlice4<'a, f64>> for Map<'a, Vector4d> {
    fn from(v: na::VectorSlice4<'a, f64>) -> Self {
        Self::new(v.data.into_slice())
    }
}
impl<'a> From<MapMut<'a, Vector4d>> for na::VectorSliceMut4<'a, f64> {
    fn from(v: MapMut<'a, Vector4d>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorageMut::from_raw_parts(v.data, (Dim::from_usize(4), Dim::from_usize(1)), (Dim::from_usize(1), Dim::from_usize(1)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::VectorSliceMut4<'a, f64>> for MapMut<'a, Vector4d> {
    fn from(v: na::VectorSliceMut4<'a, f64>) -> Self {
        Self::new(v.data.into_slice_mut())
    }
}impl<'a> MapMut<'a, Matrix2f> {
    pub fn new(data: &'a mut [f32]) -> Self {
        MapMut_Matrix2f_new::<'a>(data.as_mut_ptr())
    }
}

impl<'a> Map<'a, Matrix2f> {
    pub fn new(data: &'a [f32]) -> Self {
        Map_Matrix2f_new::<'a>(data.as_ptr())
    }
}

impl From<na::Matrix2<f32>> for Matrix2f {
    fn from(v: na::Matrix2<f32>) -> Self {
        Self {
            data: v.as_slice().try_into().unwrap()
        }
    }
}

impl From<Matrix2f> for na::Matrix2<f32> {
    fn from(v: Matrix2f) -> Self {
        let d = unsafe {
            std::mem::transmute::<[f32; 4], [[f32; 2]; 2]>(v.data)
        };
        Self::from_data(na::ArrayStorage(d))
    }
}
impl<'a> From<Map<'a, Matrix2f>> for na::MatrixSlice2<'a, f32> {
    fn from(v: Map<'a, Matrix2f>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorage::from_raw_parts(v.data, (Dim::from_usize(2), Dim::from_usize(2)), (Dim::from_usize(1), Dim::from_usize(2)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::MatrixSlice2<'a, f32>> for Map<'a, Matrix2f> {
    fn from(v: na::MatrixSlice2<'a, f32>) -> Self {
        Self::new(v.data.into_slice())
    }
}
impl<'a> From<MapMut<'a, Matrix2f>> for na::MatrixSliceMut2<'a, f32> {
    fn from(v: MapMut<'a, Matrix2f>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorageMut::from_raw_parts(v.data, (Dim::from_usize(2), Dim::from_usize(2)), (Dim::from_usize(1), Dim::from_usize(2)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::MatrixSliceMut2<'a, f32>> for MapMut<'a, Matrix2f> {
    fn from(v: na::MatrixSliceMut2<'a, f32>) -> Self {
        Self::new(v.data.into_slice_mut())
    }
}impl<'a> MapMut<'a, Matrix3f> {
    pub fn new(data: &'a mut [f32]) -> Self {
        MapMut_Matrix3f_new::<'a>(data.as_mut_ptr())
    }
}

impl<'a> Map<'a, Matrix3f> {
    pub fn new(data: &'a [f32]) -> Self {
        Map_Matrix3f_new::<'a>(data.as_ptr())
    }
}

impl From<na::Matrix3<f32>> for Matrix3f {
    fn from(v: na::Matrix3<f32>) -> Self {
        Self {
            data: v.as_slice().try_into().unwrap()
        }
    }
}

impl From<Matrix3f> for na::Matrix3<f32> {
    fn from(v: Matrix3f) -> Self {
        let d = unsafe {
            std::mem::transmute::<[f32; 9], [[f32; 3]; 3]>(v.data)
        };
        Self::from_data(na::ArrayStorage(d))
    }
}
impl<'a> From<Map<'a, Matrix3f>> for na::MatrixSlice3<'a, f32> {
    fn from(v: Map<'a, Matrix3f>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorage::from_raw_parts(v.data, (Dim::from_usize(3), Dim::from_usize(3)), (Dim::from_usize(1), Dim::from_usize(3)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::MatrixSlice3<'a, f32>> for Map<'a, Matrix3f> {
    fn from(v: na::MatrixSlice3<'a, f32>) -> Self {
        Self::new(v.data.into_slice())
    }
}
impl<'a> From<MapMut<'a, Matrix3f>> for na::MatrixSliceMut3<'a, f32> {
    fn from(v: MapMut<'a, Matrix3f>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorageMut::from_raw_parts(v.data, (Dim::from_usize(3), Dim::from_usize(3)), (Dim::from_usize(1), Dim::from_usize(3)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::MatrixSliceMut3<'a, f32>> for MapMut<'a, Matrix3f> {
    fn from(v: na::MatrixSliceMut3<'a, f32>) -> Self {
        Self::new(v.data.into_slice_mut())
    }
}impl<'a> MapMut<'a, Matrix4f> {
    pub fn new(data: &'a mut [f32]) -> Self {
        MapMut_Matrix4f_new::<'a>(data.as_mut_ptr())
    }
}

impl<'a> Map<'a, Matrix4f> {
    pub fn new(data: &'a [f32]) -> Self {
        Map_Matrix4f_new::<'a>(data.as_ptr())
    }
}

impl From<na::Matrix4<f32>> for Matrix4f {
    fn from(v: na::Matrix4<f32>) -> Self {
        Self {
            data: v.as_slice().try_into().unwrap()
        }
    }
}

impl From<Matrix4f> for na::Matrix4<f32> {
    fn from(v: Matrix4f) -> Self {
        let d = unsafe {
            std::mem::transmute::<[f32; 16], [[f32; 4]; 4]>(v.data)
        };
        Self::from_data(na::ArrayStorage(d))
    }
}
impl<'a> From<Map<'a, Matrix4f>> for na::MatrixSlice4<'a, f32> {
    fn from(v: Map<'a, Matrix4f>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorage::from_raw_parts(v.data, (Dim::from_usize(4), Dim::from_usize(4)), (Dim::from_usize(1), Dim::from_usize(4)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::MatrixSlice4<'a, f32>> for Map<'a, Matrix4f> {
    fn from(v: na::MatrixSlice4<'a, f32>) -> Self {
        Self::new(v.data.into_slice())
    }
}
impl<'a> From<MapMut<'a, Matrix4f>> for na::MatrixSliceMut4<'a, f32> {
    fn from(v: MapMut<'a, Matrix4f>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorageMut::from_raw_parts(v.data, (Dim::from_usize(4), Dim::from_usize(4)), (Dim::from_usize(1), Dim::from_usize(4)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::MatrixSliceMut4<'a, f32>> for MapMut<'a, Matrix4f> {
    fn from(v: na::MatrixSliceMut4<'a, f32>) -> Self {
        Self::new(v.data.into_slice_mut())
    }
}impl<'a> MapMut<'a, Matrix2x3f> {
    pub fn new(data: &'a mut [f32]) -> Self {
        MapMut_Matrix2x3f_new::<'a>(data.as_mut_ptr())
    }
}

impl<'a> Map<'a, Matrix2x3f> {
    pub fn new(data: &'a [f32]) -> Self {
        Map_Matrix2x3f_new::<'a>(data.as_ptr())
    }
}

impl From<na::Matrix2x3<f32>> for Matrix2x3f {
    fn from(v: na::Matrix2x3<f32>) -> Self {
        Self {
            data: v.as_slice().try_into().unwrap()
        }
    }
}

impl From<Matrix2x3f> for na::Matrix2x3<f32> {
    fn from(v: Matrix2x3f) -> Self {
        let d = unsafe {
            std::mem::transmute::<[f32; 6], [[f32; 2]; 3]>(v.data)
        };
        Self::from_data(na::ArrayStorage(d))
    }
}
impl<'a> From<Map<'a, Matrix2x3f>> for na::MatrixSlice2x3<'a, f32> {
    fn from(v: Map<'a, Matrix2x3f>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorage::from_raw_parts(v.data, (Dim::from_usize(2), Dim::from_usize(3)), (Dim::from_usize(1), Dim::from_usize(3)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::MatrixSlice2x3<'a, f32>> for Map<'a, Matrix2x3f> {
    fn from(v: na::MatrixSlice2x3<'a, f32>) -> Self {
        Self::new(v.data.into_slice())
    }
}
impl<'a> From<MapMut<'a, Matrix2x3f>> for na::MatrixSliceMut2x3<'a, f32> {
    fn from(v: MapMut<'a, Matrix2x3f>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorageMut::from_raw_parts(v.data, (Dim::from_usize(2), Dim::from_usize(3)), (Dim::from_usize(1), Dim::from_usize(3)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::MatrixSliceMut2x3<'a, f32>> for MapMut<'a, Matrix2x3f> {
    fn from(v: na::MatrixSliceMut2x3<'a, f32>) -> Self {
        Self::new(v.data.into_slice_mut())
    }
}impl<'a> MapMut<'a, Vector2f> {
    pub fn new(data: &'a mut [f32]) -> Self {
        MapMut_Vector2f_new::<'a>(data.as_mut_ptr())
    }
}

impl<'a> Map<'a, Vector2f> {
    pub fn new(data: &'a [f32]) -> Self {
        Map_Vector2f_new::<'a>(data.as_ptr())
    }
}

impl From<na::Vector2<f32>> for Vector2f {
    fn from(v: na::Vector2<f32>) -> Self {
        Self {
            data: v.as_slice().try_into().unwrap()
        }
    }
}

impl From<Vector2f> for na::Vector2<f32> {
    fn from(v: Vector2f) -> Self {
        let d = unsafe {
            std::mem::transmute::<[f32; 2], [[f32; 2]; 1]>(v.data)
        };
        Self::from_data(na::ArrayStorage(d))
    }
}
impl<'a> From<Map<'a, Vector2f>> for na::VectorSlice2<'a, f32> {
    fn from(v: Map<'a, Vector2f>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorage::from_raw_parts(v.data, (Dim::from_usize(2), Dim::from_usize(1)), (Dim::from_usize(1), Dim::from_usize(1)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::VectorSlice2<'a, f32>> for Map<'a, Vector2f> {
    fn from(v: na::VectorSlice2<'a, f32>) -> Self {
        Self::new(v.data.into_slice())
    }
}
impl<'a> From<MapMut<'a, Vector2f>> for na::VectorSliceMut2<'a, f32> {
    fn from(v: MapMut<'a, Vector2f>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorageMut::from_raw_parts(v.data, (Dim::from_usize(2), Dim::from_usize(1)), (Dim::from_usize(1), Dim::from_usize(1)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::VectorSliceMut2<'a, f32>> for MapMut<'a, Vector2f> {
    fn from(v: na::VectorSliceMut2<'a, f32>) -> Self {
        Self::new(v.data.into_slice_mut())
    }
}impl<'a> MapMut<'a, Vector3f> {
    pub fn new(data: &'a mut [f32]) -> Self {
        MapMut_Vector3f_new::<'a>(data.as_mut_ptr())
    }
}

impl<'a> Map<'a, Vector3f> {
    pub fn new(data: &'a [f32]) -> Self {
        Map_Vector3f_new::<'a>(data.as_ptr())
    }
}

impl From<na::Vector3<f32>> for Vector3f {
    fn from(v: na::Vector3<f32>) -> Self {
        Self {
            data: v.as_slice().try_into().unwrap()
        }
    }
}

impl From<Vector3f> for na::Vector3<f32> {
    fn from(v: Vector3f) -> Self {
        let d = unsafe {
            std::mem::transmute::<[f32; 3], [[f32; 3]; 1]>(v.data)
        };
        Self::from_data(na::ArrayStorage(d))
    }
}
impl<'a> From<Map<'a, Vector3f>> for na::VectorSlice3<'a, f32> {
    fn from(v: Map<'a, Vector3f>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorage::from_raw_parts(v.data, (Dim::from_usize(3), Dim::from_usize(1)), (Dim::from_usize(1), Dim::from_usize(1)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::VectorSlice3<'a, f32>> for Map<'a, Vector3f> {
    fn from(v: na::VectorSlice3<'a, f32>) -> Self {
        Self::new(v.data.into_slice())
    }
}
impl<'a> From<MapMut<'a, Vector3f>> for na::VectorSliceMut3<'a, f32> {
    fn from(v: MapMut<'a, Vector3f>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorageMut::from_raw_parts(v.data, (Dim::from_usize(3), Dim::from_usize(1)), (Dim::from_usize(1), Dim::from_usize(1)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::VectorSliceMut3<'a, f32>> for MapMut<'a, Vector3f> {
    fn from(v: na::VectorSliceMut3<'a, f32>) -> Self {
        Self::new(v.data.into_slice_mut())
    }
}impl<'a> MapMut<'a, Vector4f> {
    pub fn new(data: &'a mut [f32]) -> Self {
        MapMut_Vector4f_new::<'a>(data.as_mut_ptr())
    }
}

impl<'a> Map<'a, Vector4f> {
    pub fn new(data: &'a [f32]) -> Self {
        Map_Vector4f_new::<'a>(data.as_ptr())
    }
}

impl From<na::Vector4<f32>> for Vector4f {
    fn from(v: na::Vector4<f32>) -> Self {
        Self {
            data: v.as_slice().try_into().unwrap()
        }
    }
}

impl From<Vector4f> for na::Vector4<f32> {
    fn from(v: Vector4f) -> Self {
        let d = unsafe {
            std::mem::transmute::<[f32; 4], [[f32; 4]; 1]>(v.data)
        };
        Self::from_data(na::ArrayStorage(d))
    }
}
impl<'a> From<Map<'a, Vector4f>> for na::VectorSlice4<'a, f32> {
    fn from(v: Map<'a, Vector4f>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorage::from_raw_parts(v.data, (Dim::from_usize(4), Dim::from_usize(1)), (Dim::from_usize(1), Dim::from_usize(1)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::VectorSlice4<'a, f32>> for Map<'a, Vector4f> {
    fn from(v: na::VectorSlice4<'a, f32>) -> Self {
        Self::new(v.data.into_slice())
    }
}
impl<'a> From<MapMut<'a, Vector4f>> for na::VectorSliceMut4<'a, f32> {
    fn from(v: MapMut<'a, Vector4f>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorageMut::from_raw_parts(v.data, (Dim::from_usize(4), Dim::from_usize(1)), (Dim::from_usize(1), Dim::from_usize(1)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::VectorSliceMut4<'a, f32>> for MapMut<'a, Vector4f> {
    fn from(v: na::VectorSliceMut4<'a, f32>) -> Self {
        Self::new(v.data.into_slice_mut())
    }
}impl<'a> MapMut<'a, Matrix2i> {
    pub fn new(data: &'a mut [i32]) -> Self {
        MapMut_Matrix2i_new::<'a>(data.as_mut_ptr())
    }
}

impl<'a> Map<'a, Matrix2i> {
    pub fn new(data: &'a [i32]) -> Self {
        Map_Matrix2i_new::<'a>(data.as_ptr())
    }
}

impl From<na::Matrix2<i32>> for Matrix2i {
    fn from(v: na::Matrix2<i32>) -> Self {
        Self {
            data: v.as_slice().try_into().unwrap()
        }
    }
}

impl From<Matrix2i> for na::Matrix2<i32> {
    fn from(v: Matrix2i) -> Self {
        let d = unsafe {
            std::mem::transmute::<[i32; 4], [[i32; 2]; 2]>(v.data)
        };
        Self::from_data(na::ArrayStorage(d))
    }
}
impl<'a> From<Map<'a, Matrix2i>> for na::MatrixSlice2<'a, i32> {
    fn from(v: Map<'a, Matrix2i>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorage::from_raw_parts(v.data, (Dim::from_usize(2), Dim::from_usize(2)), (Dim::from_usize(1), Dim::from_usize(2)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::MatrixSlice2<'a, i32>> for Map<'a, Matrix2i> {
    fn from(v: na::MatrixSlice2<'a, i32>) -> Self {
        Self::new(v.data.into_slice())
    }
}
impl<'a> From<MapMut<'a, Matrix2i>> for na::MatrixSliceMut2<'a, i32> {
    fn from(v: MapMut<'a, Matrix2i>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorageMut::from_raw_parts(v.data, (Dim::from_usize(2), Dim::from_usize(2)), (Dim::from_usize(1), Dim::from_usize(2)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::MatrixSliceMut2<'a, i32>> for MapMut<'a, Matrix2i> {
    fn from(v: na::MatrixSliceMut2<'a, i32>) -> Self {
        Self::new(v.data.into_slice_mut())
    }
}impl<'a> MapMut<'a, Matrix3i> {
    pub fn new(data: &'a mut [i32]) -> Self {
        MapMut_Matrix3i_new::<'a>(data.as_mut_ptr())
    }
}

impl<'a> Map<'a, Matrix3i> {
    pub fn new(data: &'a [i32]) -> Self {
        Map_Matrix3i_new::<'a>(data.as_ptr())
    }
}

impl From<na::Matrix3<i32>> for Matrix3i {
    fn from(v: na::Matrix3<i32>) -> Self {
        Self {
            data: v.as_slice().try_into().unwrap()
        }
    }
}

impl From<Matrix3i> for na::Matrix3<i32> {
    fn from(v: Matrix3i) -> Self {
        let d = unsafe {
            std::mem::transmute::<[i32; 9], [[i32; 3]; 3]>(v.data)
        };
        Self::from_data(na::ArrayStorage(d))
    }
}
impl<'a> From<Map<'a, Matrix3i>> for na::MatrixSlice3<'a, i32> {
    fn from(v: Map<'a, Matrix3i>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorage::from_raw_parts(v.data, (Dim::from_usize(3), Dim::from_usize(3)), (Dim::from_usize(1), Dim::from_usize(3)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::MatrixSlice3<'a, i32>> for Map<'a, Matrix3i> {
    fn from(v: na::MatrixSlice3<'a, i32>) -> Self {
        Self::new(v.data.into_slice())
    }
}
impl<'a> From<MapMut<'a, Matrix3i>> for na::MatrixSliceMut3<'a, i32> {
    fn from(v: MapMut<'a, Matrix3i>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorageMut::from_raw_parts(v.data, (Dim::from_usize(3), Dim::from_usize(3)), (Dim::from_usize(1), Dim::from_usize(3)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::MatrixSliceMut3<'a, i32>> for MapMut<'a, Matrix3i> {
    fn from(v: na::MatrixSliceMut3<'a, i32>) -> Self {
        Self::new(v.data.into_slice_mut())
    }
}impl<'a> MapMut<'a, Matrix4i> {
    pub fn new(data: &'a mut [i32]) -> Self {
        MapMut_Matrix4i_new::<'a>(data.as_mut_ptr())
    }
}

impl<'a> Map<'a, Matrix4i> {
    pub fn new(data: &'a [i32]) -> Self {
        Map_Matrix4i_new::<'a>(data.as_ptr())
    }
}

impl From<na::Matrix4<i32>> for Matrix4i {
    fn from(v: na::Matrix4<i32>) -> Self {
        Self {
            data: v.as_slice().try_into().unwrap()
        }
    }
}

impl From<Matrix4i> for na::Matrix4<i32> {
    fn from(v: Matrix4i) -> Self {
        let d = unsafe {
            std::mem::transmute::<[i32; 16], [[i32; 4]; 4]>(v.data)
        };
        Self::from_data(na::ArrayStorage(d))
    }
}
impl<'a> From<Map<'a, Matrix4i>> for na::MatrixSlice4<'a, i32> {
    fn from(v: Map<'a, Matrix4i>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorage::from_raw_parts(v.data, (Dim::from_usize(4), Dim::from_usize(4)), (Dim::from_usize(1), Dim::from_usize(4)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::MatrixSlice4<'a, i32>> for Map<'a, Matrix4i> {
    fn from(v: na::MatrixSlice4<'a, i32>) -> Self {
        Self::new(v.data.into_slice())
    }
}
impl<'a> From<MapMut<'a, Matrix4i>> for na::MatrixSliceMut4<'a, i32> {
    fn from(v: MapMut<'a, Matrix4i>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorageMut::from_raw_parts(v.data, (Dim::from_usize(4), Dim::from_usize(4)), (Dim::from_usize(1), Dim::from_usize(4)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::MatrixSliceMut4<'a, i32>> for MapMut<'a, Matrix4i> {
    fn from(v: na::MatrixSliceMut4<'a, i32>) -> Self {
        Self::new(v.data.into_slice_mut())
    }
}impl<'a> MapMut<'a, Matrix2x3i> {
    pub fn new(data: &'a mut [i32]) -> Self {
        MapMut_Matrix2x3i_new::<'a>(data.as_mut_ptr())
    }
}

impl<'a> Map<'a, Matrix2x3i> {
    pub fn new(data: &'a [i32]) -> Self {
        Map_Matrix2x3i_new::<'a>(data.as_ptr())
    }
}

impl From<na::Matrix2x3<i32>> for Matrix2x3i {
    fn from(v: na::Matrix2x3<i32>) -> Self {
        Self {
            data: v.as_slice().try_into().unwrap()
        }
    }
}

impl From<Matrix2x3i> for na::Matrix2x3<i32> {
    fn from(v: Matrix2x3i) -> Self {
        let d = unsafe {
            std::mem::transmute::<[i32; 6], [[i32; 2]; 3]>(v.data)
        };
        Self::from_data(na::ArrayStorage(d))
    }
}
impl<'a> From<Map<'a, Matrix2x3i>> for na::MatrixSlice2x3<'a, i32> {
    fn from(v: Map<'a, Matrix2x3i>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorage::from_raw_parts(v.data, (Dim::from_usize(2), Dim::from_usize(3)), (Dim::from_usize(1), Dim::from_usize(3)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::MatrixSlice2x3<'a, i32>> for Map<'a, Matrix2x3i> {
    fn from(v: na::MatrixSlice2x3<'a, i32>) -> Self {
        Self::new(v.data.into_slice())
    }
}
impl<'a> From<MapMut<'a, Matrix2x3i>> for na::MatrixSliceMut2x3<'a, i32> {
    fn from(v: MapMut<'a, Matrix2x3i>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorageMut::from_raw_parts(v.data, (Dim::from_usize(2), Dim::from_usize(3)), (Dim::from_usize(1), Dim::from_usize(3)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::MatrixSliceMut2x3<'a, i32>> for MapMut<'a, Matrix2x3i> {
    fn from(v: na::MatrixSliceMut2x3<'a, i32>) -> Self {
        Self::new(v.data.into_slice_mut())
    }
}impl<'a> MapMut<'a, Vector2i> {
    pub fn new(data: &'a mut [i32]) -> Self {
        MapMut_Vector2i_new::<'a>(data.as_mut_ptr())
    }
}

impl<'a> Map<'a, Vector2i> {
    pub fn new(data: &'a [i32]) -> Self {
        Map_Vector2i_new::<'a>(data.as_ptr())
    }
}

impl From<na::Vector2<i32>> for Vector2i {
    fn from(v: na::Vector2<i32>) -> Self {
        Self {
            data: v.as_slice().try_into().unwrap()
        }
    }
}

impl From<Vector2i> for na::Vector2<i32> {
    fn from(v: Vector2i) -> Self {
        let d = unsafe {
            std::mem::transmute::<[i32; 2], [[i32; 2]; 1]>(v.data)
        };
        Self::from_data(na::ArrayStorage(d))
    }
}
impl<'a> From<Map<'a, Vector2i>> for na::VectorSlice2<'a, i32> {
    fn from(v: Map<'a, Vector2i>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorage::from_raw_parts(v.data, (Dim::from_usize(2), Dim::from_usize(1)), (Dim::from_usize(1), Dim::from_usize(1)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::VectorSlice2<'a, i32>> for Map<'a, Vector2i> {
    fn from(v: na::VectorSlice2<'a, i32>) -> Self {
        Self::new(v.data.into_slice())
    }
}
impl<'a> From<MapMut<'a, Vector2i>> for na::VectorSliceMut2<'a, i32> {
    fn from(v: MapMut<'a, Vector2i>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorageMut::from_raw_parts(v.data, (Dim::from_usize(2), Dim::from_usize(1)), (Dim::from_usize(1), Dim::from_usize(1)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::VectorSliceMut2<'a, i32>> for MapMut<'a, Vector2i> {
    fn from(v: na::VectorSliceMut2<'a, i32>) -> Self {
        Self::new(v.data.into_slice_mut())
    }
}impl<'a> MapMut<'a, Vector3i> {
    pub fn new(data: &'a mut [i32]) -> Self {
        MapMut_Vector3i_new::<'a>(data.as_mut_ptr())
    }
}

impl<'a> Map<'a, Vector3i> {
    pub fn new(data: &'a [i32]) -> Self {
        Map_Vector3i_new::<'a>(data.as_ptr())
    }
}

impl From<na::Vector3<i32>> for Vector3i {
    fn from(v: na::Vector3<i32>) -> Self {
        Self {
            data: v.as_slice().try_into().unwrap()
        }
    }
}

impl From<Vector3i> for na::Vector3<i32> {
    fn from(v: Vector3i) -> Self {
        let d = unsafe {
            std::mem::transmute::<[i32; 3], [[i32; 3]; 1]>(v.data)
        };
        Self::from_data(na::ArrayStorage(d))
    }
}
impl<'a> From<Map<'a, Vector3i>> for na::VectorSlice3<'a, i32> {
    fn from(v: Map<'a, Vector3i>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorage::from_raw_parts(v.data, (Dim::from_usize(3), Dim::from_usize(1)), (Dim::from_usize(1), Dim::from_usize(1)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::VectorSlice3<'a, i32>> for Map<'a, Vector3i> {
    fn from(v: na::VectorSlice3<'a, i32>) -> Self {
        Self::new(v.data.into_slice())
    }
}
impl<'a> From<MapMut<'a, Vector3i>> for na::VectorSliceMut3<'a, i32> {
    fn from(v: MapMut<'a, Vector3i>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorageMut::from_raw_parts(v.data, (Dim::from_usize(3), Dim::from_usize(1)), (Dim::from_usize(1), Dim::from_usize(1)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::VectorSliceMut3<'a, i32>> for MapMut<'a, Vector3i> {
    fn from(v: na::VectorSliceMut3<'a, i32>) -> Self {
        Self::new(v.data.into_slice_mut())
    }
}impl<'a> MapMut<'a, Vector4i> {
    pub fn new(data: &'a mut [i32]) -> Self {
        MapMut_Vector4i_new::<'a>(data.as_mut_ptr())
    }
}

impl<'a> Map<'a, Vector4i> {
    pub fn new(data: &'a [i32]) -> Self {
        Map_Vector4i_new::<'a>(data.as_ptr())
    }
}

impl From<na::Vector4<i32>> for Vector4i {
    fn from(v: na::Vector4<i32>) -> Self {
        Self {
            data: v.as_slice().try_into().unwrap()
        }
    }
}

impl From<Vector4i> for na::Vector4<i32> {
    fn from(v: Vector4i) -> Self {
        let d = unsafe {
            std::mem::transmute::<[i32; 4], [[i32; 4]; 1]>(v.data)
        };
        Self::from_data(na::ArrayStorage(d))
    }
}
impl<'a> From<Map<'a, Vector4i>> for na::VectorSlice4<'a, i32> {
    fn from(v: Map<'a, Vector4i>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorage::from_raw_parts(v.data, (Dim::from_usize(4), Dim::from_usize(1)), (Dim::from_usize(1), Dim::from_usize(1)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::VectorSlice4<'a, i32>> for Map<'a, Vector4i> {
    fn from(v: na::VectorSlice4<'a, i32>) -> Self {
        Self::new(v.data.into_slice())
    }
}
impl<'a> From<MapMut<'a, Vector4i>> for na::VectorSliceMut4<'a, i32> {
    fn from(v: MapMut<'a, Vector4i>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorageMut::from_raw_parts(v.data, (Dim::from_usize(4), Dim::from_usize(1)), (Dim::from_usize(1), Dim::from_usize(1)))};

        Self::from_data(s)
    }
}

impl<'a> From<na::VectorSliceMut4<'a, i32>> for MapMut<'a, Vector4i> {
    fn from(v: na::VectorSliceMut4<'a, i32>) -> Self {
        Self::new(v.data.into_slice_mut())
    }
}

impl<'a> MapMut<'a, Quaterniond> {
    pub fn new(data: &'a mut [f64]) -> Self {
        MapMut_Quaterniond_new::<'a>(data.as_mut().as_mut_ptr())
    }
}

impl<'a> Map<'a, Quaterniond> {
    pub fn new(data: &'a [f64]) -> Self {
        Map_Quaterniond_new::<'a>(data.as_ptr())
    }
}

impl Mul for Quaterniond {
    type Output = Quaterniond;

    fn mul(self, other: Self) -> Self::Output {
	Quaterniond::mul(&self, &other)
    }
}

impl Mul for &Quaterniond {
    type Output = Quaterniond;

    fn mul(self, other: Self) -> Self::Output {
	Quaterniond::mul(self, other)
    }
}

impl From<na::Quaternion<f64>> for Quaterniond {
    fn from(v: na::Quaternion<f64>) -> Self {
        Self {
            x: v.coords.x,
            y: v.coords.y,
            z: v.coords.z,
            w: v.coords.w,
        }
    }
}

impl From<Quaterniond> for na::Quaternion<f64> {
    fn from(v: Quaterniond) -> Self {
        Self::new(v.w, v.x, v.y, v.z)
    }
}

impl<'a> From<&'a na::Quaternion<f64>> for Map<'a, Quaterniond> {
    fn from(v: &'a na::Quaternion<f64>) -> Self {
        Self::new(v.coords.as_slice())
    }
}

impl<'a> From<Map<'a, Quaterniond>> for na::Quaternion<f64> {
    fn from(v: Map<'a, Quaterniond>) -> Self {
        let d = unsafe {
            std::slice::from_raw_parts(v.data, 4)
        };
        Self::from_vector(na::Vector4::from_column_slice(d))
    }
}

impl<'a> From<&'a mut na::Quaternion<f64>> for MapMut<'a, Quaterniond> {
    fn from(v: &'a mut na::Quaternion<f64>) -> Self {
        Self::new(v.coords.as_mut_slice())
    }
}

impl<'a> From<MapMut<'a, Quaterniond>> for na::Quaternion<f64> {
    fn from(v: MapMut<'a, Quaterniond>) -> Self {
        let d = unsafe {
            std::slice::from_raw_parts(v.data, 4)
        };
        Self::from_vector(na::Vector4::from_column_slice(d))
    }
}

impl<'a> MapMut<'a, Quaternionf> {
    pub fn new(data: &'a mut [f32]) -> Self {
        MapMut_Quaternionf_new::<'a>(data.as_mut().as_mut_ptr())
    }
}

impl<'a> Map<'a, Quaternionf> {
    pub fn new(data: &'a [f32]) -> Self {
        Map_Quaternionf_new::<'a>(data.as_ptr())
    }
}

impl Mul for Quaternionf {
    type Output = Quaternionf;

    fn mul(self, other: Self) -> Self::Output {
	Quaternionf::mul(&self, &other)
    }
}

impl Mul for &Quaternionf {
    type Output = Quaternionf;

    fn mul(self, other: Self) -> Self::Output {
	Quaternionf::mul(self, other)
    }
}

impl From<na::Quaternion<f32>> for Quaternionf {
    fn from(v: na::Quaternion<f32>) -> Self {
        Self {
            x: v.coords.x,
            y: v.coords.y,
            z: v.coords.z,
            w: v.coords.w,
        }
    }
}

impl From<Quaternionf> for na::Quaternion<f32> {
    fn from(v: Quaternionf) -> Self {
        Self::new(v.w, v.x, v.y, v.z)
    }
}

impl<'a> From<&'a na::Quaternion<f32>> for Map<'a, Quaternionf> {
    fn from(v: &'a na::Quaternion<f32>) -> Self {
        Self::new(v.coords.as_slice())
    }
}

impl<'a> From<Map<'a, Quaternionf>> for na::Quaternion<f32> {
    fn from(v: Map<'a, Quaternionf>) -> Self {
        let d = unsafe {
            std::slice::from_raw_parts(v.data, 4)
        };
        Self::from_vector(na::Vector4::from_column_slice(d))
    }
}

impl<'a> From<&'a mut na::Quaternion<f32>> for MapMut<'a, Quaternionf> {
    fn from(v: &'a mut na::Quaternion<f32>) -> Self {
        Self::new(v.coords.as_mut_slice())
    }
}

impl<'a> From<MapMut<'a, Quaternionf>> for na::Quaternion<f32> {
    fn from(v: MapMut<'a, Quaternionf>) -> Self {
        let d = unsafe {
            std::slice::from_raw_parts(v.data, 4)
        };
        Self::from_vector(na::Vector4::from_column_slice(d))
    }
}



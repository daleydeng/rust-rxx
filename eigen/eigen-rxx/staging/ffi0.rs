extern crate nalgebra as na;
use na::Dim;

{% from "eigen_map.rs" import struct_eigen_map_fixed, struct_eigen_map_matx, struct_eigen_map_vecx, ffi_eigen_map_fixed, ffi_eigen_map_matx, ffi_eigen_map_vecx, impl_eigen_map_fixed, impl_eigen_map_matx, impl_eigen_map_vecx -%}
{% set objs = {
    "Matrix2d": {
        "mul": True,
        "var": 0,
        "sqr": True,
        "shape": (2, 2),
        "tp": "f64",
        "rs_own": "na::Matrix2",
        "rs_ref": "na::MatrixSlice2",
        "rs_mut_ref": "na::MatrixSliceMut2",
    },
    "Matrix2i": {
        "mul": True,
        "var": 0,
        "sqr": True,
        "shape": (2, 2),
        "tp": "i32",
        "rs_own": "na::Matrix2",
        "rs_ref": "na::MatrixSlice2",
        "rs_mut_ref": "na::MatrixSliceMut2",
    },

    "Matrix3d": {
        "mul": True,
        "var": 0,
        "sqr": True,
        "shape": (3, 3),
        "tp": "f64",
        "rs_own": "na::Matrix3",
        "rs_ref": "na::MatrixSlice3",
        "rs_mut_ref": "na::MatrixSliceMut3",
    },
    "Matrix4d": {
        "mul": True,
        "var": 0,
        "sqr": True,
        "shape": (4, 4),
        "tp": "f64",
        "rs_own": "na::Matrix4",
        "rs_ref": "na::MatrixSlice4",
        "rs_mut_ref": "na::MatrixSliceMut4",
    },
    "Matrix2x3d": {
        "mul": False,
        "var": 0,
        "sqr": False,
        "tp": "f64",
        "shape": (2, 3),
    },
    "MatrixXd": {
        "mul": True,
        "var": 2,
        "sqr": False,
        "tp": "f64",
    },
    "Vector2d": {
        "mul": False,
        "var": 0,
        "sqr": False,
        "shape": (2, 1),
        "tp": "f64",
    },
    "Vector3d": {
        "mul": False,
        "var": 0,
        "sqr": False,
        "shape": (3, 1),
        "tp": "f64",
    },
    "Vector4d": {
        "mul": False,
        "var": 0,
        "sqr": False,
        "shape": (4, 1),
        "tp": "f64",
    },
    "VectorXd": {
        "mul": False,
        "var": 1,
        "sqr": False,
        "tp": "f64",
    },
} -%}
{% set geo_objs = {
    "Quaterniond": {
        "tp": "f64",
        "rs_own": "na::Quaternion",
    },
    "AngleAxisd": {
        "tp": "f64",
        "axis": "Vector3d",
    },
} -%}

{% for o in objs.values() -%}
{% do o.update({
    "size": o["shape"][0] * o["shape"][1] if "shape" in o else 0
}) %}
{%- endfor %}

use std::ops::Mul;

pub trait MatrixTrait {
    type Scalar;
}

#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct Matrix<SCALAR, const ROWS: usize, const COLS: usize> where [(); ROWS * COLS]: {
    pub data: [SCALAR; ROWS * COLS]
}

impl<SCALAR, const ROWS: usize, const COLS: usize> MatrixTrait for Matrix<SCALAR, ROWS, COLS> where [(); ROWS * COLS]: {
    type Scalar = SCALAR;
}

#[repr(C)]
#[derive(Debug)]
pub struct Map<'a, T: MatrixTrait> {
    pub data: *const T::Scalar,
    _pd: std::marker::PhantomData<&'a ()>,
}

#[repr(C)]
#[derive(Debug)]
pub struct MapMut<'a, T: MatrixTrait> {
    pub data: *mut T::Scalar,
    _pd: std::marker::PhantomData<&'a ()>,
}

{% for i, o in objs.items() -%}
{% if o["var"] == 2 -%}
#[repr(C)]
#[derive(Debug)]
pub struct {{i}} {
    pub data: *mut {{o["tp"]}},
    pub rows: isize,
    pub cols: isize,
}
{{ impl_ExternType(i) }}
{{ struct_eigen_map_matx(i, o["tp"]) }}

{% elif o["var"] == 1 -%}
#[repr(C)]
#[derive(Debug)]
pub struct {{i}} {
    pub data: *mut {{o["tp"]}},
    pub size: isize,
}
{{ impl_ExternType(i) }}
{{ struct_eigen_map_vecx(i, o["tp"]) }}

{% else %}
pub type {{i}} = Matrix<{{o["tp"]}}, {{o["shape"][0]}}, {{o["shape"][1]}}>;
{{ impl_ExternType(i) }}

#[allow(non_camel_case_types)]
pub type Map_{{i}}<'a> = Map<'a, {{i}}>;
{{ impl_ExternType("Map_"+i, True) }}

#[allow(non_camel_case_types)]
pub type MapMut_{{i}}<'a> = MapMut<'a, {{i}}>;
{{ impl_ExternType("MapMut_"+i, True) }}

{% endif %} // var
{% endfor %} // objs

{% set i = "Quaterniond" -%}
{% set o = geo_objs[i] -%}
#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct {{i}} {
    pub x: {{o["tp"]}},
    pub y: {{o["tp"]}},
    pub z: {{o["tp"]}},
    pub w: {{o["tp"]}},
}
{{ impl_ExternType(i) }}
{{ struct_eigen_map_fixed(i, o["tp"]) }}

{% set i = "AngleAxisd" -%}
{% set o = geo_objs[i] -%}
#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct {{i}} {
    pub axis: {{o["axis"]}},
    pub angle: {{o["tp"]}},
}
{{ impl_ExternType(i) }}

#[cxx::bridge(namespace = "rxx")]
mod ffi {
    unsafe extern "C++" {
        include!("eigen_rxx/include/wrapper.hh");

        type Matrix3d = super::Matrix3d;
        type MatrixXd = super::MatrixXd;
        type VectorXd = super::VectorXd;

        {% for i, o in objs.items() -%}

        {% if o["var"] == 2 %}
        fn {{i}}_new(rows: usize, cols: usize) -> {{i}};
        fn {{i}}_clone(v: &{{i}}) -> {{i}};

        {{ ffi_eigen_map_matx(i, o["tp"]) }}

        {% elif o["var"] == 1 %}
        fn {{i}}_new(size: usize) -> {{i}};
        fn {{i}}_clone(v: &{{i}}) -> {{i}};

        {{ ffi_eigen_map_vecx(i, o["tp"]) }}

        {% elif o["var"] == 0 -%}

        {{ ffi_eigen_map_fixed(i, o["tp"]) }}

        {% endif -%}

        {% endfor %} // objs

        {% for i in ["Quaterniond"] -%}
        type {{i}} = super::{{i}};
        fn normalized(self: &{{i}}) -> {{i}};
        fn normalize(self: Pin<&mut {{i}}>);
        fn inverse(self: &{{i}}) -> {{i}};
        #[rust_name = "{{i}}_mul"]
        fn op_mul(self_: &{{i}}, other: &{{i}}) -> {{i}};
        fn toRotationMatrix(self: &{{i}}) -> Matrix3d;

        {{ ffi_eigen_map_fixed(i, o["tp"]) }}

        {% endfor %}

        {% for i in ["AngleAxisd"] -%}
        type {{i}} = super::{{i}};
        fn inverse(self: &{{i}}) -> {{i}};
        #[rust_name = "{{i}}_mul"]
        fn op_mul(self_: &{{i}}, other: &{{i}}) -> Quaterniond;
        fn toRotationMatrix(self: &{{i}}) -> Matrix3d;
        {% endfor %}

    }
}

impl VectorXd {
    pub fn new(size: usize) -> Self {
        ffi::VectorXd_new(size)
    }
}

impl Clone for VectorXd {
    fn clone(&self) -> Self {
        ffi::VectorXd_clone(self)
    }

    fn clone_from(&mut self, source: &Self) {
        *self = ffi::VectorXd_clone(source);
    }
}

impl MatrixXd {
    pub fn new(rows: usize, cols: usize) -> Self {
        ffi::MatrixXd_new(rows, cols)
    }
}

impl Clone for MatrixXd {
    fn clone(&self) -> Self {
        ffi::MatrixXd_clone(self)
    }

    fn clone_from(&mut self, source: &Self) {
        *self = ffi::MatrixXd_clone(source);
    }
}

{% for i, o in objs.items() -%}

{% if o["var"] == 0 -%}
{{ impl_eigen_map_fixed(i, o["tp"]) }}

{% if "rs_own" in o -%}
impl From<{{o["rs_own"]}}<{{o["tp"]}}>> for {{i}} {
    fn from(v: {{o["rs_own"]}}<{{o["tp"]}}>) -> Self {
        Self {
            data: v.as_slice().try_into().unwrap()
        }
    }
}

impl From<{{i}}> for {{o["rs_own"]}}<{{o["tp"]}}> {
    fn from(v: {{i}}) -> Self {
        let d = unsafe {
            std::mem::transmute::<[{{o["tp"]}}; {{o["shape"][0] * o["shape"][1]}}], [[{{o["tp"]}}; {{o["shape"][0]}}]; {{o["shape"][1]}}]>(v.data)
        };
        Self::from_data(na::ArrayStorage(d))
    }
}

{%- endif %}

{% if "rs_ref" in o -%}
impl<'a> From<{{o["rs_ref"]}}<'a, {{o["tp"]}}>> for Map_{{i}}<'a> {
    fn from(v: {{o["rs_ref"]}}<'a, {{o["tp"]}}>) -> Self {
        Self::new(v.data.into_slice())
    }
}

impl<'a> From<Map_{{i}}<'a>> for {{o["rs_ref"]}}<'a, {{o["tp"]}}> {
    fn from(v: Map_{{i}}<'a>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorage::from_raw_parts(v.data, (Dim::from_usize({{o["shape"][0]}}), Dim::from_usize({{o["shape"][1]}})), (Dim::from_usize(1), Dim::from_usize({{o["shape"][1]}})))};

        Self::from_data(s)
    }
}

{%- endif %}

{% if "rs_mut_ref" in o -%}
impl<'a> From<{{o["rs_mut_ref"]}}<'a, {{o["tp"]}}>> for MapMut_{{i}}<'a> {
    fn from(v: {{o["rs_mut_ref"]}}<'a, {{o["tp"]}}>) -> Self {
        Self::new(v.data.into_slice_mut())
    }
}

impl<'a> From<MapMut_{{i}}<'a>> for {{o["rs_mut_ref"]}}<'a, {{o["tp"]}}> {
    fn from(v: MapMut_{{i}}<'a>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorageMut::from_raw_parts(v.data, (Dim::from_usize({{o["shape"][0]}}), Dim::from_usize({{o["shape"][1]}})), (Dim::from_usize(1), Dim::from_usize({{o["shape"][1]}})))};

        Self::from_data(s)
    }
}

{%- endif %}


{% elif o["var"] == 1 -%}
{{ impl_eigen_map_vecx(i, o["tp"]) }}
{% elif o["var"] == 2 -%}
{{ impl_eigen_map_matx(i, o["tp"]) }}

{% endif %}

{% endfor %} // objs

{% set i = "Quaterniond" -%}
{% set o = geo_objs[i] -%}
impl Mul for {{i}} {
    type Output = {{i}};

    fn mul(self, other: Self) -> Self::Output {
        ffi::{{i}}_mul(&self, &other)
    }
}

impl Mul for &{{i}} {
    type Output = {{i}};

    fn mul(self, other: Self) -> Self::Output {
        ffi::{{i}}_mul(self, other)
    }
}


impl From<{{o["rs_own"]}}<{{o["tp"]}}>> for {{i}} {
    fn from(v: {{o["rs_own"]}}<{{o["tp"]}}>) -> Self {
        Self {
            x: v.coords.x,
            y: v.coords.y,
            z: v.coords.z,
            w: v.coords.w,
        }
    }
}

impl From<{{i}}> for {{o["rs_own"]}}<{{o["tp"]}}> {
    fn from(v: {{i}}) -> Self {
        Self::new(v.w, v.x, v.y, v.z)
    }
}

{{ impl_eigen_map_fixed(i, o["tp"]) }}

impl<'a> From<&'a {{o["rs_own"]}}<{{o["tp"]}}>> for Map_{{i}}<'a> {
    fn from(v: &'a {{o["rs_own"]}}<{{o["tp"]}}>) -> Self {
        Self::new(v.coords.as_slice())
    }
}

impl<'a> From<Map_{{i}}<'a>> for {{o["rs_own"]}}<{{o["tp"]}}> {
    fn from(v: Map_{{i}}<'a>) -> Self {
        let d = unsafe {
            std::slice::from_raw_parts(v.data, 4)
        };
        Self::from_vector(na::Vector4::from_column_slice(d))
    }
}

impl<'a> From<&'a mut {{o["rs_own"]}}<{{o["tp"]}}>> for MapMut_{{i}}<'a> {
    fn from(v: &'a mut {{o["rs_own"]}}<{{o["tp"]}}>) -> Self {
        Self::new(v.coords.as_mut_slice())
    }
}

impl<'a> From<MapMut_{{i}}<'a>> for {{o["rs_own"]}}<{{o["tp"]}}> {
    fn from(v: MapMut_{{i}}<'a>) -> Self {
        let d = unsafe {
            std::slice::from_raw_parts(v.data, 4)
        };
        Self::from_vector(na::Vector4::from_column_slice(d))
    }
}

{% set i = "AngleAxisd" -%}
impl Mul for {{i}} {
    type Output = Quaterniond;

    fn mul(self, other: Self) -> Self::Output {
        ffi::{{i}}_mul(&self, &other)
    }
}

impl Mul for &{{i}} {
    type Output = Quaterniond;

    fn mul(self, other: Self) -> Self::Output {
        ffi::{{i}}_mul(self, other)
    }
}

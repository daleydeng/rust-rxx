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

{% for k, o in cffi_fixed_matrices.items() -%}
pub type {{k}} = Matrix<{{o.tp}}, {{o.shape[0]}}, {{o.shape[1]}}>;
{% endfor %}

{% for k, o in cffi_quats.items() -%}
#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct {{k}} {
    pub x: {{o.tp}},
    pub y: {{o.tp}},
    pub z: {{o.tp}},
    pub w: {{o.tp}},
}

impl FixedSize for {{k}} {
    type Scalar = {{o.tp}};
}
{% endfor %}

{% for k, o in cffi_aas.items() -%}
#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct {{k}} {
    pub axis: {{o.axis}},
    pub angle: {{o.tp}},
}
{%- endfor %}

{% for k, o in cffi_fns.items() -%}
{{ cffi_genrs_fn(k, type_mapping=type_mapping, **o) }}
{% endfor %}

{% for k, o in cffi_fixed_matrices.items() -%}
impl<'a> MapMut<'a, {{k}}> {
    pub fn new(data: &'a mut [{{o.tp}}]) -> Self {
        MapMut_{{k}}_new::<'a>(data.as_mut_ptr())
    }
}

impl<'a> Map<'a, {{k}}> {
    pub fn new(data: &'a [{{o.tp}}]) -> Self {
        Map_{{k}}_new::<'a>(data.as_ptr())
    }
}

{% if o.rs_own -%}
impl From<{{o.rs_own}}<{{o.tp}}>> for {{k}} {
    fn from(v: {{o.rs_own}}<{{o.tp}}>) -> Self {
        Self {
            data: v.as_slice().try_into().unwrap()
        }
    }
}

impl From<{{k}}> for {{o.rs_own}}<{{o.tp}}> {
    fn from(v: {{k}}) -> Self {
        let d = unsafe {
            std::mem::transmute::<[{{o.tp}}; {{o.size}}], [[{{o.tp}}; {{o.shape[0]}}]; {{o.shape[1]}}]>(v.data)
        };
        Self::from_data(na::ArrayStorage(d))
    }
}

{%- endif %}
{% if o.rs_ref -%}

impl<'a> From<Map<'a, {{k}}>> for {{o.rs_ref}}<'a, {{o.tp}}> {
    fn from(v: Map<'a, {{k}}>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorage::from_raw_parts(v.data, (Dim::from_usize({{o.shape[0]}}), Dim::from_usize({{o.shape[1]}})), (Dim::from_usize(1), Dim::from_usize({{o.shape[1]}})))};

        Self::from_data(s)
    }
}

impl<'a> From<{{o.rs_ref}}<'a, {{o.tp}}>> for Map<'a, {{k}}> {
    fn from(v: {{o.rs_ref}}<'a, {{o.tp}}>) -> Self {
        Self::new(v.data.into_slice())
    }
}
{%- endif %}
{% if o.rs_mut_ref -%}
impl<'a> From<MapMut<'a, {{k}}>> for {{o["rs_mut_ref"]}}<'a, {{o.tp}}> {
    fn from(v: MapMut<'a, {{k}}>) -> Self {
        // stride BUG strides: (CStride, RStride)
        let s = unsafe {na::SliceStorageMut::from_raw_parts(v.data, (Dim::from_usize({{o.shape[0]}}), Dim::from_usize({{o.shape[1]}})), (Dim::from_usize(1), Dim::from_usize({{o.shape[1]}})))};

        Self::from_data(s)
    }
}

impl<'a> From<{{o.rs_mut_ref}}<'a, {{o.tp}}>> for MapMut<'a, {{k}}> {
    fn from(v: {{o.rs_mut_ref}}<'a, {{o.tp}}>) -> Self {
        Self::new(v.data.into_slice_mut())
    }
}
{%- endif %}

{%- endfor %}

{% for k, o in cffi_quats.items() -%}

impl<'a> MapMut<'a, {{k}}> {
    pub fn new(data: &'a mut [{{o.tp}}]) -> Self {
        MapMut_{{k}}_new::<'a>(data.as_mut().as_mut_ptr())
    }
}

impl<'a> Map<'a, {{k}}> {
    pub fn new(data: &'a [{{o.tp}}]) -> Self {
        Map_{{k}}_new::<'a>(data.as_ptr())
    }
}

impl Mul for {{k}} {
    type Output = {{k}};

    fn mul(self, other: Self) -> Self::Output {
	{{k}}::mul(&self, &other)
    }
}

impl Mul for &{{k}} {
    type Output = {{k}};

    fn mul(self, other: Self) -> Self::Output {
	{{k}}::mul(self, other)
    }
}

impl From<{{o.rs_own}}<{{o.tp}}>> for {{k}} {
    fn from(v: {{o.rs_own}}<{{o.tp}}>) -> Self {
        Self {
            x: v.coords.x,
            y: v.coords.y,
            z: v.coords.z,
            w: v.coords.w,
        }
    }
}

impl From<{{k}}> for {{o.rs_own}}<{{o.tp}}> {
    fn from(v: {{k}}) -> Self {
        Self::new(v.w, v.x, v.y, v.z)
    }
}

impl<'a> From<&'a {{o.rs_own}}<{{o.tp}}>> for Map<'a, {{k}}> {
    fn from(v: &'a {{o.rs_own}}<{{o.tp}}>) -> Self {
        Self::new(v.coords.as_slice())
    }
}

impl<'a> From<Map<'a, {{k}}>> for {{o.rs_own}}<{{o.tp}}> {
    fn from(v: Map<'a, {{k}}>) -> Self {
        let d = unsafe {
            std::slice::from_raw_parts(v.data, 4)
        };
        Self::from_vector(na::Vector4::from_column_slice(d))
    }
}

impl<'a> From<&'a mut {{o.rs_own}}<{{o.tp}}>> for MapMut<'a, {{k}}> {
    fn from(v: &'a mut {{o.rs_own}}<{{o.tp}}>) -> Self {
        Self::new(v.coords.as_mut_slice())
    }
}

impl<'a> From<MapMut<'a, {{k}}>> for {{o.rs_own}}<{{o.tp}}> {
    fn from(v: MapMut<'a, {{k}}>) -> Self {
        let d = unsafe {
            std::slice::from_raw_parts(v.data, 4)
        };
        Self::from_vector(na::Vector4::from_column_slice(d))
    }
}

{% endfor %}

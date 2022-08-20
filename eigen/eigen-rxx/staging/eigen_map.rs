{% macro struct_eigen_map_matx(cls, tp) -%}

#[repr(C)]
#[derive(Debug)]
pub struct MapMut_{{cls}}<'a> {
    pub data: *mut {{tp}},
    pub rows: isize,
    pub cols: isize,
    _pd: std::marker::PhantomData<&'a ()>,
}

#[repr(C)]
#[derive(Debug)]
pub struct Map_{{cls}}<'a> {
    pub data: *const {{tp}},
    pub rows: isize,
    pub cols: isize,
    _pd: std::marker::PhantomData<&'a ()>,
}

#[repr(C)]
#[derive(Debug)]
pub struct MapMut_{{cls}}_stride<'a> {
    pub data: *mut {{tp}},
    pub rows: isize,
    pub cols: isize,
    pub s0: isize,
    pub s1: isize,
    _pd: std::marker::PhantomData<&'a ()>,
}

#[repr(C)]
#[derive(Debug)]
pub struct Map_{{cls}}_stride<'a> {
    pub data: *const {{tp}},
    pub rows: isize,
    pub cols: isize,
    pub s0: isize,
    pub s1: isize,
    _pd: std::marker::PhantomData<&'a ()>,
}

{%- endmacro %}

{% macro struct_eigen_map_vecx(cls, tp) -%}

#[repr(C)]
#[derive(Debug)]
pub struct MapMut_{{cls}}<'a> {
    pub data: *mut {{tp}},
    pub size: isize,
    _pd: std::marker::PhantomData<&'a ()>,
}

#[repr(C)]
#[derive(Debug)]
pub struct Map_{{cls}}<'a> {
    pub data: *const {{tp}},
    pub size: isize,
    _pd: std::marker::PhantomData<&'a ()>,
}

#[repr(C)]
#[derive(Debug)]
pub struct MapMut_{{cls}}_stride<'a> {
    pub data: *mut {{tp}},
    pub size: isize,
    pub stride: isize,
    _pd: std::marker::PhantomData<&'a ()>,
}

#[repr(C)]
#[derive(Debug)]
pub struct Map_{{cls}}_stride<'a> {
    pub data: *const {{tp}},
    pub size: isize,
    pub stride: isize,
    _pd: std::marker::PhantomData<&'a ()>,
}

{%- endmacro %}

{% macro ffi_eigen_map_fixed(cls, tp) -%}
#[rust_name = "MapMut_{{cls}}_new"]
unsafe fn MapMut_fixed_new<'a>(data: *mut {{tp}}) -> MapMut_{{cls}}<'a>;
#[rust_name = "Map_{{cls}}_new"]
unsafe fn Map_fixed_new<'a>(data: *const {{tp}}) -> Map_{{cls}}<'a>;
{%- endmacro %}

{% macro ffi_eigen_map_matx(cls, tp) -%}

type MapMut_{{cls}}<'a> = super::MapMut_{{cls}}<'a>;
type Map_{{cls}}<'a> = super::Map_{{cls}}<'a>;
type MapMut_{{cls}}_stride<'a> = super::MapMut_{{cls}}_stride<'a>;
type Map_{{cls}}_stride<'a> = super::Map_{{cls}}_stride<'a>;
unsafe fn MapMut_{{cls}}_new<'a>(data: *mut {{tp}}, rows: usize, cols: usize) -> MapMut_{{cls}}<'a>;
unsafe fn Map_{{cls}}_new<'a>(data: *const {{tp}}, rows: usize, cols: usize) -> Map_{{cls}}<'a>;
unsafe fn MapMut_{{cls}}_stride_new<'a>(data: *mut {{tp}}, rows: usize, cols: usize, s0: usize, s1: usize) -> MapMut_{{cls}}_stride<'a>;
unsafe fn Map_{{cls}}_stride_new<'a>(data: *const {{tp}}, rows: usize, cols: usize, s0: usize, s1: usize) -> Map_{{cls}}_stride<'a>;

{%- endmacro %}

{% macro ffi_eigen_map_vecx(cls, tp) -%}

type MapMut_{{cls}}<'a> = super::MapMut_{{cls}}<'a>;
type Map_{{cls}}<'a> = super::Map_{{cls}}<'a>;
type MapMut_{{cls}}_stride<'a> = super::MapMut_{{cls}}_stride<'a>;
type Map_{{cls}}_stride<'a> = super::Map_{{cls}}_stride<'a>;
unsafe fn MapMut_{{cls}}_new<'a>(data: *mut {{tp}}, size: usize) -> MapMut_{{cls}}<'a>;
unsafe fn Map_{{cls}}_new<'a>(data: *const {{tp}}, size: usize) -> Map_{{cls}}<'a>;

unsafe fn MapMut_{{cls}}_stride_new<'a>(data: *mut {{tp}}, size: usize, s: usize) -> MapMut_{{cls}}_stride<'a>;
unsafe fn Map_{{cls}}_stride_new<'a>(data: *const {{tp}}, size: usize, s: usize) -> Map_{{cls}}_stride<'a>;

{%- endmacro %}

{% macro impl_eigen_map_fixed(cls, tp) -%}

impl<'a> MapMut_{{cls}}<'a> {
    pub fn new(data: &'a mut [{{tp}}]) -> Self {
        unsafe {
            ffi::MapMut_{{cls}}_new::<'a>(data.as_mut_ptr())
        }
    }
}

impl<'a> Map_{{cls}}<'a> {
    pub fn new(data: &'a [{{tp}}]) -> Self {
        unsafe {
            ffi::Map_{{cls}}_new::<'a>(data.as_ptr())
        }
    }
}

{%- endmacro %}

{% macro impl_eigen_map_matx(cls, tp) -%}

impl<'a> MapMut_{{cls}}<'a> {
    pub fn new(data: &'a mut [{{tp}}], rows: usize, cols: usize) -> Self {
        unsafe {
            ffi::MapMut_{{cls}}_new::<'a>(data.as_mut_ptr(), rows, cols)
        }
    }
}

impl<'a> Map_{{cls}}<'a> {
    pub fn new(data: &'a [{{tp}}], rows: usize, cols: usize) -> Self {
        unsafe {
            ffi::Map_{{cls}}_new::<'a>(data.as_ptr(), rows, cols)
        }
    }
}

impl<'a> MapMut_{{cls}}_stride<'a> {
    pub fn new(data: &'a mut [{{tp}}], rows: usize, cols: usize, s0: usize, s1: usize) -> Self {
        unsafe {
            ffi::MapMut_{{cls}}_stride_new::<'a>(data.as_mut_ptr(), rows, cols, s0, s1)
        }
    }
}

impl<'a> Map_{{cls}}_stride<'a> {
    pub fn new(data: &'a [{{tp}}], rows: usize, cols: usize, s0: usize, s1: usize) -> Self {
        unsafe {
            ffi::Map_{{cls}}_stride_new::<'a>(data.as_ptr(), rows, cols, s0, s1)
        }
    }
}

{%- endmacro %}

{% macro impl_eigen_map_vecx(cls, tp) -%}

impl<'a> MapMut_{{cls}}<'a> {
    pub fn new(data: &'a mut [{{tp}}], size: usize) -> Self {
        unsafe {
            ffi::MapMut_{{cls}}_new::<'a>(data.as_mut_ptr(), size)
        }
    }
}

impl<'a> Map_{{cls}}<'a> {
    pub fn new(data: &'a [{{tp}}], size: usize) -> Self {
        unsafe {
            ffi::Map_{{cls}}_new::<'a>(data.as_ptr(), size)
        }
    }
}

impl<'a> MapMut_{{cls}}_stride<'a> {
    pub fn new(data: &'a mut [{{tp}}], size: usize, s: usize) -> Self {
        unsafe {
            ffi::MapMut_{{cls}}_stride_new::<'a>(data.as_mut_ptr(), size, s)
        }
    }
}

impl<'a> Map_{{cls}}_stride<'a> {
    pub fn new(data: &'a [{{tp}}], size: usize, s: usize) -> Self {
        unsafe {
            ffi::Map_{{cls}}_stride_new::<'a>(data.as_ptr(), size, s)
        }
    }
}

{%- endmacro %}

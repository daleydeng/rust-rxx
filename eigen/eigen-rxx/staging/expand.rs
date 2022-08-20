#![feature(prelude_import)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;

mod ffi {












    extern crate nalgebra as na;
    use na::Dim;
    use std::ops::Mul;
    pub trait MatrixTrait {
        type Scalar;
    }
    #[repr(C, align(16))]
    pub struct Matrix<SCALAR, const ROWS : usize, const COLS : usize> where
        [(); ROWS * COLS] {
        pub data: [SCALAR; ROWS * COLS],
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<SCALAR: ::core::fmt::Debug, const ROWS : usize, const COLS : usize>
        ::core::fmt::Debug for Matrix<SCALAR, ROWS, COLS> where
        [(); ROWS * COLS] {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Self { data: ref __self_0_0 } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "Matrix");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "data",
                            &&(*__self_0_0));
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<SCALAR: ::core::marker::Copy, const ROWS : usize, const COLS : usize>
        ::core::marker::Copy for Matrix<SCALAR, ROWS, COLS> where
        [(); ROWS * COLS] {
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<SCALAR: ::core::clone::Clone, const ROWS : usize, const COLS : usize>
        ::core::clone::Clone for Matrix<SCALAR, ROWS, COLS> where
        [(); ROWS * COLS] {
        #[inline]
        fn clone(&self) -> Matrix<SCALAR, ROWS, COLS> {
            match *self {
                Self { data: ref __self_0_0 } =>
                    Matrix {
                        data: ::core::clone::Clone::clone(&(*__self_0_0)),
                    },
            }
        }
    }
    impl<SCALAR, const ROWS : usize, const COLS : usize> MatrixTrait for
        Matrix<SCALAR, ROWS, COLS> where [(); ROWS * COLS] {
        type Scalar = SCALAR;
    }
    #[repr(C)]
    pub struct Map<'a, T: MatrixTrait> {
        pub data: *const T::Scalar,
        _pd: std::marker::PhantomData<&'a ()>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a, T: ::core::fmt::Debug + MatrixTrait> ::core::fmt::Debug for
        Map<'a, T> where T::Scalar: ::core::fmt::Debug {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Self { data: ref __self_0_0, _pd: ref __self_0_1 } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "Map");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "data",
                            &&(*__self_0_0));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "_pd",
                            &&(*__self_0_1));
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[repr(C)]
    pub struct MapMut<'a, T: MatrixTrait> {
        pub data: *mut T::Scalar,
        _pd: std::marker::PhantomData<&'a ()>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a, T: ::core::fmt::Debug + MatrixTrait> ::core::fmt::Debug for
        MapMut<'a, T> where T::Scalar: ::core::fmt::Debug {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Self { data: ref __self_0_0, _pd: ref __self_0_1 } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "MapMut");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "data",
                            &&(*__self_0_0));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "_pd",
                            &&(*__self_0_1));
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    pub type Matrix2d = Matrix<f64, 2, 2>;
    unsafe impl cxx::ExternType for Matrix2d {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a, ::cxx::t,
            ::cxx::r, ::cxx::i, ::cxx::x, ::cxx::_2, ::cxx::d);
        type Kind = cxx::kind::Trivial;
    }
    #[allow(non_camel_case_types)]
    pub type Map_Matrix2d<'a> = Map<'a, Matrix2d>;
    unsafe impl cxx::ExternType for Map_Matrix2d<'_> {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a, ::cxx::p,
            ::cxx::__, ::cxx::M, ::cxx::a, ::cxx::t, ::cxx::r, ::cxx::i,
            ::cxx::x, ::cxx::_2, ::cxx::d);
        type Kind = cxx::kind::Trivial;
    }
    #[allow(non_camel_case_types)]
    pub type MapMut_Matrix2d<'a> = MapMut<'a, Matrix2d>;
    unsafe impl cxx::ExternType for MapMut_Matrix2d<'_> {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a, ::cxx::p,
            ::cxx::M, ::cxx::u, ::cxx::t, ::cxx::__, ::cxx::M, ::cxx::a,
            ::cxx::t, ::cxx::r, ::cxx::i, ::cxx::x, ::cxx::_2, ::cxx::d);
        type Kind = cxx::kind::Trivial;
    }
    pub type Matrix2i = Matrix<i32, 2, 2>;
    unsafe impl cxx::ExternType for Matrix2i {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a, ::cxx::t,
            ::cxx::r, ::cxx::i, ::cxx::x, ::cxx::_2, ::cxx::i);
        type Kind = cxx::kind::Trivial;
    }
    #[allow(non_camel_case_types)]
    pub type Map_Matrix2i<'a> = Map<'a, Matrix2i>;
    unsafe impl cxx::ExternType for Map_Matrix2i<'_> {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a, ::cxx::p,
            ::cxx::__, ::cxx::M, ::cxx::a, ::cxx::t, ::cxx::r, ::cxx::i,
            ::cxx::x, ::cxx::_2, ::cxx::i);
        type Kind = cxx::kind::Trivial;
    }
    #[allow(non_camel_case_types)]
    pub type MapMut_Matrix2i<'a> = MapMut<'a, Matrix2i>;
    unsafe impl cxx::ExternType for MapMut_Matrix2i<'_> {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a, ::cxx::p,
            ::cxx::M, ::cxx::u, ::cxx::t, ::cxx::__, ::cxx::M, ::cxx::a,
            ::cxx::t, ::cxx::r, ::cxx::i, ::cxx::x, ::cxx::_2, ::cxx::i);
        type Kind = cxx::kind::Trivial;
    }
    pub type Matrix3d = Matrix<f64, 3, 3>;
    unsafe impl cxx::ExternType for Matrix3d {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a, ::cxx::t,
            ::cxx::r, ::cxx::i, ::cxx::x, ::cxx::_3, ::cxx::d);
        type Kind = cxx::kind::Trivial;
    }
    #[allow(non_camel_case_types)]
    pub type Map_Matrix3d<'a> = Map<'a, Matrix3d>;
    unsafe impl cxx::ExternType for Map_Matrix3d<'_> {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a, ::cxx::p,
            ::cxx::__, ::cxx::M, ::cxx::a, ::cxx::t, ::cxx::r, ::cxx::i,
            ::cxx::x, ::cxx::_3, ::cxx::d);
        type Kind = cxx::kind::Trivial;
    }
    #[allow(non_camel_case_types)]
    pub type MapMut_Matrix3d<'a> = MapMut<'a, Matrix3d>;
    unsafe impl cxx::ExternType for MapMut_Matrix3d<'_> {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a, ::cxx::p,
            ::cxx::M, ::cxx::u, ::cxx::t, ::cxx::__, ::cxx::M, ::cxx::a,
            ::cxx::t, ::cxx::r, ::cxx::i, ::cxx::x, ::cxx::_3, ::cxx::d);
        type Kind = cxx::kind::Trivial;
    }
    pub type Matrix4d = Matrix<f64, 4, 4>;
    unsafe impl cxx::ExternType for Matrix4d {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a, ::cxx::t,
            ::cxx::r, ::cxx::i, ::cxx::x, ::cxx::_4, ::cxx::d);
        type Kind = cxx::kind::Trivial;
    }
    #[allow(non_camel_case_types)]
    pub type Map_Matrix4d<'a> = Map<'a, Matrix4d>;
    unsafe impl cxx::ExternType for Map_Matrix4d<'_> {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a, ::cxx::p,
            ::cxx::__, ::cxx::M, ::cxx::a, ::cxx::t, ::cxx::r, ::cxx::i,
            ::cxx::x, ::cxx::_4, ::cxx::d);
        type Kind = cxx::kind::Trivial;
    }
    #[allow(non_camel_case_types)]
    pub type MapMut_Matrix4d<'a> = MapMut<'a, Matrix4d>;
    unsafe impl cxx::ExternType for MapMut_Matrix4d<'_> {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a, ::cxx::p,
            ::cxx::M, ::cxx::u, ::cxx::t, ::cxx::__, ::cxx::M, ::cxx::a,
            ::cxx::t, ::cxx::r, ::cxx::i, ::cxx::x, ::cxx::_4, ::cxx::d);
        type Kind = cxx::kind::Trivial;
    }
    pub type Matrix2x3d = Matrix<f64, 2, 3>;
    unsafe impl cxx::ExternType for Matrix2x3d {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a, ::cxx::t,
            ::cxx::r, ::cxx::i, ::cxx::x, ::cxx::_2, ::cxx::x, ::cxx::_3,
            ::cxx::d);
        type Kind = cxx::kind::Trivial;
    }
    #[allow(non_camel_case_types)]
    pub type Map_Matrix2x3d<'a> = Map<'a, Matrix2x3d>;
    unsafe impl cxx::ExternType for Map_Matrix2x3d<'_> {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a, ::cxx::p,
            ::cxx::__, ::cxx::M, ::cxx::a, ::cxx::t, ::cxx::r, ::cxx::i,
            ::cxx::x, ::cxx::_2, ::cxx::x, ::cxx::_3, ::cxx::d);
        type Kind = cxx::kind::Trivial;
    }
    #[allow(non_camel_case_types)]
    pub type MapMut_Matrix2x3d<'a> = MapMut<'a, Matrix2x3d>;
    unsafe impl cxx::ExternType for MapMut_Matrix2x3d<'_> {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a, ::cxx::p,
            ::cxx::M, ::cxx::u, ::cxx::t, ::cxx::__, ::cxx::M, ::cxx::a,
            ::cxx::t, ::cxx::r, ::cxx::i, ::cxx::x, ::cxx::_2, ::cxx::x,
            ::cxx::_3, ::cxx::d);
        type Kind = cxx::kind::Trivial;
    }
    #[repr(C)]
    pub struct MatrixXd {
        pub data: *mut f64,
        pub rows: isize,
        pub cols: isize,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for MatrixXd {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Self {
                    data: ref __self_0_0,
                    rows: ref __self_0_1,
                    cols: ref __self_0_2 } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "MatrixXd");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "data",
                            &&(*__self_0_0));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "rows",
                            &&(*__self_0_1));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "cols",
                            &&(*__self_0_2));
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    unsafe impl cxx::ExternType for MatrixXd {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a, ::cxx::t,
            ::cxx::r, ::cxx::i, ::cxx::x, ::cxx::X, ::cxx::d);
        type Kind = cxx::kind::Trivial;
    }
    #[allow(non_camel_case_types)]
    #[repr(C)]
    pub struct MapMut_MatrixXd<'a> {
        pub data: *mut f64,
        pub rows: isize,
        pub cols: isize,
        _pd: std::marker::PhantomData<&'a ()>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    #[allow(non_camel_case_types)]
    impl<'a> ::core::fmt::Debug for MapMut_MatrixXd<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Self {
                    data: ref __self_0_0,
                    rows: ref __self_0_1,
                    cols: ref __self_0_2,
                    _pd: ref __self_0_3 } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f,
                                "MapMut_MatrixXd");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "data",
                            &&(*__self_0_0));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "rows",
                            &&(*__self_0_1));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "cols",
                            &&(*__self_0_2));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "_pd",
                            &&(*__self_0_3));
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    unsafe impl cxx::ExternType for MapMut_MatrixXd<'_> {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a, ::cxx::p,
            ::cxx::M, ::cxx::u, ::cxx::t, ::cxx::__, ::cxx::M, ::cxx::a,
            ::cxx::t, ::cxx::r, ::cxx::i, ::cxx::x, ::cxx::X, ::cxx::d);
        type Kind = cxx::kind::Trivial;
    }
    #[allow(non_camel_case_types)]
    #[repr(C)]
    pub struct Map_MatrixXd<'a> {
        pub data: *const f64,
        pub rows: isize,
        pub cols: isize,
        _pd: std::marker::PhantomData<&'a ()>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    #[allow(non_camel_case_types)]
    impl<'a> ::core::fmt::Debug for Map_MatrixXd<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Self {
                    data: ref __self_0_0,
                    rows: ref __self_0_1,
                    cols: ref __self_0_2,
                    _pd: ref __self_0_3 } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f,
                                "Map_MatrixXd");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "data",
                            &&(*__self_0_0));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "rows",
                            &&(*__self_0_1));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "cols",
                            &&(*__self_0_2));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "_pd",
                            &&(*__self_0_3));
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    unsafe impl cxx::ExternType for Map_MatrixXd<'_> {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a, ::cxx::p,
            ::cxx::__, ::cxx::M, ::cxx::a, ::cxx::t, ::cxx::r, ::cxx::i,
            ::cxx::x, ::cxx::X, ::cxx::d);
        type Kind = cxx::kind::Trivial;
    }
    #[allow(non_camel_case_types)]
    #[repr(C)]
    pub struct MapMut_MatrixXd_stride<'a> {
        pub data: *mut f64,
        pub rows: isize,
        pub cols: isize,
        pub s0: isize,
        pub s1: isize,
        _pd: std::marker::PhantomData<&'a ()>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    #[allow(non_camel_case_types)]
    impl<'a> ::core::fmt::Debug for MapMut_MatrixXd_stride<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Self {
                    data: ref __self_0_0,
                    rows: ref __self_0_1,
                    cols: ref __self_0_2,
                    s0: ref __self_0_3,
                    s1: ref __self_0_4,
                    _pd: ref __self_0_5 } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f,
                                "MapMut_MatrixXd_stride");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "data",
                            &&(*__self_0_0));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "rows",
                            &&(*__self_0_1));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "cols",
                            &&(*__self_0_2));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "s0",
                            &&(*__self_0_3));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "s1",
                            &&(*__self_0_4));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "_pd",
                            &&(*__self_0_5));
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    unsafe impl cxx::ExternType for MapMut_MatrixXd_stride<'_> {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a, ::cxx::p,
            ::cxx::M, ::cxx::u, ::cxx::t, ::cxx::__, ::cxx::M, ::cxx::a,
            ::cxx::t, ::cxx::r, ::cxx::i, ::cxx::x, ::cxx::X, ::cxx::d,
            ::cxx::__, ::cxx::s, ::cxx::t, ::cxx::r, ::cxx::i, ::cxx::d,
            ::cxx::e);
        type Kind = cxx::kind::Trivial;
    }
    #[allow(non_camel_case_types)]
    #[repr(C)]
    pub struct Map_MatrixXd_stride<'a> {
        pub data: *const f64,
        pub rows: isize,
        pub cols: isize,
        pub s0: isize,
        pub s1: isize,
        _pd: std::marker::PhantomData<&'a ()>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    #[allow(non_camel_case_types)]
    impl<'a> ::core::fmt::Debug for Map_MatrixXd_stride<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Self {
                    data: ref __self_0_0,
                    rows: ref __self_0_1,
                    cols: ref __self_0_2,
                    s0: ref __self_0_3,
                    s1: ref __self_0_4,
                    _pd: ref __self_0_5 } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f,
                                "Map_MatrixXd_stride");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "data",
                            &&(*__self_0_0));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "rows",
                            &&(*__self_0_1));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "cols",
                            &&(*__self_0_2));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "s0",
                            &&(*__self_0_3));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "s1",
                            &&(*__self_0_4));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "_pd",
                            &&(*__self_0_5));
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    unsafe impl cxx::ExternType for Map_MatrixXd_stride<'_> {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a, ::cxx::p,
            ::cxx::__, ::cxx::M, ::cxx::a, ::cxx::t, ::cxx::r, ::cxx::i,
            ::cxx::x, ::cxx::X, ::cxx::d, ::cxx::__, ::cxx::s, ::cxx::t,
            ::cxx::r, ::cxx::i, ::cxx::d, ::cxx::e);
        type Kind = cxx::kind::Trivial;
    }
    pub type Vector2d = Matrix<f64, 2, 1>;
    unsafe impl cxx::ExternType for Vector2d {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::V, ::cxx::e, ::cxx::c,
            ::cxx::t, ::cxx::o, ::cxx::r, ::cxx::_2, ::cxx::d);
        type Kind = cxx::kind::Trivial;
    }
    #[allow(non_camel_case_types)]
    pub type Map_Vector2d<'a> = Map<'a, Vector2d>;
    unsafe impl cxx::ExternType for Map_Vector2d<'_> {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a, ::cxx::p,
            ::cxx::__, ::cxx::V, ::cxx::e, ::cxx::c, ::cxx::t, ::cxx::o,
            ::cxx::r, ::cxx::_2, ::cxx::d);
        type Kind = cxx::kind::Trivial;
    }
    #[allow(non_camel_case_types)]
    pub type MapMut_Vector2d<'a> = MapMut<'a, Vector2d>;
    unsafe impl cxx::ExternType for MapMut_Vector2d<'_> {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a, ::cxx::p,
            ::cxx::M, ::cxx::u, ::cxx::t, ::cxx::__, ::cxx::V, ::cxx::e,
            ::cxx::c, ::cxx::t, ::cxx::o, ::cxx::r, ::cxx::_2, ::cxx::d);
        type Kind = cxx::kind::Trivial;
    }
    pub type Vector3d = Matrix<f64, 3, 1>;
    unsafe impl cxx::ExternType for Vector3d {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::V, ::cxx::e, ::cxx::c,
            ::cxx::t, ::cxx::o, ::cxx::r, ::cxx::_3, ::cxx::d);
        type Kind = cxx::kind::Trivial;
    }
    #[allow(non_camel_case_types)]
    pub type Map_Vector3d<'a> = Map<'a, Vector3d>;
    unsafe impl cxx::ExternType for Map_Vector3d<'_> {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a, ::cxx::p,
            ::cxx::__, ::cxx::V, ::cxx::e, ::cxx::c, ::cxx::t, ::cxx::o,
            ::cxx::r, ::cxx::_3, ::cxx::d);
        type Kind = cxx::kind::Trivial;
    }
    #[allow(non_camel_case_types)]
    pub type MapMut_Vector3d<'a> = MapMut<'a, Vector3d>;
    unsafe impl cxx::ExternType for MapMut_Vector3d<'_> {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a, ::cxx::p,
            ::cxx::M, ::cxx::u, ::cxx::t, ::cxx::__, ::cxx::V, ::cxx::e,
            ::cxx::c, ::cxx::t, ::cxx::o, ::cxx::r, ::cxx::_3, ::cxx::d);
        type Kind = cxx::kind::Trivial;
    }
    pub type Vector4d = Matrix<f64, 4, 1>;
    unsafe impl cxx::ExternType for Vector4d {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::V, ::cxx::e, ::cxx::c,
            ::cxx::t, ::cxx::o, ::cxx::r, ::cxx::_4, ::cxx::d);
        type Kind = cxx::kind::Trivial;
    }
    #[allow(non_camel_case_types)]
    pub type Map_Vector4d<'a> = Map<'a, Vector4d>;
    unsafe impl cxx::ExternType for Map_Vector4d<'_> {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a, ::cxx::p,
            ::cxx::__, ::cxx::V, ::cxx::e, ::cxx::c, ::cxx::t, ::cxx::o,
            ::cxx::r, ::cxx::_4, ::cxx::d);
        type Kind = cxx::kind::Trivial;
    }
    #[allow(non_camel_case_types)]
    pub type MapMut_Vector4d<'a> = MapMut<'a, Vector4d>;
    unsafe impl cxx::ExternType for MapMut_Vector4d<'_> {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a, ::cxx::p,
            ::cxx::M, ::cxx::u, ::cxx::t, ::cxx::__, ::cxx::V, ::cxx::e,
            ::cxx::c, ::cxx::t, ::cxx::o, ::cxx::r, ::cxx::_4, ::cxx::d);
        type Kind = cxx::kind::Trivial;
    }
    #[repr(C)]
    pub struct VectorXd {
        pub data: *mut f64,
        pub size: isize,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for VectorXd {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Self { data: ref __self_0_0, size: ref __self_0_1 } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "VectorXd");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "data",
                            &&(*__self_0_0));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "size",
                            &&(*__self_0_1));
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    unsafe impl cxx::ExternType for VectorXd {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::V, ::cxx::e, ::cxx::c,
            ::cxx::t, ::cxx::o, ::cxx::r, ::cxx::X, ::cxx::d);
        type Kind = cxx::kind::Trivial;
    }
    #[allow(non_camel_case_types)]
    #[repr(C)]
    pub struct MapMut_VectorXd<'a> {
        pub data: *mut f64,
        pub size: isize,
        _pd: std::marker::PhantomData<&'a ()>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    #[allow(non_camel_case_types)]
    impl<'a> ::core::fmt::Debug for MapMut_VectorXd<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Self {
                    data: ref __self_0_0,
                    size: ref __self_0_1,
                    _pd: ref __self_0_2 } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f,
                                "MapMut_VectorXd");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "data",
                            &&(*__self_0_0));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "size",
                            &&(*__self_0_1));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "_pd",
                            &&(*__self_0_2));
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    unsafe impl cxx::ExternType for MapMut_VectorXd<'_> {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a, ::cxx::p,
            ::cxx::M, ::cxx::u, ::cxx::t, ::cxx::__, ::cxx::V, ::cxx::e,
            ::cxx::c, ::cxx::t, ::cxx::o, ::cxx::r, ::cxx::X, ::cxx::d);
        type Kind = cxx::kind::Trivial;
    }
    #[allow(non_camel_case_types)]
    #[repr(C)]
    pub struct Map_VectorXd<'a> {
        pub data: *const f64,
        pub size: isize,
        _pd: std::marker::PhantomData<&'a ()>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    #[allow(non_camel_case_types)]
    impl<'a> ::core::fmt::Debug for Map_VectorXd<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Self {
                    data: ref __self_0_0,
                    size: ref __self_0_1,
                    _pd: ref __self_0_2 } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f,
                                "Map_VectorXd");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "data",
                            &&(*__self_0_0));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "size",
                            &&(*__self_0_1));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "_pd",
                            &&(*__self_0_2));
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    unsafe impl cxx::ExternType for Map_VectorXd<'_> {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a, ::cxx::p,
            ::cxx::__, ::cxx::V, ::cxx::e, ::cxx::c, ::cxx::t, ::cxx::o,
            ::cxx::r, ::cxx::X, ::cxx::d);
        type Kind = cxx::kind::Trivial;
    }
    #[allow(non_camel_case_types)]
    #[repr(C)]
    pub struct MapMut_VectorXd_stride<'a> {
        pub data: *mut f64,
        pub size: isize,
        pub stride: isize,
        _pd: std::marker::PhantomData<&'a ()>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    #[allow(non_camel_case_types)]
    impl<'a> ::core::fmt::Debug for MapMut_VectorXd_stride<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Self {
                    data: ref __self_0_0,
                    size: ref __self_0_1,
                    stride: ref __self_0_2,
                    _pd: ref __self_0_3 } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f,
                                "MapMut_VectorXd_stride");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "data",
                            &&(*__self_0_0));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "size",
                            &&(*__self_0_1));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder,
                            "stride", &&(*__self_0_2));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "_pd",
                            &&(*__self_0_3));
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    unsafe impl cxx::ExternType for MapMut_VectorXd_stride<'_> {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a, ::cxx::p,
            ::cxx::M, ::cxx::u, ::cxx::t, ::cxx::__, ::cxx::V, ::cxx::e,
            ::cxx::c, ::cxx::t, ::cxx::o, ::cxx::r, ::cxx::X, ::cxx::d,
            ::cxx::__, ::cxx::s, ::cxx::t, ::cxx::r, ::cxx::i, ::cxx::d,
            ::cxx::e);
        type Kind = cxx::kind::Trivial;
    }
    #[allow(non_camel_case_types)]
    #[repr(C)]
    pub struct Map_VectorXd_stride<'a> {
        pub data: *const f64,
        pub size: isize,
        pub stride: isize,
        _pd: std::marker::PhantomData<&'a ()>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    #[allow(non_camel_case_types)]
    impl<'a> ::core::fmt::Debug for Map_VectorXd_stride<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Self {
                    data: ref __self_0_0,
                    size: ref __self_0_1,
                    stride: ref __self_0_2,
                    _pd: ref __self_0_3 } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f,
                                "Map_VectorXd_stride");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "data",
                            &&(*__self_0_0));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "size",
                            &&(*__self_0_1));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder,
                            "stride", &&(*__self_0_2));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "_pd",
                            &&(*__self_0_3));
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    unsafe impl cxx::ExternType for Map_VectorXd_stride<'_> {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a, ::cxx::p,
            ::cxx::__, ::cxx::V, ::cxx::e, ::cxx::c, ::cxx::t, ::cxx::o,
            ::cxx::r, ::cxx::X, ::cxx::d, ::cxx::__, ::cxx::s, ::cxx::t,
            ::cxx::r, ::cxx::i, ::cxx::d, ::cxx::e);
        type Kind = cxx::kind::Trivial;
    }
    #[repr(C, align(16))]
    pub struct Quaterniond {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub w: f64,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Quaterniond {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Self {
                    x: ref __self_0_0,
                    y: ref __self_0_1,
                    z: ref __self_0_2,
                    w: ref __self_0_3 } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "Quaterniond");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "x",
                            &&(*__self_0_0));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "y",
                            &&(*__self_0_1));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "z",
                            &&(*__self_0_2));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "w",
                            &&(*__self_0_3));
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for Quaterniond { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Quaterniond {
        #[inline]
        fn clone(&self) -> Quaterniond {
            {
                let _: ::core::clone::AssertParamIsClone<f64>;
                let _: ::core::clone::AssertParamIsClone<f64>;
                let _: ::core::clone::AssertParamIsClone<f64>;
                let _: ::core::clone::AssertParamIsClone<f64>;
                *self
            }
        }
    }
    unsafe impl cxx::ExternType for Quaterniond {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::Q, ::cxx::u, ::cxx::a,
            ::cxx::t, ::cxx::e, ::cxx::r, ::cxx::n, ::cxx::i, ::cxx::o,
            ::cxx::n, ::cxx::d);
        type Kind = cxx::kind::Trivial;
    }
    #[allow(non_camel_case_types)]
    #[repr(C)]
    pub struct MapMut_Quaterniond<'a> {
        pub data: *mut f64,
        _pd: std::marker::PhantomData<&'a ()>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    #[allow(non_camel_case_types)]
    impl<'a> ::core::fmt::Debug for MapMut_Quaterniond<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Self { data: ref __self_0_0, _pd: ref __self_0_1 } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f,
                                "MapMut_Quaterniond");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "data",
                            &&(*__self_0_0));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "_pd",
                            &&(*__self_0_1));
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    unsafe impl cxx::ExternType for MapMut_Quaterniond<'_> {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a, ::cxx::p,
            ::cxx::M, ::cxx::u, ::cxx::t, ::cxx::__, ::cxx::Q, ::cxx::u,
            ::cxx::a, ::cxx::t, ::cxx::e, ::cxx::r, ::cxx::n, ::cxx::i,
            ::cxx::o, ::cxx::n, ::cxx::d);
        type Kind = cxx::kind::Trivial;
    }
    #[allow(non_camel_case_types)]
    #[repr(C)]
    pub struct Map_Quaterniond<'a> {
        pub data: *const f64,
        _pd: std::marker::PhantomData<&'a ()>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    #[allow(non_camel_case_types)]
    impl<'a> ::core::fmt::Debug for Map_Quaterniond<'a> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Self { data: ref __self_0_0, _pd: ref __self_0_1 } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f,
                                "Map_Quaterniond");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "data",
                            &&(*__self_0_0));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "_pd",
                            &&(*__self_0_1));
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    unsafe impl cxx::ExternType for Map_Quaterniond<'_> {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a, ::cxx::p,
            ::cxx::__, ::cxx::Q, ::cxx::u, ::cxx::a, ::cxx::t, ::cxx::e,
            ::cxx::r, ::cxx::n, ::cxx::i, ::cxx::o, ::cxx::n, ::cxx::d);
        type Kind = cxx::kind::Trivial;
    }
    #[repr(C, align(16))]
    pub struct AngleAxisd {
        pub axis: Vector3d,
        pub angle: f64,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for AngleAxisd {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Self { axis: ref __self_0_0, angle: ref __self_0_1 } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "AngleAxisd");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "axis",
                            &&(*__self_0_0));
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder,
                            "angle", &&(*__self_0_1));
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for AngleAxisd { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for AngleAxisd {
        #[inline]
        fn clone(&self) -> AngleAxisd {
            {
                let _: ::core::clone::AssertParamIsClone<Vector3d>;
                let _: ::core::clone::AssertParamIsClone<f64>;
                *self
            }
        }
    }
    unsafe impl cxx::ExternType for AngleAxisd {
        type Id =
            (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::A, ::cxx::n, ::cxx::g,
            ::cxx::l, ::cxx::e, ::cxx::A, ::cxx::x, ::cxx::i, ::cxx::s,
            ::cxx::d);
        type Kind = cxx::kind::Trivial;
    }
    #[deny(improper_ctypes, improper_ctypes_definitions)]
    #[allow(clippy :: unknown_clippy_lints)]
    #[allow(non_camel_case_types, non_snake_case, clippy ::
    upper_case_acronyms)]
    mod ffi {
        pub type Matrix3d = super::Matrix3d;
        pub type MatrixXd = super::MatrixXd;
        pub type VectorXd = super::VectorXd;
        pub type MapMut_Matrix2d<'a> = super::MapMut_Matrix2d<'a>;
        pub type Map_Matrix2d<'a> = super::Map_Matrix2d<'a>;
        pub unsafe fn MapMut_Matrix2d_new<'a>(data: *mut f64)
            -> MapMut_Matrix2d<'a> {
            extern "C" {
                #[link_name = "rxx$cxxbridge1$MapMut_Matrix2d_new"]
                fn __MapMut_Matrix2d_new<'a>(data: *mut f64,
                __return: *mut MapMut_Matrix2d<'a>);
            }
            let mut __return =
                ::cxx::core::mem::MaybeUninit::<MapMut_Matrix2d<'a>>::uninit();
            __MapMut_Matrix2d_new(data, __return.as_mut_ptr());
            __return.assume_init()
        }
        pub unsafe fn Map_Matrix2d_new<'a>(data: *const f64)
            -> Map_Matrix2d<'a> {
            extern "C" {
                #[link_name = "rxx$cxxbridge1$Map_Matrix2d_new"]
                fn __Map_Matrix2d_new<'a>(data: *const f64,
                __return: *mut Map_Matrix2d<'a>);
            }
            let mut __return =
                ::cxx::core::mem::MaybeUninit::<Map_Matrix2d<'a>>::uninit();
            __Map_Matrix2d_new(data, __return.as_mut_ptr());
            __return.assume_init()
        }
        pub type MapMut_Matrix2i<'a> = super::MapMut_Matrix2i<'a>;
        pub type Map_Matrix2i<'a> = super::Map_Matrix2i<'a>;
        pub unsafe fn MapMut_Matrix2i_new<'a>(data: *mut i32)
            -> MapMut_Matrix2i<'a> {
            extern "C" {
                #[link_name = "rxx$cxxbridge1$MapMut_Matrix2i_new"]
                fn __MapMut_Matrix2i_new<'a>(data: *mut i32,
                __return: *mut MapMut_Matrix2i<'a>);
            }
            let mut __return =
                ::cxx::core::mem::MaybeUninit::<MapMut_Matrix2i<'a>>::uninit();
            __MapMut_Matrix2i_new(data, __return.as_mut_ptr());
            __return.assume_init()
        }
        pub unsafe fn Map_Matrix2i_new<'a>(data: *const i32)
            -> Map_Matrix2i<'a> {
            extern "C" {
                #[link_name = "rxx$cxxbridge1$Map_Matrix2i_new"]
                fn __Map_Matrix2i_new<'a>(data: *const i32,
                __return: *mut Map_Matrix2i<'a>);
            }
            let mut __return =
                ::cxx::core::mem::MaybeUninit::<Map_Matrix2i<'a>>::uninit();
            __Map_Matrix2i_new(data, __return.as_mut_ptr());
            __return.assume_init()
        }
        pub type MapMut_Matrix3d<'a> = super::MapMut_Matrix3d<'a>;
        pub type Map_Matrix3d<'a> = super::Map_Matrix3d<'a>;
        pub unsafe fn MapMut_Matrix3d_new<'a>(data: *mut f64)
            -> MapMut_Matrix3d<'a> {
            extern "C" {
                #[link_name = "rxx$cxxbridge1$MapMut_Matrix3d_new"]
                fn __MapMut_Matrix3d_new<'a>(data: *mut f64,
                __return: *mut MapMut_Matrix3d<'a>);
            }
            let mut __return =
                ::cxx::core::mem::MaybeUninit::<MapMut_Matrix3d<'a>>::uninit();
            __MapMut_Matrix3d_new(data, __return.as_mut_ptr());
            __return.assume_init()
        }
        pub unsafe fn Map_Matrix3d_new<'a>(data: *const f64)
            -> Map_Matrix3d<'a> {
            extern "C" {
                #[link_name = "rxx$cxxbridge1$Map_Matrix3d_new"]
                fn __Map_Matrix3d_new<'a>(data: *const f64,
                __return: *mut Map_Matrix3d<'a>);
            }
            let mut __return =
                ::cxx::core::mem::MaybeUninit::<Map_Matrix3d<'a>>::uninit();
            __Map_Matrix3d_new(data, __return.as_mut_ptr());
            __return.assume_init()
        }
        pub type MapMut_Matrix4d<'a> = super::MapMut_Matrix4d<'a>;
        pub type Map_Matrix4d<'a> = super::Map_Matrix4d<'a>;
        pub unsafe fn MapMut_Matrix4d_new<'a>(data: *mut f64)
            -> MapMut_Matrix4d<'a> {
            extern "C" {
                #[link_name = "rxx$cxxbridge1$MapMut_Matrix4d_new"]
                fn __MapMut_Matrix4d_new<'a>(data: *mut f64,
                __return: *mut MapMut_Matrix4d<'a>);
            }
            let mut __return =
                ::cxx::core::mem::MaybeUninit::<MapMut_Matrix4d<'a>>::uninit();
            __MapMut_Matrix4d_new(data, __return.as_mut_ptr());
            __return.assume_init()
        }
        pub unsafe fn Map_Matrix4d_new<'a>(data: *const f64)
            -> Map_Matrix4d<'a> {
            extern "C" {
                #[link_name = "rxx$cxxbridge1$Map_Matrix4d_new"]
                fn __Map_Matrix4d_new<'a>(data: *const f64,
                __return: *mut Map_Matrix4d<'a>);
            }
            let mut __return =
                ::cxx::core::mem::MaybeUninit::<Map_Matrix4d<'a>>::uninit();
            __Map_Matrix4d_new(data, __return.as_mut_ptr());
            __return.assume_init()
        }
        pub type MapMut_Matrix2x3d<'a> = super::MapMut_Matrix2x3d<'a>;
        pub type Map_Matrix2x3d<'a> = super::Map_Matrix2x3d<'a>;
        pub unsafe fn MapMut_Matrix2x3d_new<'a>(data: *mut f64)
            -> MapMut_Matrix2x3d<'a> {
            extern "C" {
                #[link_name = "rxx$cxxbridge1$MapMut_Matrix2x3d_new"]
                fn __MapMut_Matrix2x3d_new<'a>(data: *mut f64,
                __return: *mut MapMut_Matrix2x3d<'a>);
            }
            let mut __return =
                ::cxx::core::mem::MaybeUninit::<MapMut_Matrix2x3d<'a>>::uninit();
            __MapMut_Matrix2x3d_new(data, __return.as_mut_ptr());
            __return.assume_init()
        }
        pub unsafe fn Map_Matrix2x3d_new<'a>(data: *const f64)
            -> Map_Matrix2x3d<'a> {
            extern "C" {
                #[link_name = "rxx$cxxbridge1$Map_Matrix2x3d_new"]
                fn __Map_Matrix2x3d_new<'a>(data: *const f64,
                __return: *mut Map_Matrix2x3d<'a>);
            }
            let mut __return =
                ::cxx::core::mem::MaybeUninit::<Map_Matrix2x3d<'a>>::uninit();
            __Map_Matrix2x3d_new(data, __return.as_mut_ptr());
            __return.assume_init()
        }
        pub fn MatrixXd_new(rows: usize, cols: usize) -> MatrixXd {
            extern "C" {
                #[link_name = "rxx$cxxbridge1$MatrixXd_new"]
                fn __MatrixXd_new(rows: usize, cols: usize,
                __return: *mut MatrixXd);
            }
            unsafe {
                let mut __return =
                    ::cxx::core::mem::MaybeUninit::<MatrixXd>::uninit();
                __MatrixXd_new(rows, cols, __return.as_mut_ptr());
                __return.assume_init()
            }
        }
        pub fn MatrixXd_clone(v: &MatrixXd) -> MatrixXd {
            extern "C" {
                #[link_name = "rxx$cxxbridge1$MatrixXd_clone"]
                fn __MatrixXd_clone(v: *const ::cxx::core::ffi::c_void,
                __return: *mut MatrixXd);
            }
            unsafe {
                let mut __return =
                    ::cxx::core::mem::MaybeUninit::<MatrixXd>::uninit();
                __MatrixXd_clone(v as *const MatrixXd as
                        *const ::cxx::core::ffi::c_void, __return.as_mut_ptr());
                __return.assume_init()
            }
        }
        pub type MapMut_MatrixXd<'a> = super::MapMut_MatrixXd<'a>;
        pub type Map_MatrixXd<'a> = super::Map_MatrixXd<'a>;
        pub type MapMut_MatrixXd_stride<'a> =
            super::MapMut_MatrixXd_stride<'a>;
        pub type Map_MatrixXd_stride<'a> = super::Map_MatrixXd_stride<'a>;
        pub unsafe fn MapMut_MatrixXd_new<'a>(data: *mut f64, rows: usize,
            cols: usize) -> MapMut_MatrixXd<'a> {
            extern "C" {
                #[link_name = "rxx$cxxbridge1$MapMut_MatrixXd_new"]
                fn __MapMut_MatrixXd_new<'a>(data: *mut f64, rows: usize,
                cols: usize, __return: *mut MapMut_MatrixXd<'a>);
            }
            let mut __return =
                ::cxx::core::mem::MaybeUninit::<MapMut_MatrixXd<'a>>::uninit();
            __MapMut_MatrixXd_new(data, rows, cols, __return.as_mut_ptr());
            __return.assume_init()
        }
        pub unsafe fn Map_MatrixXd_new<'a>(data: *const f64, rows: usize,
            cols: usize) -> Map_MatrixXd<'a> {
            extern "C" {
                #[link_name = "rxx$cxxbridge1$Map_MatrixXd_new"]
                fn __Map_MatrixXd_new<'a>(data: *const f64, rows: usize,
                cols: usize, __return: *mut Map_MatrixXd<'a>);
            }
            let mut __return =
                ::cxx::core::mem::MaybeUninit::<Map_MatrixXd<'a>>::uninit();
            __Map_MatrixXd_new(data, rows, cols, __return.as_mut_ptr());
            __return.assume_init()
        }
        pub unsafe fn MapMut_MatrixXd_stride_new<'a>(data: *mut f64,
            rows: usize, cols: usize, s0: usize, s1: usize)
            -> MapMut_MatrixXd_stride<'a> {
            extern "C" {
                #[link_name = "rxx$cxxbridge1$MapMut_MatrixXd_stride_new"]
                fn __MapMut_MatrixXd_stride_new<'a>(data: *mut f64,
                rows: usize, cols: usize, s0: usize, s1: usize,
                __return: *mut MapMut_MatrixXd_stride<'a>);
            }
            let mut __return =
                ::cxx::core::mem::MaybeUninit::<MapMut_MatrixXd_stride<'a>>::uninit();
            __MapMut_MatrixXd_stride_new(data, rows, cols, s0, s1,
                __return.as_mut_ptr());
            __return.assume_init()
        }
        pub unsafe fn Map_MatrixXd_stride_new<'a>(data: *const f64,
            rows: usize, cols: usize, s0: usize, s1: usize)
            -> Map_MatrixXd_stride<'a> {
            extern "C" {
                #[link_name = "rxx$cxxbridge1$Map_MatrixXd_stride_new"]
                fn __Map_MatrixXd_stride_new<'a>(data: *const f64,
                rows: usize, cols: usize, s0: usize, s1: usize,
                __return: *mut Map_MatrixXd_stride<'a>);
            }
            let mut __return =
                ::cxx::core::mem::MaybeUninit::<Map_MatrixXd_stride<'a>>::uninit();
            __Map_MatrixXd_stride_new(data, rows, cols, s0, s1,
                __return.as_mut_ptr());
            __return.assume_init()
        }
        pub type MapMut_Vector2d<'a> = super::MapMut_Vector2d<'a>;
        pub type Map_Vector2d<'a> = super::Map_Vector2d<'a>;
        pub unsafe fn MapMut_Vector2d_new<'a>(data: *mut f64)
            -> MapMut_Vector2d<'a> {
            extern "C" {
                #[link_name = "rxx$cxxbridge1$MapMut_Vector2d_new"]
                fn __MapMut_Vector2d_new<'a>(data: *mut f64,
                __return: *mut MapMut_Vector2d<'a>);
            }
            let mut __return =
                ::cxx::core::mem::MaybeUninit::<MapMut_Vector2d<'a>>::uninit();
            __MapMut_Vector2d_new(data, __return.as_mut_ptr());
            __return.assume_init()
        }
        pub unsafe fn Map_Vector2d_new<'a>(data: *const f64)
            -> Map_Vector2d<'a> {
            extern "C" {
                #[link_name = "rxx$cxxbridge1$Map_Vector2d_new"]
                fn __Map_Vector2d_new<'a>(data: *const f64,
                __return: *mut Map_Vector2d<'a>);
            }
            let mut __return =
                ::cxx::core::mem::MaybeUninit::<Map_Vector2d<'a>>::uninit();
            __Map_Vector2d_new(data, __return.as_mut_ptr());
            __return.assume_init()
        }
        pub type MapMut_Vector3d<'a> = super::MapMut_Vector3d<'a>;
        pub type Map_Vector3d<'a> = super::Map_Vector3d<'a>;
        pub unsafe fn MapMut_Vector3d_new<'a>(data: *mut f64)
            -> MapMut_Vector3d<'a> {
            extern "C" {
                #[link_name = "rxx$cxxbridge1$MapMut_Vector3d_new"]
                fn __MapMut_Vector3d_new<'a>(data: *mut f64,
                __return: *mut MapMut_Vector3d<'a>);
            }
            let mut __return =
                ::cxx::core::mem::MaybeUninit::<MapMut_Vector3d<'a>>::uninit();
            __MapMut_Vector3d_new(data, __return.as_mut_ptr());
            __return.assume_init()
        }
        pub unsafe fn Map_Vector3d_new<'a>(data: *const f64)
            -> Map_Vector3d<'a> {
            extern "C" {
                #[link_name = "rxx$cxxbridge1$Map_Vector3d_new"]
                fn __Map_Vector3d_new<'a>(data: *const f64,
                __return: *mut Map_Vector3d<'a>);
            }
            let mut __return =
                ::cxx::core::mem::MaybeUninit::<Map_Vector3d<'a>>::uninit();
            __Map_Vector3d_new(data, __return.as_mut_ptr());
            __return.assume_init()
        }
        pub type MapMut_Vector4d<'a> = super::MapMut_Vector4d<'a>;
        pub type Map_Vector4d<'a> = super::Map_Vector4d<'a>;
        pub unsafe fn MapMut_Vector4d_new<'a>(data: *mut f64)
            -> MapMut_Vector4d<'a> {
            extern "C" {
                #[link_name = "rxx$cxxbridge1$MapMut_Vector4d_new"]
                fn __MapMut_Vector4d_new<'a>(data: *mut f64,
                __return: *mut MapMut_Vector4d<'a>);
            }
            let mut __return =
                ::cxx::core::mem::MaybeUninit::<MapMut_Vector4d<'a>>::uninit();
            __MapMut_Vector4d_new(data, __return.as_mut_ptr());
            __return.assume_init()
        }
        pub unsafe fn Map_Vector4d_new<'a>(data: *const f64)
            -> Map_Vector4d<'a> {
            extern "C" {
                #[link_name = "rxx$cxxbridge1$Map_Vector4d_new"]
                fn __Map_Vector4d_new<'a>(data: *const f64,
                __return: *mut Map_Vector4d<'a>);
            }
            let mut __return =
                ::cxx::core::mem::MaybeUninit::<Map_Vector4d<'a>>::uninit();
            __Map_Vector4d_new(data, __return.as_mut_ptr());
            __return.assume_init()
        }
        pub fn VectorXd_new(size: usize) -> VectorXd {
            extern "C" {
                #[link_name = "rxx$cxxbridge1$VectorXd_new"]
                fn __VectorXd_new(size: usize, __return: *mut VectorXd);
            }
            unsafe {
                let mut __return =
                    ::cxx::core::mem::MaybeUninit::<VectorXd>::uninit();
                __VectorXd_new(size, __return.as_mut_ptr());
                __return.assume_init()
            }
        }
        pub fn VectorXd_clone(v: &VectorXd) -> VectorXd {
            extern "C" {
                #[link_name = "rxx$cxxbridge1$VectorXd_clone"]
                fn __VectorXd_clone(v: *const ::cxx::core::ffi::c_void,
                __return: *mut VectorXd);
            }
            unsafe {
                let mut __return =
                    ::cxx::core::mem::MaybeUninit::<VectorXd>::uninit();
                __VectorXd_clone(v as *const VectorXd as
                        *const ::cxx::core::ffi::c_void, __return.as_mut_ptr());
                __return.assume_init()
            }
        }
        pub type MapMut_VectorXd<'a> = super::MapMut_VectorXd<'a>;
        pub type Map_VectorXd<'a> = super::Map_VectorXd<'a>;
        pub type MapMut_VectorXd_stride<'a> =
            super::MapMut_VectorXd_stride<'a>;
        pub type Map_VectorXd_stride<'a> = super::Map_VectorXd_stride<'a>;
        pub unsafe fn MapMut_VectorXd_new<'a>(data: *mut f64, size: usize)
            -> MapMut_VectorXd<'a> {
            extern "C" {
                #[link_name = "rxx$cxxbridge1$MapMut_VectorXd_new"]
                fn __MapMut_VectorXd_new<'a>(data: *mut f64, size: usize,
                __return: *mut MapMut_VectorXd<'a>);
            }
            let mut __return =
                ::cxx::core::mem::MaybeUninit::<MapMut_VectorXd<'a>>::uninit();
            __MapMut_VectorXd_new(data, size, __return.as_mut_ptr());
            __return.assume_init()
        }
        pub unsafe fn Map_VectorXd_new<'a>(data: *const f64, size: usize)
            -> Map_VectorXd<'a> {
            extern "C" {
                #[link_name = "rxx$cxxbridge1$Map_VectorXd_new"]
                fn __Map_VectorXd_new<'a>(data: *const f64, size: usize,
                __return: *mut Map_VectorXd<'a>);
            }
            let mut __return =
                ::cxx::core::mem::MaybeUninit::<Map_VectorXd<'a>>::uninit();
            __Map_VectorXd_new(data, size, __return.as_mut_ptr());
            __return.assume_init()
        }
        pub unsafe fn MapMut_VectorXd_stride_new<'a>(data: *mut f64,
            size: usize, s: usize) -> MapMut_VectorXd_stride<'a> {
            extern "C" {
                #[link_name = "rxx$cxxbridge1$MapMut_VectorXd_stride_new"]
                fn __MapMut_VectorXd_stride_new<'a>(data: *mut f64,
                size: usize, s: usize,
                __return: *mut MapMut_VectorXd_stride<'a>);
            }
            let mut __return =
                ::cxx::core::mem::MaybeUninit::<MapMut_VectorXd_stride<'a>>::uninit();
            __MapMut_VectorXd_stride_new(data, size, s,
                __return.as_mut_ptr());
            __return.assume_init()
        }
        pub unsafe fn Map_VectorXd_stride_new<'a>(data: *const f64,
            size: usize, s: usize) -> Map_VectorXd_stride<'a> {
            extern "C" {
                #[link_name = "rxx$cxxbridge1$Map_VectorXd_stride_new"]
                fn __Map_VectorXd_stride_new<'a>(data: *const f64,
                size: usize, s: usize,
                __return: *mut Map_VectorXd_stride<'a>);
            }
            let mut __return =
                ::cxx::core::mem::MaybeUninit::<Map_VectorXd_stride<'a>>::uninit();
            __Map_VectorXd_stride_new(data, size, s, __return.as_mut_ptr());
            __return.assume_init()
        }
        pub type Quaterniond = super::Quaterniond;
        impl Quaterniond {
            pub fn normalized(&self) -> Quaterniond {
                extern "C" {
                    #[link_name = "rxx$cxxbridge1$Quaterniond$normalized"]
                    fn __normalized(_: &Quaterniond,
                    __return: *mut Quaterniond);
                }
                unsafe {
                    let mut __return =
                        ::cxx::core::mem::MaybeUninit::<Quaterniond>::uninit();
                    __normalized(self, __return.as_mut_ptr());
                    __return.assume_init()
                }
            }
        }
        impl Quaterniond {
            pub fn normalize(self: ::std::pin::Pin<&mut Self>) {
                extern "C" {
                    #[link_name = "rxx$cxxbridge1$Quaterniond$normalize"]
                    fn __normalize(_: ::std::pin::Pin<&mut Quaterniond>);
                }
                unsafe { __normalize(self) }
            }
        }
        impl Quaterniond {
            pub fn inverse(&self) -> Quaterniond {
                extern "C" {
                    #[link_name = "rxx$cxxbridge1$Quaterniond$inverse"]
                    fn __inverse(_: &Quaterniond, __return: *mut Quaterniond);
                }
                unsafe {
                    let mut __return =
                        ::cxx::core::mem::MaybeUninit::<Quaterniond>::uninit();
                    __inverse(self, __return.as_mut_ptr());
                    __return.assume_init()
                }
            }
        }
        pub fn Quaterniond_mul(self_: &Quaterniond, other: &Quaterniond)
            -> Quaterniond {
            extern "C" {
                #[link_name = "rxx$cxxbridge1$Quaterniond_mul"]
                fn __Quaterniond_mul(self_: *const ::cxx::core::ffi::c_void,
                other: *const ::cxx::core::ffi::c_void,
                __return: *mut Quaterniond);
            }
            unsafe {
                let mut __return =
                    ::cxx::core::mem::MaybeUninit::<Quaterniond>::uninit();
                __Quaterniond_mul(self_ as *const Quaterniond as
                        *const ::cxx::core::ffi::c_void,
                    other as *const Quaterniond as
                        *const ::cxx::core::ffi::c_void, __return.as_mut_ptr());
                __return.assume_init()
            }
        }
        impl Quaterniond {
            pub fn toRotationMatrix(&self) -> Matrix3d {
                extern "C" {
                    #[link_name = "rxx$cxxbridge1$Quaterniond$toRotationMatrix"]
                    fn __toRotationMatrix(_: &Quaterniond,
                    __return: *mut Matrix3d);
                }
                unsafe {
                    let mut __return =
                        ::cxx::core::mem::MaybeUninit::<Matrix3d>::uninit();
                    __toRotationMatrix(self, __return.as_mut_ptr());
                    __return.assume_init()
                }
            }
        }
        pub type MapMut_Quaterniond<'a> = super::MapMut_Quaterniond<'a>;
        pub type Map_Quaterniond<'a> = super::Map_Quaterniond<'a>;
        pub unsafe fn MapMut_Quaterniond_new<'a>(data: *mut f64)
            -> MapMut_Quaterniond<'a> {
            extern "C" {
                #[link_name = "rxx$cxxbridge1$MapMut_Quaterniond_new"]
                fn __MapMut_Quaterniond_new<'a>(data: *mut f64,
                __return: *mut MapMut_Quaterniond<'a>);
            }
            let mut __return =
                ::cxx::core::mem::MaybeUninit::<MapMut_Quaterniond<'a>>::uninit();
            __MapMut_Quaterniond_new(data, __return.as_mut_ptr());
            __return.assume_init()
        }
        pub unsafe fn Map_Quaterniond_new<'a>(data: *const f64)
            -> Map_Quaterniond<'a> {
            extern "C" {
                #[link_name = "rxx$cxxbridge1$Map_Quaterniond_new"]
                fn __Map_Quaterniond_new<'a>(data: *const f64,
                __return: *mut Map_Quaterniond<'a>);
            }
            let mut __return =
                ::cxx::core::mem::MaybeUninit::<Map_Quaterniond<'a>>::uninit();
            __Map_Quaterniond_new(data, __return.as_mut_ptr());
            __return.assume_init()
        }
        pub type AngleAxisd = super::AngleAxisd;
        impl AngleAxisd {
            pub fn inverse(&self) -> AngleAxisd {
                extern "C" {
                    #[link_name = "rxx$cxxbridge1$AngleAxisd$inverse"]
                    fn __inverse(_: &AngleAxisd, __return: *mut AngleAxisd);
                }
                unsafe {
                    let mut __return =
                        ::cxx::core::mem::MaybeUninit::<AngleAxisd>::uninit();
                    __inverse(self, __return.as_mut_ptr());
                    __return.assume_init()
                }
            }
        }
        pub fn AngleAxisd_mul(self_: &AngleAxisd, other: &AngleAxisd)
            -> Quaterniond {
            extern "C" {
                #[link_name = "rxx$cxxbridge1$AngleAxisd_mul"]
                fn __AngleAxisd_mul(self_: *const ::cxx::core::ffi::c_void,
                other: *const ::cxx::core::ffi::c_void,
                __return: *mut Quaterniond);
            }
            unsafe {
                let mut __return =
                    ::cxx::core::mem::MaybeUninit::<Quaterniond>::uninit();
                __AngleAxisd_mul(self_ as *const AngleAxisd as
                        *const ::cxx::core::ffi::c_void,
                    other as *const AngleAxisd as
                        *const ::cxx::core::ffi::c_void, __return.as_mut_ptr());
                __return.assume_init()
            }
        }
        impl AngleAxisd {
            pub fn toRotationMatrix(&self) -> Matrix3d {
                extern "C" {
                    #[link_name = "rxx$cxxbridge1$AngleAxisd$toRotationMatrix"]
                    fn __toRotationMatrix(_: &AngleAxisd,
                    __return: *mut Matrix3d);
                }
                unsafe {
                    let mut __return =
                        ::cxx::core::mem::MaybeUninit::<Matrix3d>::uninit();
                    __toRotationMatrix(self, __return.as_mut_ptr());
                    __return.assume_init()
                }
            }
        }
        #[doc(hidden)]
        const _: () =
            {
                const _: fn() =
                    ::cxx::private::verify_extern_type::<Matrix3d,
                        (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a,
                        ::cxx::t, ::cxx::r, ::cxx::i, ::cxx::x, ::cxx::_3,
                        ::cxx::d)>;
                const _: fn() =
                    ::cxx::private::verify_extern_kind::<Matrix3d,
                        ::cxx::kind::Trivial>;
                const _: fn() =
                    ::cxx::private::verify_extern_type::<MatrixXd,
                        (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a,
                        ::cxx::t, ::cxx::r, ::cxx::i, ::cxx::x, ::cxx::X,
                        ::cxx::d)>;
                const _: fn() =
                    ::cxx::private::verify_extern_kind::<MatrixXd,
                        ::cxx::kind::Trivial>;
                const _: fn() =
                    ::cxx::private::verify_extern_type::<VectorXd,
                        (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::V, ::cxx::e,
                        ::cxx::c, ::cxx::t, ::cxx::o, ::cxx::r, ::cxx::X,
                        ::cxx::d)>;
                const _: fn() =
                    ::cxx::private::verify_extern_kind::<VectorXd,
                        ::cxx::kind::Trivial>;
                const _: fn() =
                    ::cxx::private::verify_extern_type::<MapMut_Matrix2d,
                        (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a,
                        ::cxx::p, ::cxx::M, ::cxx::u, ::cxx::t, ::cxx::__, ::cxx::M,
                        ::cxx::a, ::cxx::t, ::cxx::r, ::cxx::i, ::cxx::x, ::cxx::_2,
                        ::cxx::d)>;
                const _: fn() =
                    ::cxx::private::verify_extern_kind::<MapMut_Matrix2d,
                        ::cxx::kind::Trivial>;
                const _: fn() =
                    ::cxx::private::verify_extern_type::<Map_Matrix2d,
                        (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a,
                        ::cxx::p, ::cxx::__, ::cxx::M, ::cxx::a, ::cxx::t, ::cxx::r,
                        ::cxx::i, ::cxx::x, ::cxx::_2, ::cxx::d)>;
                const _: fn() =
                    ::cxx::private::verify_extern_kind::<Map_Matrix2d,
                        ::cxx::kind::Trivial>;
                const _: fn() =
                    ::cxx::private::verify_extern_type::<MapMut_Matrix2i,
                        (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a,
                        ::cxx::p, ::cxx::M, ::cxx::u, ::cxx::t, ::cxx::__, ::cxx::M,
                        ::cxx::a, ::cxx::t, ::cxx::r, ::cxx::i, ::cxx::x, ::cxx::_2,
                        ::cxx::i)>;
                const _: fn() =
                    ::cxx::private::verify_extern_kind::<MapMut_Matrix2i,
                        ::cxx::kind::Trivial>;
                const _: fn() =
                    ::cxx::private::verify_extern_type::<Map_Matrix2i,
                        (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a,
                        ::cxx::p, ::cxx::__, ::cxx::M, ::cxx::a, ::cxx::t, ::cxx::r,
                        ::cxx::i, ::cxx::x, ::cxx::_2, ::cxx::i)>;
                const _: fn() =
                    ::cxx::private::verify_extern_kind::<Map_Matrix2i,
                        ::cxx::kind::Trivial>;
                const _: fn() =
                    ::cxx::private::verify_extern_type::<MapMut_Matrix3d,
                        (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a,
                        ::cxx::p, ::cxx::M, ::cxx::u, ::cxx::t, ::cxx::__, ::cxx::M,
                        ::cxx::a, ::cxx::t, ::cxx::r, ::cxx::i, ::cxx::x, ::cxx::_3,
                        ::cxx::d)>;
                const _: fn() =
                    ::cxx::private::verify_extern_kind::<MapMut_Matrix3d,
                        ::cxx::kind::Trivial>;
                const _: fn() =
                    ::cxx::private::verify_extern_type::<Map_Matrix3d,
                        (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a,
                        ::cxx::p, ::cxx::__, ::cxx::M, ::cxx::a, ::cxx::t, ::cxx::r,
                        ::cxx::i, ::cxx::x, ::cxx::_3, ::cxx::d)>;
                const _: fn() =
                    ::cxx::private::verify_extern_kind::<Map_Matrix3d,
                        ::cxx::kind::Trivial>;
                const _: fn() =
                    ::cxx::private::verify_extern_type::<MapMut_Matrix4d,
                        (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a,
                        ::cxx::p, ::cxx::M, ::cxx::u, ::cxx::t, ::cxx::__, ::cxx::M,
                        ::cxx::a, ::cxx::t, ::cxx::r, ::cxx::i, ::cxx::x, ::cxx::_4,
                        ::cxx::d)>;
                const _: fn() =
                    ::cxx::private::verify_extern_kind::<MapMut_Matrix4d,
                        ::cxx::kind::Trivial>;
                const _: fn() =
                    ::cxx::private::verify_extern_type::<Map_Matrix4d,
                        (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a,
                        ::cxx::p, ::cxx::__, ::cxx::M, ::cxx::a, ::cxx::t, ::cxx::r,
                        ::cxx::i, ::cxx::x, ::cxx::_4, ::cxx::d)>;
                const _: fn() =
                    ::cxx::private::verify_extern_kind::<Map_Matrix4d,
                        ::cxx::kind::Trivial>;
                const _: fn() =
                    ::cxx::private::verify_extern_type::<MapMut_Matrix2x3d,
                        (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a,
                        ::cxx::p, ::cxx::M, ::cxx::u, ::cxx::t, ::cxx::__, ::cxx::M,
                        ::cxx::a, ::cxx::t, ::cxx::r, ::cxx::i, ::cxx::x, ::cxx::_2,
                        ::cxx::x, ::cxx::_3, ::cxx::d)>;
                const _: fn() =
                    ::cxx::private::verify_extern_kind::<MapMut_Matrix2x3d,
                        ::cxx::kind::Trivial>;
                const _: fn() =
                    ::cxx::private::verify_extern_type::<Map_Matrix2x3d,
                        (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a,
                        ::cxx::p, ::cxx::__, ::cxx::M, ::cxx::a, ::cxx::t, ::cxx::r,
                        ::cxx::i, ::cxx::x, ::cxx::_2, ::cxx::x, ::cxx::_3,
                        ::cxx::d)>;
                const _: fn() =
                    ::cxx::private::verify_extern_kind::<Map_Matrix2x3d,
                        ::cxx::kind::Trivial>;
                const _: fn() =
                    ::cxx::private::verify_extern_type::<MapMut_MatrixXd,
                        (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a,
                        ::cxx::p, ::cxx::M, ::cxx::u, ::cxx::t, ::cxx::__, ::cxx::M,
                        ::cxx::a, ::cxx::t, ::cxx::r, ::cxx::i, ::cxx::x, ::cxx::X,
                        ::cxx::d)>;
                const _: fn() =
                    ::cxx::private::verify_extern_kind::<MapMut_MatrixXd,
                        ::cxx::kind::Trivial>;
                const _: fn() =
                    ::cxx::private::verify_extern_type::<Map_MatrixXd,
                        (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a,
                        ::cxx::p, ::cxx::__, ::cxx::M, ::cxx::a, ::cxx::t, ::cxx::r,
                        ::cxx::i, ::cxx::x, ::cxx::X, ::cxx::d)>;
                const _: fn() =
                    ::cxx::private::verify_extern_kind::<Map_MatrixXd,
                        ::cxx::kind::Trivial>;
                const _: fn() =
                    ::cxx::private::verify_extern_type::<MapMut_MatrixXd_stride,
                        (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a,
                        ::cxx::p, ::cxx::M, ::cxx::u, ::cxx::t, ::cxx::__, ::cxx::M,
                        ::cxx::a, ::cxx::t, ::cxx::r, ::cxx::i, ::cxx::x, ::cxx::X,
                        ::cxx::d, ::cxx::__, ::cxx::s, ::cxx::t, ::cxx::r, ::cxx::i,
                        ::cxx::d, ::cxx::e)>;
                const _: fn() =
                    ::cxx::private::verify_extern_kind::<MapMut_MatrixXd_stride,
                        ::cxx::kind::Trivial>;
                const _: fn() =
                    ::cxx::private::verify_extern_type::<Map_MatrixXd_stride,
                        (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a,
                        ::cxx::p, ::cxx::__, ::cxx::M, ::cxx::a, ::cxx::t, ::cxx::r,
                        ::cxx::i, ::cxx::x, ::cxx::X, ::cxx::d, ::cxx::__, ::cxx::s,
                        ::cxx::t, ::cxx::r, ::cxx::i, ::cxx::d, ::cxx::e)>;
                const _: fn() =
                    ::cxx::private::verify_extern_kind::<Map_MatrixXd_stride,
                        ::cxx::kind::Trivial>;
                const _: fn() =
                    ::cxx::private::verify_extern_type::<MapMut_Vector2d,
                        (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a,
                        ::cxx::p, ::cxx::M, ::cxx::u, ::cxx::t, ::cxx::__, ::cxx::V,
                        ::cxx::e, ::cxx::c, ::cxx::t, ::cxx::o, ::cxx::r, ::cxx::_2,
                        ::cxx::d)>;
                const _: fn() =
                    ::cxx::private::verify_extern_kind::<MapMut_Vector2d,
                        ::cxx::kind::Trivial>;
                const _: fn() =
                    ::cxx::private::verify_extern_type::<Map_Vector2d,
                        (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a,
                        ::cxx::p, ::cxx::__, ::cxx::V, ::cxx::e, ::cxx::c, ::cxx::t,
                        ::cxx::o, ::cxx::r, ::cxx::_2, ::cxx::d)>;
                const _: fn() =
                    ::cxx::private::verify_extern_kind::<Map_Vector2d,
                        ::cxx::kind::Trivial>;
                const _: fn() =
                    ::cxx::private::verify_extern_type::<MapMut_Vector3d,
                        (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a,
                        ::cxx::p, ::cxx::M, ::cxx::u, ::cxx::t, ::cxx::__, ::cxx::V,
                        ::cxx::e, ::cxx::c, ::cxx::t, ::cxx::o, ::cxx::r, ::cxx::_3,
                        ::cxx::d)>;
                const _: fn() =
                    ::cxx::private::verify_extern_kind::<MapMut_Vector3d,
                        ::cxx::kind::Trivial>;
                const _: fn() =
                    ::cxx::private::verify_extern_type::<Map_Vector3d,
                        (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a,
                        ::cxx::p, ::cxx::__, ::cxx::V, ::cxx::e, ::cxx::c, ::cxx::t,
                        ::cxx::o, ::cxx::r, ::cxx::_3, ::cxx::d)>;
                const _: fn() =
                    ::cxx::private::verify_extern_kind::<Map_Vector3d,
                        ::cxx::kind::Trivial>;
                const _: fn() =
                    ::cxx::private::verify_extern_type::<MapMut_Vector4d,
                        (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a,
                        ::cxx::p, ::cxx::M, ::cxx::u, ::cxx::t, ::cxx::__, ::cxx::V,
                        ::cxx::e, ::cxx::c, ::cxx::t, ::cxx::o, ::cxx::r, ::cxx::_4,
                        ::cxx::d)>;
                const _: fn() =
                    ::cxx::private::verify_extern_kind::<MapMut_Vector4d,
                        ::cxx::kind::Trivial>;
                const _: fn() =
                    ::cxx::private::verify_extern_type::<Map_Vector4d,
                        (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a,
                        ::cxx::p, ::cxx::__, ::cxx::V, ::cxx::e, ::cxx::c, ::cxx::t,
                        ::cxx::o, ::cxx::r, ::cxx::_4, ::cxx::d)>;
                const _: fn() =
                    ::cxx::private::verify_extern_kind::<Map_Vector4d,
                        ::cxx::kind::Trivial>;
                const _: fn() =
                    ::cxx::private::verify_extern_type::<MapMut_VectorXd,
                        (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a,
                        ::cxx::p, ::cxx::M, ::cxx::u, ::cxx::t, ::cxx::__, ::cxx::V,
                        ::cxx::e, ::cxx::c, ::cxx::t, ::cxx::o, ::cxx::r, ::cxx::X,
                        ::cxx::d)>;
                const _: fn() =
                    ::cxx::private::verify_extern_kind::<MapMut_VectorXd,
                        ::cxx::kind::Trivial>;
                const _: fn() =
                    ::cxx::private::verify_extern_type::<Map_VectorXd,
                        (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a,
                        ::cxx::p, ::cxx::__, ::cxx::V, ::cxx::e, ::cxx::c, ::cxx::t,
                        ::cxx::o, ::cxx::r, ::cxx::X, ::cxx::d)>;
                const _: fn() =
                    ::cxx::private::verify_extern_kind::<Map_VectorXd,
                        ::cxx::kind::Trivial>;
                const _: fn() =
                    ::cxx::private::verify_extern_type::<MapMut_VectorXd_stride,
                        (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a,
                        ::cxx::p, ::cxx::M, ::cxx::u, ::cxx::t, ::cxx::__, ::cxx::V,
                        ::cxx::e, ::cxx::c, ::cxx::t, ::cxx::o, ::cxx::r, ::cxx::X,
                        ::cxx::d, ::cxx::__, ::cxx::s, ::cxx::t, ::cxx::r, ::cxx::i,
                        ::cxx::d, ::cxx::e)>;
                const _: fn() =
                    ::cxx::private::verify_extern_kind::<MapMut_VectorXd_stride,
                        ::cxx::kind::Trivial>;
                const _: fn() =
                    ::cxx::private::verify_extern_type::<Map_VectorXd_stride,
                        (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a,
                        ::cxx::p, ::cxx::__, ::cxx::V, ::cxx::e, ::cxx::c, ::cxx::t,
                        ::cxx::o, ::cxx::r, ::cxx::X, ::cxx::d, ::cxx::__, ::cxx::s,
                        ::cxx::t, ::cxx::r, ::cxx::i, ::cxx::d, ::cxx::e)>;
                const _: fn() =
                    ::cxx::private::verify_extern_kind::<Map_VectorXd_stride,
                        ::cxx::kind::Trivial>;
                const _: fn() =
                    ::cxx::private::verify_extern_type::<Quaterniond,
                        (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::Q, ::cxx::u,
                        ::cxx::a, ::cxx::t, ::cxx::e, ::cxx::r, ::cxx::n, ::cxx::i,
                        ::cxx::o, ::cxx::n, ::cxx::d)>;
                const _: fn() =
                    ::cxx::private::verify_extern_kind::<Quaterniond,
                        ::cxx::kind::Trivial>;
                const _: fn() =
                    ::cxx::private::verify_extern_type::<MapMut_Quaterniond,
                        (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a,
                        ::cxx::p, ::cxx::M, ::cxx::u, ::cxx::t, ::cxx::__, ::cxx::Q,
                        ::cxx::u, ::cxx::a, ::cxx::t, ::cxx::e, ::cxx::r, ::cxx::n,
                        ::cxx::i, ::cxx::o, ::cxx::n, ::cxx::d)>;
                const _: fn() =
                    ::cxx::private::verify_extern_kind::<MapMut_Quaterniond,
                        ::cxx::kind::Trivial>;
                const _: fn() =
                    ::cxx::private::verify_extern_type::<Map_Quaterniond,
                        (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::M, ::cxx::a,
                        ::cxx::p, ::cxx::__, ::cxx::Q, ::cxx::u, ::cxx::a, ::cxx::t,
                        ::cxx::e, ::cxx::r, ::cxx::n, ::cxx::i, ::cxx::o, ::cxx::n,
                        ::cxx::d)>;
                const _: fn() =
                    ::cxx::private::verify_extern_kind::<Map_Quaterniond,
                        ::cxx::kind::Trivial>;
                const _: fn() =
                    ::cxx::private::verify_extern_type::<AngleAxisd,
                        (::cxx::r, ::cxx::x, ::cxx::x, (), ::cxx::A, ::cxx::n,
                        ::cxx::g, ::cxx::l, ::cxx::e, ::cxx::A, ::cxx::x, ::cxx::i,
                        ::cxx::s, ::cxx::d)>;
                const _: fn() =
                    ::cxx::private::verify_extern_kind::<AngleAxisd,
                        ::cxx::kind::Trivial>;
            };
    }
    impl VectorXd {
        pub fn new(size: usize) -> Self { ffi::VectorXd_new(size) }
    }
    impl Clone for VectorXd {
        fn clone(&self) -> Self { ffi::VectorXd_clone(self) }
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
        fn clone(&self) -> Self { ffi::MatrixXd_clone(self) }
        fn clone_from(&mut self, source: &Self) {
            *self = ffi::MatrixXd_clone(source);
        }
    }
    impl<'a> MapMut_Matrix2d<'a> {
        pub fn new(data: &'a mut [f64]) -> Self {
            unsafe { ffi::MapMut_Matrix2d_new::<'a>(data.as_mut_ptr()) }
        }
    }
    impl<'a> Map_Matrix2d<'a> {
        pub fn new(data: &'a [f64]) -> Self {
            unsafe { ffi::Map_Matrix2d_new::<'a>(data.as_ptr()) }
        }
    }
    impl From<na::Matrix2<f64>> for Matrix2d {
        fn from(v: na::Matrix2<f64>) -> Self {
            Self { data: v.as_slice().try_into().unwrap() }
        }
    }
    impl From<Matrix2d> for na::Matrix2<f64> {
        fn from(v: Matrix2d) -> Self {
            let d =
                unsafe {
                    std::mem::transmute::<[f64; 4], [[f64; 2]; 2]>(v.data)
                };
            Self::from_data(na::ArrayStorage(d))
        }
    }
    impl<'a> From<na::MatrixSlice2<'a, f64>> for Map_Matrix2d<'a> {
        fn from(v: na::MatrixSlice2<'a, f64>) -> Self {
            Self::new(v.data.into_slice())
        }
    }
    impl<'a> From<Map_Matrix2d<'a>> for na::MatrixSlice2<'a, f64> {
        fn from(v: Map_Matrix2d<'a>) -> Self {
            let s =
                unsafe {
                    na::SliceStorage::from_raw_parts(v.data,
                        (Dim::from_usize(2), Dim::from_usize(2)),
                        (Dim::from_usize(1), Dim::from_usize(2)))
                };
            Self::from_data(s)
        }
    }
    impl<'a> From<na::MatrixSliceMut2<'a, f64>> for MapMut_Matrix2d<'a> {
        fn from(v: na::MatrixSliceMut2<'a, f64>) -> Self {
            Self::new(v.data.into_slice_mut())
        }
    }
    impl<'a> From<MapMut_Matrix2d<'a>> for na::MatrixSliceMut2<'a, f64> {
        fn from(v: MapMut_Matrix2d<'a>) -> Self {
            let s =
                unsafe {
                    na::SliceStorageMut::from_raw_parts(v.data,
                        (Dim::from_usize(2), Dim::from_usize(2)),
                        (Dim::from_usize(1), Dim::from_usize(2)))
                };
            Self::from_data(s)
        }
    }
    impl<'a> MapMut_Matrix2i<'a> {
        pub fn new(data: &'a mut [i32]) -> Self {
            unsafe { ffi::MapMut_Matrix2i_new::<'a>(data.as_mut_ptr()) }
        }
    }
    impl<'a> Map_Matrix2i<'a> {
        pub fn new(data: &'a [i32]) -> Self {
            unsafe { ffi::Map_Matrix2i_new::<'a>(data.as_ptr()) }
        }
    }
    impl From<na::Matrix2<i32>> for Matrix2i {
        fn from(v: na::Matrix2<i32>) -> Self {
            Self { data: v.as_slice().try_into().unwrap() }
        }
    }
    impl From<Matrix2i> for na::Matrix2<i32> {
        fn from(v: Matrix2i) -> Self {
            let d =
                unsafe {
                    std::mem::transmute::<[i32; 4], [[i32; 2]; 2]>(v.data)
                };
            Self::from_data(na::ArrayStorage(d))
        }
    }
    impl<'a> From<na::MatrixSlice2<'a, i32>> for Map_Matrix2i<'a> {
        fn from(v: na::MatrixSlice2<'a, i32>) -> Self {
            Self::new(v.data.into_slice())
        }
    }
    impl<'a> From<Map_Matrix2i<'a>> for na::MatrixSlice2<'a, i32> {
        fn from(v: Map_Matrix2i<'a>) -> Self {
            let s =
                unsafe {
                    na::SliceStorage::from_raw_parts(v.data,
                        (Dim::from_usize(2), Dim::from_usize(2)),
                        (Dim::from_usize(1), Dim::from_usize(2)))
                };
            Self::from_data(s)
        }
    }
    impl<'a> From<na::MatrixSliceMut2<'a, i32>> for MapMut_Matrix2i<'a> {
        fn from(v: na::MatrixSliceMut2<'a, i32>) -> Self {
            Self::new(v.data.into_slice_mut())
        }
    }
    impl<'a> From<MapMut_Matrix2i<'a>> for na::MatrixSliceMut2<'a, i32> {
        fn from(v: MapMut_Matrix2i<'a>) -> Self {
            let s =
                unsafe {
                    na::SliceStorageMut::from_raw_parts(v.data,
                        (Dim::from_usize(2), Dim::from_usize(2)),
                        (Dim::from_usize(1), Dim::from_usize(2)))
                };
            Self::from_data(s)
        }
    }
    impl<'a> MapMut_Matrix3d<'a> {
        pub fn new(data: &'a mut [f64]) -> Self {
            unsafe { ffi::MapMut_Matrix3d_new::<'a>(data.as_mut_ptr()) }
        }
    }
    impl<'a> Map_Matrix3d<'a> {
        pub fn new(data: &'a [f64]) -> Self {
            unsafe { ffi::Map_Matrix3d_new::<'a>(data.as_ptr()) }
        }
    }
    impl From<na::Matrix3<f64>> for Matrix3d {
        fn from(v: na::Matrix3<f64>) -> Self {
            Self { data: v.as_slice().try_into().unwrap() }
        }
    }
    impl From<Matrix3d> for na::Matrix3<f64> {
        fn from(v: Matrix3d) -> Self {
            let d =
                unsafe {
                    std::mem::transmute::<[f64; 9], [[f64; 3]; 3]>(v.data)
                };
            Self::from_data(na::ArrayStorage(d))
        }
    }
    impl<'a> From<na::MatrixSlice3<'a, f64>> for Map_Matrix3d<'a> {
        fn from(v: na::MatrixSlice3<'a, f64>) -> Self {
            Self::new(v.data.into_slice())
        }
    }
    impl<'a> From<Map_Matrix3d<'a>> for na::MatrixSlice3<'a, f64> {
        fn from(v: Map_Matrix3d<'a>) -> Self {
            let s =
                unsafe {
                    na::SliceStorage::from_raw_parts(v.data,
                        (Dim::from_usize(3), Dim::from_usize(3)),
                        (Dim::from_usize(1), Dim::from_usize(3)))
                };
            Self::from_data(s)
        }
    }
    impl<'a> From<na::MatrixSliceMut3<'a, f64>> for MapMut_Matrix3d<'a> {
        fn from(v: na::MatrixSliceMut3<'a, f64>) -> Self {
            Self::new(v.data.into_slice_mut())
        }
    }
    impl<'a> From<MapMut_Matrix3d<'a>> for na::MatrixSliceMut3<'a, f64> {
        fn from(v: MapMut_Matrix3d<'a>) -> Self {
            let s =
                unsafe {
                    na::SliceStorageMut::from_raw_parts(v.data,
                        (Dim::from_usize(3), Dim::from_usize(3)),
                        (Dim::from_usize(1), Dim::from_usize(3)))
                };
            Self::from_data(s)
        }
    }
    impl<'a> MapMut_Matrix4d<'a> {
        pub fn new(data: &'a mut [f64]) -> Self {
            unsafe { ffi::MapMut_Matrix4d_new::<'a>(data.as_mut_ptr()) }
        }
    }
    impl<'a> Map_Matrix4d<'a> {
        pub fn new(data: &'a [f64]) -> Self {
            unsafe { ffi::Map_Matrix4d_new::<'a>(data.as_ptr()) }
        }
    }
    impl From<na::Matrix4<f64>> for Matrix4d {
        fn from(v: na::Matrix4<f64>) -> Self {
            Self { data: v.as_slice().try_into().unwrap() }
        }
    }
    impl From<Matrix4d> for na::Matrix4<f64> {
        fn from(v: Matrix4d) -> Self {
            let d =
                unsafe {
                    std::mem::transmute::<[f64; 16], [[f64; 4]; 4]>(v.data)
                };
            Self::from_data(na::ArrayStorage(d))
        }
    }
    impl<'a> From<na::MatrixSlice4<'a, f64>> for Map_Matrix4d<'a> {
        fn from(v: na::MatrixSlice4<'a, f64>) -> Self {
            Self::new(v.data.into_slice())
        }
    }
    impl<'a> From<Map_Matrix4d<'a>> for na::MatrixSlice4<'a, f64> {
        fn from(v: Map_Matrix4d<'a>) -> Self {
            let s =
                unsafe {
                    na::SliceStorage::from_raw_parts(v.data,
                        (Dim::from_usize(4), Dim::from_usize(4)),
                        (Dim::from_usize(1), Dim::from_usize(4)))
                };
            Self::from_data(s)
        }
    }
    impl<'a> From<na::MatrixSliceMut4<'a, f64>> for MapMut_Matrix4d<'a> {
        fn from(v: na::MatrixSliceMut4<'a, f64>) -> Self {
            Self::new(v.data.into_slice_mut())
        }
    }
    impl<'a> From<MapMut_Matrix4d<'a>> for na::MatrixSliceMut4<'a, f64> {
        fn from(v: MapMut_Matrix4d<'a>) -> Self {
            let s =
                unsafe {
                    na::SliceStorageMut::from_raw_parts(v.data,
                        (Dim::from_usize(4), Dim::from_usize(4)),
                        (Dim::from_usize(1), Dim::from_usize(4)))
                };
            Self::from_data(s)
        }
    }
    impl<'a> MapMut_Matrix2x3d<'a> {
        pub fn new(data: &'a mut [f64]) -> Self {
            unsafe { ffi::MapMut_Matrix2x3d_new::<'a>(data.as_mut_ptr()) }
        }
    }
    impl<'a> Map_Matrix2x3d<'a> {
        pub fn new(data: &'a [f64]) -> Self {
            unsafe { ffi::Map_Matrix2x3d_new::<'a>(data.as_ptr()) }
        }
    }
    impl<'a> MapMut_MatrixXd<'a> {
        pub fn new(data: &'a mut [f64], rows: usize, cols: usize) -> Self {
            unsafe {
                ffi::MapMut_MatrixXd_new::<'a>(data.as_mut_ptr(), rows, cols)
            }
        }
    }
    impl<'a> Map_MatrixXd<'a> {
        pub fn new(data: &'a [f64], rows: usize, cols: usize) -> Self {
            unsafe { ffi::Map_MatrixXd_new::<'a>(data.as_ptr(), rows, cols) }
        }
    }
    impl<'a> MapMut_MatrixXd_stride<'a> {
        pub fn new(data: &'a mut [f64], rows: usize, cols: usize, s0: usize,
            s1: usize) -> Self {
            unsafe {
                ffi::MapMut_MatrixXd_stride_new::<'a>(data.as_mut_ptr(), rows,
                    cols, s0, s1)
            }
        }
    }
    impl<'a> Map_MatrixXd_stride<'a> {
        pub fn new(data: &'a [f64], rows: usize, cols: usize, s0: usize,
            s1: usize) -> Self {
            unsafe {
                ffi::Map_MatrixXd_stride_new::<'a>(data.as_ptr(), rows, cols,
                    s0, s1)
            }
        }
    }
    impl<'a> MapMut_Vector2d<'a> {
        pub fn new(data: &'a mut [f64]) -> Self {
            unsafe { ffi::MapMut_Vector2d_new::<'a>(data.as_mut_ptr()) }
        }
    }
    impl<'a> Map_Vector2d<'a> {
        pub fn new(data: &'a [f64]) -> Self {
            unsafe { ffi::Map_Vector2d_new::<'a>(data.as_ptr()) }
        }
    }
    impl<'a> MapMut_Vector3d<'a> {
        pub fn new(data: &'a mut [f64]) -> Self {
            unsafe { ffi::MapMut_Vector3d_new::<'a>(data.as_mut_ptr()) }
        }
    }
    impl<'a> Map_Vector3d<'a> {
        pub fn new(data: &'a [f64]) -> Self {
            unsafe { ffi::Map_Vector3d_new::<'a>(data.as_ptr()) }
        }
    }
    impl<'a> MapMut_Vector4d<'a> {
        pub fn new(data: &'a mut [f64]) -> Self {
            unsafe { ffi::MapMut_Vector4d_new::<'a>(data.as_mut_ptr()) }
        }
    }
    impl<'a> Map_Vector4d<'a> {
        pub fn new(data: &'a [f64]) -> Self {
            unsafe { ffi::Map_Vector4d_new::<'a>(data.as_ptr()) }
        }
    }
    impl<'a> MapMut_VectorXd<'a> {
        pub fn new(data: &'a mut [f64], size: usize) -> Self {
            unsafe { ffi::MapMut_VectorXd_new::<'a>(data.as_mut_ptr(), size) }
        }
    }
    impl<'a> Map_VectorXd<'a> {
        pub fn new(data: &'a [f64], size: usize) -> Self {
            unsafe { ffi::Map_VectorXd_new::<'a>(data.as_ptr(), size) }
        }
    }
    impl<'a> MapMut_VectorXd_stride<'a> {
        pub fn new(data: &'a mut [f64], size: usize, s: usize) -> Self {
            unsafe {
                ffi::MapMut_VectorXd_stride_new::<'a>(data.as_mut_ptr(), size,
                    s)
            }
        }
    }
    impl<'a> Map_VectorXd_stride<'a> {
        pub fn new(data: &'a [f64], size: usize, s: usize) -> Self {
            unsafe {
                ffi::Map_VectorXd_stride_new::<'a>(data.as_ptr(), size, s)
            }
        }
    }
    impl Mul for Quaterniond {
        type Output = Quaterniond;
        fn mul(self, other: Self) -> Self::Output {
            ffi::Quaterniond_mul(&self, &other)
        }
    }
    impl Mul for &Quaterniond {
        type Output = Quaterniond;
        fn mul(self, other: Self) -> Self::Output {
            ffi::Quaterniond_mul(self, other)
        }
    }
    impl<'a> MapMut_Quaterniond<'a> {
        pub fn new(data: &'a mut [f64]) -> Self {
            unsafe { ffi::MapMut_Quaterniond_new::<'a>(data.as_mut_ptr()) }
        }
    }
    impl<'a> Map_Quaterniond<'a> {
        pub fn new(data: &'a [f64]) -> Self {
            unsafe { ffi::Map_Quaterniond_new::<'a>(data.as_ptr()) }
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
        fn from(v: Quaterniond) -> Self { Self::new(v.w, v.x, v.y, v.z) }
    }
    impl<'a> From<&'a na::Quaternion<f64>> for Map_Quaterniond<'a> {
        fn from(v: &'a na::Quaternion<f64>) -> Self {
            Self::new(v.coords.as_slice())
        }
    }
    impl<'a> From<Map_Quaterniond<'a>> for na::Quaternion<f64> {
        fn from(v: Map_Quaterniond<'a>) -> Self {
            let d = unsafe { std::slice::from_raw_parts(v.data, 4) };
            Self::from_vector(na::Vector4::from_column_slice(d))
        }
    }
    impl<'a> From<&'a mut na::Quaternion<f64>> for MapMut_Quaterniond<'a> {
        fn from(v: &'a mut na::Quaternion<f64>) -> Self {
            Self::new(v.coords.as_mut_slice())
        }
    }
    impl<'a> From<MapMut_Quaterniond<'a>> for na::Quaternion<f64> {
        fn from(v: MapMut_Quaterniond<'a>) -> Self {
            let d = unsafe { std::slice::from_raw_parts(v.data, 4) };
            Self::from_vector(na::Vector4::from_column_slice(d))
        }
    }
    impl Mul for AngleAxisd {
        type Output = Quaterniond;
        fn mul(self, other: Self) -> Self::Output {
            ffi::AngleAxisd_mul(&self, &other)
        }
    }
    impl Mul for &AngleAxisd {
        type Output = Quaterniond;
        fn mul(self, other: Self) -> Self::Output {
            ffi::AngleAxisd_mul(self, other)
        }
    }
}
pub use ffi::*;
extern crate nalgebra as na;

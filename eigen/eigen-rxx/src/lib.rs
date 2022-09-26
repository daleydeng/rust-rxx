#![allow(incomplete_features, non_camel_case_types, non_snake_case, clippy::identity_op)]
#![feature(generic_const_exprs, pin_macro)]

extern crate nalgebra as na;

pub mod ffi;
pub use ffi::*;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_basic() {
        let x = na::Matrix2::new(1., 3., 2., 4.);
        let raw_x: ffi::Matrix2d = x.into();
        assert_eq!(raw_x.to_string(), "1 3\n2 4");

        assert_eq!(&raw_x.data, &[1., 2., 3., 4.]);
        let x_rec: na::Matrix2<f64> = raw_x.into();
        assert_eq!(x, x_rec);

        let x = na::Matrix2::new(1, 3, 2, 4);
        let raw_x: ffi::Matrix<_, 2, 2> = x.into();
        assert_eq!(&raw_x.data, &[1, 2, 3, 4]);
        let x_rec1: na::Matrix2<i32> = raw_x.into();
        assert_eq!(x, x_rec1);
    }

    #[test]
    fn test_map() {
        let x = na::Matrix2::new(1., 3., 2., 4.);
        let x: na::MatrixSlice2<_> = (&x).into();
        let raw_x = ffi::Map::<ffi::Matrix2d>::from(x);
        assert_eq!(raw_x.to_string(), "1 3\n2 4");

        // raw_x.print();
        let x_rec = na::MatrixSlice2::from(raw_x);
        assert_eq!(x.as_slice(), x_rec.as_slice());
    }

    #[test]
    fn test_mut_map() {
        let mut x0 = na::Matrix2::new(1., 3., 2., 4.);

        let x_rec: na::Matrix2<_> = {
            let x: na::MatrixSliceMut2<_> = (&mut x0).into();
            let y: ffi::MapMut<ffi::Matrix2d> = x.into();
            assert_eq!(y.to_string(), "1 3\n2 4");
            na::MatrixSliceMut2::from(y).into()
        };

        x0[(0, 0)] = 4.;

        let x_rec1: na::Matrix2<_> = {
            let x: na::MatrixSliceMut2<_> = (&mut x0).into();
            let y: ffi::MapMut<ffi::Matrix2d> = x.into();
            assert_eq!(y.to_string(), "4 3\n2 4");
            na::MatrixSliceMut2::from(y).into()
        };

        assert_eq!(x_rec1 - x_rec, na::Matrix2::new(3., 0., 0., 0.))
    }

    #[test]
    fn test_quat() {
        let q = na::Quaternion::new(1., 2., 3., 4.);
        let x: ffi::Quaterniond = q.into();
        let q_rec: na::Quaternion<_> = x.into();
        assert_eq!(q, q_rec);

        let q1 = na::Quaternion::new(5., 6., 7., 8.);
        let x1: ffi::Quaterniond = q1.into();

        let q_prod = q * q1;
        let x_prod_rec: na::Quaternion<_> = (x * x1).into();
        assert_eq!(q_prod, x_prod_rec)
    }

    #[test]
    fn test_map_quat() {
        let q = na::Quaternion::new(1., 2., 3., 4.);
        let x: ffi::Map<ffi::Quaterniond> = (&q).into();
        let q_rec: na::Quaternion<_> = x.into();
        assert_eq!(q, q_rec);
    }

    #[test]
    fn test_mut_map_quat() {
        let mut q = na::Quaternion::new(1., 2., 3., 4.);
        let x: ffi::MapMut<ffi::Quaterniond> = (&mut q).into();
        let q_rec: na::Quaternion<_> = x.into();
        assert_eq!(q, q_rec);
    }
}

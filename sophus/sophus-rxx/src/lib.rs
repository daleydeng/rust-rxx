use eigen_rxx::*;
use num::traits::Float;
use std::ops::Mul;

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct SO2<T: Float> {
    pub unit_complex_: Matrix<T, 2, 1>,
}

pub type SO2d = SO2<f64>;

rxx_macro::genrs_fn!(
    impl SO2d {
    #[ffi(link_name="sophus_rxx_get_SO2d_DoF", atomic)]
    pub fn dof() -> i32 {}
    #[ffi(link_name="sophus_rxx_get_SO2d_num_parameters", atomic)]
    pub fn num_parameters() -> i32 {}
    #[ffi(link_name="sophus_rxx_get_SO2d_N", atomic)]
    pub fn n() -> i32 {}
    #[ffi(link_name="sophus_rxx_get_SO2d_Dim", atomic)]
    pub fn dim() -> i32 {}
    #[ffi(link_name="sophus_rxx_SO2d_Adj", atomic)]
    pub fn adj(&self) -> f64 {}
    #[ffi(link_name="sophus_rxx_SO2d_inverse", object)]
    pub fn inverse(&self) -> SO2d {}
    #[ffi(link_name="sophus_rxx_SO2d_log", atomic)]
    pub fn log(&self) -> f64 {}
    #[ffi(link_name="sophus_rxx_SO2d_normalize")]
    pub fn normalize(&mut self) {}
    #[ffi(link_name="sophus_rxx_SO2d_matrix")]
    pub fn matrix(&self) -> Matrix2d {}

    #[ffi(link_name="sophus_rxx_SO2d_params")]
    pub fn params(&self) -> Vector2d {}

    #[ffi(link_name="sophus_rxx_SO2d_setComplex")]
    pub fn set_complex(&mut self, v: &Vector2d) {}

    #[ffi(link_name="sophus_rxx_SO2d_unit_complex", atomic)]
    pub fn unit_complex(&mut self) -> &Vector2d {}
    }

    impl Mul for &SO2d {
    type Output = SO2d;
    #[ffi(link_name="sophus_rxx_SO2d_mul")]
    fn mul(self, rhs: Self) -> Self::Output {}
    }

    impl Mul for SO2d {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        (&self).mul(&rhs)
    }
    }

    impl Mul<&Vector2d> for &SO2d {
    type Output = Vector2d;
    #[ffi(link_name="sophus_rxx_SO2d_mul_point")]
    fn mul(self, rhs: &Self::Output) -> Self::Output {}
    }

    impl Mul<Vector2d> for SO2d {
    type Output = Vector2d;
    fn mul(self, rhs: Self::Output) -> Self::Output {
        (&self).mul(&rhs)
    }
    }

    impl Mul<&Vector3d> for &SO2d {
    type Output = Vector3d;
    #[ffi(link_name="sophus_rxx_SO2d_mul_hpoint")]
    fn mul(self, rhs: &Self::Output) -> Self::Output {}
    }

    impl Mul<Vector3d> for SO2d {
    type Output = Vector3d;
    fn mul(self, rhs: Self::Output) -> Self::Output {
        (&self).mul(&rhs)
    }
    }

);

#[cfg(test)]
mod tests {
    use crate::*;
    use std::f64::consts::PI;

    #[test]
    fn test_const() {
        assert_eq!(SO2d::dof(), 1);
        assert_eq!(SO2d::num_parameters(), 2);
        assert_eq!(SO2d::n(), 2);
        assert_eq!(SO2d::dim(), 2);
    }

    #[test]
    fn test_so2() {
        let mut s = SO2d::default();
        assert_eq!(s.adj(), 1.0);
        s.unit_complex_.data = [0.0, 2.0];
        let s1 = s.inverse();
        assert_eq!(s1.unit_complex_.data, [0.0, -1.0]);
        assert_eq!(s.log(), PI / 2.);

        let mut s = s;
        s.normalize();
        assert_eq!(s.unit_complex_.data, [0.0, 1.0]);

        assert_eq!(
            s.matrix(),
            Matrix::<f64, 2, 2> {
                data: [0.0, 1.0, -1.0, 0.0]
            }
        );

        assert_eq!((s * s1).unit_complex_.data, [1., 0.]);

        let p = Vector2d { data: [5., 10.] };

        assert_eq!((s * p).data, [-10., 5.0]);

        let p = Vector3d {
            data: [5., 10., 1.],
        };
        assert_eq!((s * p).data, [-10., 5.0, 1.]);
        assert_eq!(s.params().data, [0.0, 1.0]);

        s.set_complex(&Vector2d { data: [0.5, 0.5] });
        let gt = [1. / 2.0.sqrt(), 1. / 2.0.sqrt()];
        assert_eq!(s.params().data, gt);
        assert_eq!(s.unit_complex().data, gt);
    }

    // #[test]
    // fn it_works() {
    //     let result = 2 + 2;
    //     assert_eq!(result, 4);

    //     let mut v1 = SO2d {
    //         unit_complex_: Vector2d { data: [1., 2.] },
    //     };

    //     Pin::new(&mut v1).normalize();
    //     println!("v1 {:?}", v1);
    //     let v2 = v1.inverse();
    //     println!("v2 {:?}", v2);
    //     println!("v2 adj {:?}", v2.Adj());

    //     println!("v2 matrix {:?}", v2.matrix());

    //     let m1 = v1.matrix();
    //     let m2 = v2.matrix();
    //     let m3 = m1 * m2;
    //     println!("m3 matrix {:?} {:?} {:?}", m1, m2, m3);

    //     let v2 = SO2d {
    //         unit_complex_: Vector2d { data: [3., 4.] },
    //     };
    //     let mut v3 = v1 * v2;
    //     println!("v3 {:?}", v3);

    //     let p1 = Vector2d { data: [5., 6.] };
    //     let p2 = v3 * p1;
    //     println!("p2 {:?}", p2);

    //     let p3 = Vector3d { data: [7., 8., 9.] };
    //     let p4 = v3 * p3;
    //     println!("p4 {:?}", p4);

    //     println!("v3 params {:?}", v3.params());

    //     Pin::new(&mut v3).setComplex(&Vector2d { data: [5., 6.] });
    //     println!("v3 {:?}", v3);

    //     println!("v3 {:?}", v3.unit_complex());

    //     let v5 = SO2d_exp(&0.);
    //     println!("v5 {:?}", v5);

    //     println!("hat {:?}", SO2d_hat(&45.));
    //     println!("vee {:?}", SO2d_vee(&SO2d_hat(&45.)));

    //     let mut buf_v6 = [1., 2.];
    //     {
    //         let map_v6 = Map_SO2d_const::new(&buf_v6);
    //         println!("map {:?}", unsafe {
    //             std::slice::from_raw_parts(map_v6.unit_complex_.data, 2)
    //         });
    //     }

    //     {
    //         let mut mut_map_v6 = Map_SO2d::new(&mut buf_v6);
    //         println!("mut map {:?}", unsafe {
    //             std::slice::from_raw_parts(mut_map_v6.unit_complex_.data, 2)
    //         });
    //         Pin::new(&mut mut_map_v6).normalize();
    //         println!("mut map {:?}", unsafe {
    //             std::slice::from_raw_parts(mut_map_v6.unit_complex_.data, 2)
    //         });
    //         println!("buf {:?}", buf_v6);
    //     }

    //     // let mut v6 = SO3d_exp(&Vector3d { data: [1., 2., 3.] });
    //     // println!("v6 {:?}", v6);

    //     // println!("v6 adj {:?}", v6.Adj());

    //     // Pin::new(&mut v6).setQuaternion(&Quaterniond {
    //     //     x: 1.,
    //     //     y: 2.,
    //     //     z: 3.,
    //     //     w: 4.,
    //     // });
    //     // println!("v6 logtheta {:?}", v6.logAndTheta());

    //     // let mut v7 = RxSO2d_exp(&Vector2d { data: [1.57, 2.] });
    //     // println!("v7 {:?}", v7);

    //     // Pin::new(&mut v7).setScale(&10.);
    //     // println!("v7 {:?}", v7);

    //     // let mut v8 = SE2d {
    //     //     so2: SO2d {
    //     //         unit_complex: Vector2d { data: [1., 2.] },
    //     //     },
    //     //     translation: Vector2d { data: [3., 4.] },
    //     // };
    //     // println!("v8 {:?}", v8);
    //     // Pin::new(&mut v8).normalize();
    //     // println!("v8 inv {:?}", v8.inverse());
    //     // println!("v8 {:?}", v8);

    //     // let v8_so = Pin::new(&mut v8).mut_so2();
    //     // v8_so.setComplex(&Vector2d { data: [4., 2.] });
    //     // println!("v8 mut_so {:?}", v8.so2());
    //     // println!("v8  {:?}", v8);
    //     // println!("v8 trans {:?}", v8.translation());

    //     // Pin::new(&mut v8).mut_translation().data = [1., 1.];
    //     // println!("v8 trans {:?}", v8.translation());
    // }
}

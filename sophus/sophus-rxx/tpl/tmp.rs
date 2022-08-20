    // ("SO2d", "unit_complex", "Vector2d"),
    // ("SO3d", "unit_quaternion", "Quaterniond"),
    // ("RxSO2d", "complex", "Vector2d"),
    // ("RxSO3d", "quaternion", "Quaterniond"),
    // "SO3d": {
    //     "adj": "Matrix3d",
    //     "params": "Vector4d",
    //     "tangent": "Vector3d",
    //     "T": "Matrix3d",
    //     "muls": [
    //         ("SO3d", "mul"),
    //         ("Vector3d", "mul_point"),
    //         ("Vector4d", "mul_hpoint"),
    //     ],
    //     "fit": "Matrix3d",
    // },
    // "RxSO2d": {
    //     "adj": "Matrix2d",
    //     "params": "Vector2d",
    //     "tangent": "Vector2d",
    //     "T": "Matrix2d",
    //     "muls": [
    //         ("RxSO2d", "mul"),
    //         ("Vector2d", "mul_point"),
    //         ("Vector3d", "mul_hpoint")
    //     ],
    //     "rot": "Matrix2d",
    //     "so": "so2",
    //     "scale": True,
    // },
    // "RxSO3d": {
    //     "adj": "Matrix4d",
    //     "params": "Vector4d",
    //     "tangent": "Vector4d",
    //     "T": "Matrix3d",
    //     "muls": [
    //         ("RxSO3d", "mul"),
    //         ("Vector3d", "mul_point"),
    //         ("Vector4d", "mul_hpoint"),
    //     ],
    //     "rot": "Matrix3d",
    //     "so": "so3",
    //     "scale": True,
    // },
    // "SE2d": {
    //     "adj": "Matrix3d",
    //     "params": "Vector4d",
    //     "tangent": "Vector3d",
    //     "T": "Matrix3d",
    //     "muls": [
    //         ("SE2d", "mul"),
    //         ("Vector2d", "mul_point"),
    //         ("Vector3d", "mul_hpoint")
    //     ],
    //     "rot": "Matrix2d",
    //     "so": "so2",
    //     "trans": "Vector2d",
    //     "fit": "Matrix3d",
    // },


// {% set cls = "SE2d" -%}
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct {{cls}} {
//     pub so2: SO2d,
//     pub translation: Vector2d,
// }
// {{ impl_ExternType(cls) }}

// {% for i in ["SO3d", "RxSO3d"] -%}
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct {{i}}_TangentAndTheta {
//     tangent: {{objs[i]["tangent"]}},
//     theta: f64,
// }
// {{ impl_ExternType(i+"_TangentAndTheta") }}
// {% endfor %}


        // type SO3d_TangentAndTheta = super::SO3d_TangentAndTheta;
        // type RxSO3d_TangentAndTheta = super::RxSO3d_TangentAndTheta;

        // {% set i = "SO3d" -%}
        // fn angleX(self: &{{i}}) -> f64;
        // fn angleY(self: &{{i}}) -> f64;
        // fn angleZ(self: &{{i}}) -> f64;
        // fn logAndTheta(self: &{{i}}) -> {{i}}_TangentAndTheta;
        // fn normalize(self: Pin<&mut {{i}}>);
        // fn setQuaternion(self: Pin<&mut {{i}}>, v: &Quaterniond);
        // fn unit_quaternion(self: &{{i}}) -> &Quaterniond;
        // fn {{i}}_rotX(v: &f64) -> {{i}};
        // fn {{i}}_rotY(v: &f64) -> {{i}};
        // fn {{i}}_rotZ(v: &f64) -> {{i}};

        // {% set i = "RxSO2d" -%}
        // fn angle(self: &{{i}}) -> f64;
        // fn setComplex(self: Pin<&mut {{i}}>, complex: &Vector2d);
        // fn complex(self: &{{i}}) -> &Vector2d;

        // fn setSO2(self: Pin<&mut {{i}}>, v: &SO2d);
        // fn so2(self: &{{i}}) -> SO2d;

        // {% set i = "RxSO3d" -%}
        // fn logAndTheta(self: &{{i}}) -> {{i}}_TangentAndTheta;
        // fn setQuaternion(self: Pin<&mut {{i}}>, v: &Quaterniond);
        // fn quaternion(self: &{{i}}) -> &Quaterniond;

        // fn setSO3(self: Pin<&mut {{i}}>, v: &SO3d);
        // fn so3(self: &{{i}}) -> SO3d;

        // // [s] leftJacobian
        // // [s] leftJacbianInverse
        // // [s] expAndTheta

        // {% set i = "SE2d" -%}
        // fn normalize(self: Pin<&mut {{i}}>);
        // fn matrix2x3(self: &{{i}}) -> Matrix2x3d;

        // fn so2(self: &{{i}}) -> &SO2d;
        // #[rust_name = "mut_so2"]
        // fn so2(self: Pin<&mut {{i}}>)-> Pin<&mut SO2d>;
        // fn unit_complex(self: &{{i}}) -> &Vector2d;

        // fn {{i}}_rot(v: &f64) -> {{i}};
        // fn {{i}}_trans(v: &Vector2d) -> {{i}};

// pub use ffi::{SO3d_rotX, SO3d_rotY, SO3d_rotZ};
// pub use ffi::{SE2d_rot, SE2d_trans};

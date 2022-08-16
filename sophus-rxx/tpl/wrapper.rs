{% set objs = {
    "SO2d": {
        "adj": "f64",
        "params": "Vector2d",
        "tangent": "f64",
        "T": "Matrix2d",
        "muls": [
            ("SO2d", "mul"),
            ("Vector2d", "mul_point"),
            ("Vector3d", "mul_hpoint")
        ],
        "fit": "Matrix2d",
        "fields": [("unit_complex_", "Vector2d")],
    },
} -%}

use std::ops::Mul;
use eigen_rxx;
use eigen_rxx::{Vector2d, Vector3d, Vector4d, Map_Vector2d, Map_Vector2d_const};

{% for i, v in objs.items() -%}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct {{i}} {
    {% for f, t in v["fields"] -%}
    pub {{f}}: {{t}},
    {% endfor -%}
}
{{ impl_ExternType(i) }}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_{{i}}<'a> {
    {% for f, t in v["fields"] -%}
    pub {{f}}: Map_{{t}}<'a>,
    {% endfor -%}
    _pd: std::marker::PhantomData<&'a ()>,
}
{{ impl_ExternType("Map_"+i, True) }}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_{{i}}_const<'a> {
    {% for f, t in v["fields"] -%}
    pub {{f}}: Map_{{t}}_const<'a>,
    {% endfor -%}
    _pd: std::marker::PhantomData<&'a ()>,
}
{{ impl_ExternType("Map_"+i+"_const", True) }}

{% endfor -%}

#[cxx::bridge(namespace = "rxx")]
mod ffi {
    unsafe extern "C++" {
        include!("sophus_rxx/include/wrapper.hh");

        type Vector2d = eigen_rxx::Vector2d;
        type Vector3d = eigen_rxx::Vector3d;
        type Vector4d = eigen_rxx::Vector4d;
        type Matrix2d = eigen_rxx::Matrix2d;
        type Matrix3d = eigen_rxx::Matrix3d;
        type Matrix4d = eigen_rxx::Matrix4d;
        type Matrix2x3d = eigen_rxx::Matrix2x3d;

        type Map_Vector2d<'a> = eigen_rxx::Map_Vector2d<'a>;

        type Quaterniond = eigen_rxx::Quaterniond;

        {% for i, v in objs.items() %}
        type {{i}} = super::{{i}};

        {{ ffi_eigen_map_fixed(i) }}

        {% for cls in [i, "Map_"+i, "Map_"+i+"_const"] %}
        fn Adj(self: &{{cls}}) -> {{v["adj"]}};
        fn params(self: &{{cls}}) -> {{v["params"]}};
        fn log(self: &{{cls}}) -> {{v["tangent"]}};
        fn matrix(self: &{{cls}}) -> {{v["T"]}};
        fn inverse(self: &{{cls}}) -> {{i}};

        {% for j, meth in v["muls"] -%}
        #[rust_name = "{{cls}}_{{meth}}"]
        fn op_mul(self_: &{{cls}}, other: &{{j}}) -> {{j}};
        {% endfor -%}

        {% if "Map" not in cls %}
        fn {{cls}}_exp(theta: &{{v["tangent"]}}) -> {{cls}};
        fn {{cls}}_hat(theta: &{{v["tangent"]}}) -> {{v["T"]}};
        fn {{cls}}_vee(omega: &{{v["T"]}}) -> {{v["tangent"]}};

        {% if "fit" in v -%}
        fn {{i}}_fit(val: &{{v["fit"]}}) -> {{i}};
        {% endif %}

        {% endif %}

        {% if "rot" in v -%}
        fn rotationMatrix(self: &{{cls}}) -> {{v["rot"]}};
        fn setRotationMatrix(self: Pin<&mut {{cls}}>, v: &{{v["rot"]}});
        {% endif %}

        {% if "scale" in v -%}
        fn scale(self: &{{cls}}) -> f64;
        fn setScale(self: Pin<&mut {{cls}}>, v: &f64);
        {% endif %}

        {% if "rot" in v and "scale" in v-%}
        fn setScaledRotationMatrix(self: Pin<&mut {{cls}}>, v: &{{v["rot"]}});
        {% endif %}

        {% if "trans" in v -%}
        fn translation(self: &{{cls}}) -> &{{v["trans"]}};
        #[rust_name="mut_translation"]
        fn translation(self: Pin<&mut {{cls}}>) -> Pin<&mut {{v["trans"]}}>;
        {% endif %}

        // mul line
        // mul hyperplane
        // *=
        // Dx_this_mul_exp_x_at_0
        // Dx_log_this_inv_by_x_at_this
        // [s] Dx_exp_x
        // [s] Dx_exp_x_at_0
        // [s] Dxi_exp_x_matrix_at_0
        // [s] generator

        // [s] lieBracket
        // [s] sampleUniform

        // Map*

        {% endfor %} // clses
        {% endfor %} // objs

        {% set i = "SO2d" -%}
        {% for p in ["{}", "Map_{}"] %}
        fn normalize(self: Pin<&mut {{p.format(i)}}>);
        fn setComplex(self: Pin<&mut {{p.format(i)}}>, complex: &Vector2d);
        fn unit_complex(self: &{{p.format(i)}}) -> &{{p.format("Vector2d")}};
        {% endfor %}
    }
}

{% for i, v in objs.items() -%}
pub use ffi::{ {{i}}_exp, {{i}}_hat, {{i}}_vee};
{% if "fit" in v -%}
pub use ffi::{{i}}_fit;
{% endif %}
{% endfor %}

{% for i, v in objs.items() -%}

{{ impl_eigen_map_fixed(i) }}

{% for j, fn_name in v["muls"] -%}
impl Mul<{{j}}> for {{i}} {
    type Output = {{j}};

    fn mul(self, other: {{j}}) -> Self::Output {
        ffi::{{i}}_{{fn_name}}(&self, &other)
    }
}

impl Mul<&{{j}}> for &{{i}} {
    type Output = {{j}};

    fn mul(self, other: &{{j}}) -> Self::Output {
        ffi::{{i}}_{{fn_name}}(self, other)
    }
}
{% endfor %}
{% endfor %}

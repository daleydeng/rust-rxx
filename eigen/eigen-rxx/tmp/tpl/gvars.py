from easydict import EasyDict as edict

from_ctype = {
    "double": "f64",
    "float": "f32",
    "int": "i32",
}

postfix = {
    'double': 'd',
    'float': 'f',
    'int': 'i'
}

def link_name(name, cls=''):
    if not cls:
        return f"eigen_rxx${name}"
    return f"eigen_rxx${cls}${name}"

def build_Map_new(cls, tp):
    return {
        link_name(f"MapMut_{cls}_new"): {
            "c_sig": {
                "fn": f"MapMut_fixed_new<{cls}, {tp}>",
                "ret_type": f"Eigen::Map<{cls}>",
                "args": [
                    (f"{tp} *", "data"),
                ],
            },
            "rs_sig": {
                "fn": f"MapMut_{cls}_new<'a>",
                "ret_type": f"MapMut<'a, {cls}>",
                "fn_gt": "<'a>",
            },
        },
        link_name(f"Map_{cls}_new"): {
            "c_sig": {
                "fn": f"Map_fixed_new<{cls}, {tp}>",
                "ret_type": f"Eigen::Map<{cls} const>",
                "args": [
                    (f"{tp} const*", "data"),
                ],
            },
            "rs_sig": {
                "fn": f"Map_{cls}_new<'a>",
                "ret_type": f"Map<'a, {cls}>",
                "fn_gt": "<'a>",
            },
        },
    }

def get_cffi_config():
    fns = {}
    fixed_matrices = {}
    all_types = []

    for tp in ["double", "float", "int"]:
        t = postfix[tp]
        rs_tp = from_ctype[tp]
        for R, C in [(2, 2), (3, 3), (4, 4), (2, 3), (2, 1), (3, 1), (4, 1)]:
            kws = {}
            if C == 1:
                cls = f'Vector{R}{t}'
                rs_own = f'na::Vector{R}'
                rs_ref = f'na::VectorSlice{R}'
                rs_mut_ref = f'na::VectorSliceMut{R}'

            elif R == C:
                cls = f'Matrix{R}{t}'
                rs_own = f'na::Matrix{R}'
                rs_ref = f'na::MatrixSlice{R}'
                rs_mut_ref = f'na::MatrixSliceMut{R}'

            else:
                cls = f'Matrix{R}x{C}{t}'
                rs_own = f'na::Matrix{R}x{C}'
                rs_ref = f'na::MatrixSlice{R}x{C}'
                rs_mut_ref = f'na::MatrixSliceMut{R}x{C}'

                kws.update({
                    'mul': True,
                })

            size = R * C
            fixed_matrices.update({
                cls: {
                    'shape': (R, C),
                    "size": size,
                    'tp': rs_tp,
                    'rs_own': rs_own,
                    'rs_ref': rs_ref,
                    'rs_mut_ref': rs_mut_ref,
                    **kws
                }
            })

            fns.update(build_Map_new(cls, tp))

            all_types.append((cls, cls))
            all_types.append((f"MapMut_{cls}", f"MapMut<'a, {cls}>"))
            all_types.append((f"Map_{cls}", f"Map<'a, {cls}>"))

    quats = {}
    aas = {}
    for tp in ["double", "float"]:
        t = postfix[tp]
        cls = f'Quaternion{t}'
        all_types.append((cls, cls))

        rs_tp = from_ctype[tp]

        quats.update({
            cls: {
                "tp": rs_tp,
                "rs_own": "na::Quaternion",
            }
        })

        fns.update({
            link_name("normalized", cls): {
                "c_sig": {
                    "cls": cls,
                    "fn": "&$C::normalized",
                    "ret_type": "$C",
                },
                "rs_sig": {}
            },
            link_name("normalize", cls): {
                "c_sig": {
                    "cls": cls,
                    "is_const": False,
                    "fn": "&$C::normalize",
                },
                "rs_sig": {}
            },
            link_name("inverse", cls): {
                "c_sig": {
                    "cls": cls,
                    "fn": "&$C::inverse",
                    "ret_type": "$C",
                },
                "rs_sig": {}
            },
            link_name("mul", cls): {
                "c_sig": {
                    "cls": cls,
                    "fn": "op_mul",
                    "ret_type": "$C",
                    "args": [
                        ("$C const&", "other"),
                    ],
                },
                "rs_sig": {
                    "fn": "mul",
                },
            },
            link_name("toRotationMatrix", cls): {
                "c_sig": {
                    "cls": cls,
                    "fn": "&$C::toRotationMatrix",
                    "ret_type": f"Matrix3{t}",
                }
            },
        })

        fns.update(build_Map_new(cls, tp))

        quat_cls = cls
        cls = f'AngleAxis{t}'
        all_types.append((cls, cls))

        aas.update({
            cls: {
                'tp': rs_tp,
                'axis': f"Vector3{t}",
            }
        })

        fns.update({
            link_name("inverse", cls): {
                "c_sig": {
                    "cls": cls,
                    "fn": "&$C::inverse",
                    "ret_type": "$C",
                },
                "rs_sig": {}
            },
            link_name("mul", cls): {
                "c_sig": {
                    "cls": cls,
                    "fn": "op_mul",
                    "ret_type": quat_cls,
                    "args": [
                        ("$C const&", "other"),
                    ],
                },
                "rs_sig": {
                    "fn": "mul",
                },
            },
            link_name("toRotationMatrix", cls): {
                "c_sig": {
                    "cls": cls,
                    "fn": "&$C::toRotationMatrix",
                    "ret_type": f"Matrix3{t}",
                }
            },
        })

        # AngleAxis doesn't support Map

    for c_tp, rs_tp in all_types:
        if 'AngleAxis' in c_tp: # does not support print
            continue

        rs_sig = {
            'cls': rs_tp,
            'fn': 'to_cxx_string',
            'ret_type': "rxx::UniquePtr<rxx::CxxString>",
            'args': [],
        }
        if "'a" in rs_tp:
            rs_sig.update({
                'gt': "<'a>",
            })

        fns.update({
            f'to_string_{c_tp}': {
                'c_sig': {
                    'fn': f'eigen_rxx::to_string<{c_tp}>',
                    "ret_type": "std::unique_ptr<std::string>",
                    "args": [
                        (f"{c_tp} const&", "v"),
                    ],
                },
                "rs_sig": rs_sig
            },
        })


    tm = from_ctype.copy()
    tm.update(dict(all_types))

    return edict({
        "type_mapping": tm,
        "cffi_fixed_matrices": fixed_matrices,
        "cffi_quats": quats,
        "cffi_aas": aas,
        "cffi_fns": fns,
    })

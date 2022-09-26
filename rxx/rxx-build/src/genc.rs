use handlebars::Handlebars;
use serde_json::json;

static TPL_RET_OBJECT_FN: &str = r#"
extern "C" void {{name}}({{{decl_link_args}}}{{{ret_type}}} *__ret) noexcept {
    {{{ret_type}}} (*__func)({{{decl_args}}}) = {{{c_fn}}};
    new (__ret) ({{{ret_type}}})(__func({{{call_args}}}));
}
"#;

static TPL_RET_ATOMIC_FN: &str = r#"
extern "C" {{{ret_type}}} {{name}}({{{decl_link_args}}}) noexcept {
    {{{ret_type}}} (*__func)({{{decl_args}}}) = {{{c_fn}}};
    return __func({{{call_args}}});
}
"#;

static TPL_VOID_FN: &str = r#"
extern "C" void {{name}}({{{decl_link_args}}}) noexcept {
    void (*__func)({{{decl_args}}}) = {{{c_fn}}};
    __func({{{call_args}}});
}
"#;

static TPL_RET_OBJECT_MEMFN: &str = r#"
extern "C" void {{name}}({{{cls}}} const &self{{{decl_link_args}}}{{{ret_type}}} *__ret) noexcept {
    {{{ret_type}}} ({{{cls}}}::*__func)({{{decl_args}}}) const = {{{c_fn}}};
    new (__ret) {{{ret_type}}}((self.*__func)({{{call_args}}}));
}
"#;

static TPL_RET_ATOMIC_MEMFN: &str = r#"
extern "C" {{{ret_type}}} {{name}}({{{cls}}} const &self{{{decl_link_args}}}) noexcept {
    {{{ret_type}}} ({{{cls}}}::*__func)({{{decl_args}}}) const = {{{c_fn}}};
    return (self.*__func)({{{call_args}}});
}
"#;

static TPL_VOID_MEMFN: &str = r#"
extern "C" void {{name}}({{{cls}}} const &self{{{decl_link_args}}}) noexcept {
    void ({{{cls}}}::*__func)({{{decl_args}}}) const = {{{c_fn}}};
    (self.*__func)({{{call_args}}});
}
"#;

static TPL_RET_OBJECT_MEMFN_MUT: &str = r#"
extern "C" void {{name}}({{{cls}}} &self{{{decl_link_args}}}{{{ret_type}}} *__ret) noexcept {
    {{{ret_type}}} ({{{cls}}}::*__func)({{{decl_args}}}) = {{{c_fn}}};
    new (__ret) {{{ret_type}}}((self.*__func)({{{call_args}}}));
}
"#;

static TPL_RET_ATOMIC_MEMFN_MUT: &str = r#"
extern "C" {{{ret_type}}} {{name}}({{{cls}}} &self{{{decl_link_args}}}) noexcept {
    {{{ret_type}}} ({{{cls}}}::*__func)({{{decl_args}}}) = {{{c_fn}}};
    return (self.*__func)({{{call_args}}});
}
"#;

static TPL_VOID_MEMFN_MUT: &str = r#"
extern "C" void {{name}}({{{cls}}} &self{{{decl_link_args}}}) noexcept {
    void ({{{cls}}}::*__func)({{{decl_args}}}) = {{{c_fn}}};
    (self.*__func)({{{call_args}}});
}
"#;

static TPL_GET_OBJECT_VAL: &str = r#"
extern "C" void {{name}}({{ret_type}} *ret) noexcept {
    *ret = {{{c_tp}}};
}
"#;

static TPL_GET_ATOMIC_VAL: &str = r#"
extern "C" {{ret_type}} {{name}}() noexcept {
    return {{{c_tp}}};
}
"#;

static TPL_UNIQUE_PTR: &str = r#"
extern "C" void {{name}}_destroy({{{c_tp}}} &self) noexcept {
    rxx::destroy(self);
}
"#;

static TPL_SHARED_PTR: &str = r#"
extern "C" void {{name}}_destroy({{{c_tp}}} &self) noexcept {
    rxx::destroy(self);
}

extern "C" void {{name}}_clone(const {{{c_tp}}} &self, {{{c_tp}}} *out) noexcept {
    rxx::shared_ptr_clone(self, out);
}
"#;

static TPL_WEAK_PTR: &str = r#"
extern "C" void {{name}}_destroy({{{c_tp}}} &self) noexcept {
    rxx::destroy(self);
}

extern "C" void {{name}}_clone(const {{{c_tp}}} &self, {{{c_tp}}} *out) noexcept {
    rxx::weak_ptr_clone(self, out);
}

extern "C" void {{name}}_upgrade(const {{{c_tp}}} &self, {{{c_shared_tp}}} *out) {
    rxx::weak_ptr_upgrade(self, out);
}

extern "C"  void {{name}}_downgrade(const {{{c_shared_tp}}} &self, {{{c_tp}}} *out) {
    rxx::weak_ptr_downgrade(self, out);
}
"#;

static TPL_VECTOR: &str = r#"
extern "C" void {{name}}_destroy(const {{{c_tp}}} &self) {
    rxx::destroy(self);
}

extern "C" std::size_t {{name}}_size(const {{{c_tp}}} &self) {
    return rxx::vector_size(self);
}

extern "C" const {{{c_item_tp}}}& {{name}}_get(const {{{c_tp}}} &self, size_t pos) {
    return rxx::vector_get(self, pos);
}

extern "C" {{{c_item_tp}}}& {{name}}_get_mut({{{c_tp}}} &self, size_t pos) {
    return rxx::vector_get_mut(self, pos);
}

extern "C" void {{name}}_push_back({{{c_tp}}} &self, {{{c_item_tp}}} &val) {
    return rxx::vector_push_back(self, val);
}

extern "C" void {{name}}_pop_back({{{c_tp}}} &self, {{{c_item_tp}}} *out) {
    rxx::vector_pop_back(self, out);
}
"#;

lazy_static::lazy_static! {
    static ref HANDLEBARS: Handlebars<'static> = {
        let mut hb = Handlebars::new();
        hb.set_strict_mode(true);

        for (k, v) in &[
            ("tpl_ret_object_fn", TPL_RET_OBJECT_FN),
            ("tpl_ret_atomic_fn", TPL_RET_ATOMIC_FN),
            ("tpl_void_fn", TPL_VOID_FN),
            ("tpl_ret_object_memfn", TPL_RET_OBJECT_MEMFN),
            ("tpl_ret_atomic_memfn", TPL_RET_ATOMIC_MEMFN),
            ("tpl_void_memfn", TPL_VOID_MEMFN),
            ("tpl_ret_object_memfn_mut", TPL_RET_OBJECT_MEMFN_MUT),
            ("tpl_ret_atomic_memfn_mut", TPL_RET_ATOMIC_MEMFN_MUT),
            ("tpl_void_memfn_mut", TPL_VOID_MEMFN_MUT),
        ("tpl_get_object_val", TPL_GET_OBJECT_VAL),
        ("tpl_get_atomic_val", TPL_GET_ATOMIC_VAL),
            ("tpl_unique_ptr", TPL_UNIQUE_PTR),
            ("tpl_shared_ptr", TPL_SHARED_PTR),
            ("tpl_weak_ptr", TPL_WEAK_PTR),
            ("tpl_vector", TPL_VECTOR),
        ] {
            hb.register_template_string(k, v.trim_start()).unwrap();
        }

        hb
    };
}

#[derive(Default, Clone, Copy)]
pub enum ReturnType<'a> {
    #[default]
    None,
    Object(&'a str),
    Atomic(&'a str),
}

impl ReturnType<'_> {
    pub fn is_none(&self) -> bool {
        matches!(*self, Self::None)
    }

    pub fn is_object(&self) -> bool {
        matches!(*self, Self::Object(_))
    }

    pub fn is_atomic(&self) -> bool {
        matches!(*self, Self::Atomic(_))
    }
}

#[derive(Default)]
pub struct FnSig<'a> {
    pub cls: Option<&'a str>,
    pub is_mut: bool,
    pub c_fn: &'a str,

    pub ret_type: ReturnType<'a>,
    pub args: &'a [(&'a str, &'a str)],
}

pub fn genc_fn(link_name: &str, fn_sig: FnSig) -> String {
    let s_decl_args = fn_sig
        .args
        .iter()
        .map(|(tp, val)| format!("{tp} {val}"))
        .collect::<Vec<_>>()
        .join(", ");

    let s_call_args = fn_sig
        .args
        .iter()
        .map(|(_, val)| val.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    let mut s_decl_link_args = s_decl_args.clone();

    if !s_decl_link_args.is_empty() {
        if fn_sig.ret_type.is_object() {
            s_decl_link_args += ", ";
        }
        if fn_sig.cls.is_some() {
            s_decl_link_args.insert_str(0, ", ");
        }
    } else if fn_sig.ret_type.is_object() && fn_sig.cls.is_some() {
        s_decl_link_args = ", ".to_string();
    }

    match fn_sig.cls {
        None => {
            let (ret_type, tpl_name) = match fn_sig.ret_type {
                ReturnType::None => ("", "tpl_void_fn"),
                ReturnType::Object(rt) => (rt, "tpl_ret_object_fn"),
                ReturnType::Atomic(rt) => (rt, "tpl_ret_atomic_fn"),
            };

            HANDLEBARS
                .render(
                    tpl_name,
                    &json!({
                    "name": link_name,
                    "c_fn": fn_sig.c_fn,
                    "ret_type": ret_type,
                    "decl_link_args": s_decl_link_args,
                    "decl_args": s_decl_args,
                    "call_args": s_call_args,

                    }),
                )
                .unwrap()
        }

        Some(cls) => {
            let (ret_type, tpl_name) = match (fn_sig.ret_type, fn_sig.is_mut) {
                (ReturnType::None, false) => ("", "tpl_void_memfn"),
                (ReturnType::None, true) => ("", "tpl_void_memfn_mut"),
                (ReturnType::Object(rt), false) => (rt, "tpl_ret_object_memfn"),
                (ReturnType::Object(rt), true) => (rt, "tpl_ret_object_memfn_mut"),
                (ReturnType::Atomic(rt), false) => (rt, "tpl_ret_atomic_memfn"),
                (ReturnType::Atomic(rt), true) => (rt, "tpl_ret_atomic_memfn_mut"),
            };

            let fn_name = fn_sig.c_fn.replace("$C", cls);
            let ret_type = ret_type.replace("$C", cls);
            let s_decl_link_args = s_decl_link_args.replace("$C", cls);
            let s_decl_args = s_decl_args.replace("$C", cls);
            let s_call_args = s_call_args.replace("$C", cls);

            HANDLEBARS
                .render(
                    tpl_name,
                    &json!({
                    "cls": cls,
                    "name": link_name,
                    "c_fn": fn_name,
                    "ret_type": ret_type,
                    "decl_link_args": s_decl_link_args,
                    "decl_args": s_decl_args,
                    "call_args": s_call_args,

                    }),
                )
                .unwrap()
        }
    }
}

pub fn genc_delete(link_name: &str, c_tp: &str) -> String {
    format!(
        r#"
extern "C" void {link_name}({c_tp} *ptr) noexcept {{
    delete ptr;
}}
"#
    )
    .trim_start()
    .into()
}

pub fn genc_destroy(link_name: &str, c_tp: &str) -> String {
    format!(
        r#"
extern "C" void {link_name}({c_tp} &self) noexcept {{
    self.~{c_tp}();
}}
"#
    )
    .trim_start()
    .into()
}

pub fn genc_get_val<'a>(link_name: &str, ret_type: ReturnType<'a>, c_tp: &str) -> String {
    match ret_type {
        ReturnType::Object(v) => HANDLEBARS
            .render(
                "tpl_get_object_val",
                &json!({
                    "name": link_name,
                    "ret_type": v,
                    "c_tp": c_tp,
                }),
            )
            .unwrap(),
        ReturnType::Atomic(v) => HANDLEBARS
            .render(
                "tpl_get_atomic_val",
                &json!({
                    "name": link_name,
                    "ret_type": v,
                    "c_tp": c_tp,
                }),
            )
            .unwrap(),
        ReturnType::None => panic!("return type cannot be None"),
    }
}

pub fn genc_unique_ptr(link_name: &str, c_tp: &str) -> String {
    HANDLEBARS
        .render(
            "tpl_unique_ptr",
            &json!({
            "name": link_name,
            "c_tp": c_tp,
            }),
        )
        .unwrap()
}

pub fn genc_shared_ptr(link_name: &str, c_tp: &str) -> String {
    HANDLEBARS
        .render(
            "tpl_shared_ptr",
            &json!({
            "name": link_name,
            "c_tp": c_tp,
            }),
        )
        .unwrap()
}

pub fn genc_weak_ptr(link_name: &str, c_tp: &str, c_shared_tp: &str) -> String {
    HANDLEBARS
        .render(
            "tpl_weak_ptr",
            &json!({
            "name": link_name,
            "c_tp": c_tp,
            "c_shared_tp": c_shared_tp,
            }),
        )
        .unwrap()
}

pub fn genc_vector(link_name: &str, c_tp: &str, c_item_tp: &str) -> String {
    HANDLEBARS
        .render(
            "tpl_vector",
            &json!({
            "name": link_name,
            "c_tp": c_tp,
            "c_item_tp": c_item_tp,
            }),
        )
        .unwrap()
}

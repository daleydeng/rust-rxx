#include "wrapper.hh"

namespace eigen_rxx {
extern "C" {

{% for k, o in cffi_fns.items() -%}
{{ cffi_genc_fn(k, **o) }}
{% endfor %}

} // extern "C"
} // namespace

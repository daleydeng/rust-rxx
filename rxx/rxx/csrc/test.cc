#include <rxx/wrapper.hh>
#include "test.hh"

using namespace rxx;

std::vector<int64_t> dummy_cpp_new_vector_i64(int v) {
  return {v};
}

void dummy_cpp_add_vector_i64(std::vector<int64_t>& v, int n) {
  v[0] += n;
}

int64_t dummy_cpp_addret_vector_i64(std::vector<int64_t>& v, int n) {
  v[0] += n;
  return v[0];
}

int64_t dummy_cpp_get_vector_i64(std::vector<int64_t>const& v) {
  return v[0];
}

void dummy_cpp_getvoid_vector_i64(std::vector<int64_t>const& v, int a) {
  a += v[0];
}

int64_t const & dummy_cpp_getref_vector_i64(std::vector<int64_t> const &v, int idx) {
  return v[idx];
}

extern "C" {

void rxx_dummy_new_unique_i64(int64_t v, std::unique_ptr<int64_t> *out) {
  new (out) std::unique_ptr<int64_t>(new int64_t(v));
}

void rxx_dummy_new_shared_i64(int64_t v, std::shared_ptr<int64_t> *out) {
  new (out) std::shared_ptr<int64_t>(new int64_t(v));
}

void rxx_dummy_new_vector_i64(const int64_t *data, size_t len, std::vector<int64_t> *out) {
  new (out) std::vector<int64_t>;
  out->assign(data, data+len);
}

void rxx_dummy_new_unique_string(std::unique_ptr<std::string> *out) {
  new (out) std::unique_ptr<std::string>(new std::string("test"));
}

void rxx_dummy_new_shared_string(std::shared_ptr<std::string> *out) {
  new (out) std::shared_ptr<std::string>(new std::string("test"));
}

int64_t *rxx_dummy_i64_new() {
  return new int64_t(5);
}

} // extern "C"

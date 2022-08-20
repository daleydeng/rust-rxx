#include "eigen_rxx/include/wrapper.hh"
#include <cstddef>
#include <cstdint>
#include <new>
#include <type_traits>

namespace rust {
inline namespace cxxbridge1 {
// #include "rust/cxx.h"

#ifndef CXXBRIDGE1_RELOCATABLE
#define CXXBRIDGE1_RELOCATABLE
namespace detail {
template <typename... Ts>
struct make_void {
  using type = void;
};

template <typename... Ts>
using void_t = typename make_void<Ts...>::type;

template <typename Void, template <typename...> class, typename...>
struct detect : std::false_type {};
template <template <typename...> class T, typename... A>
struct detect<void_t<T<A...>>, T, A...> : std::true_type {};

template <template <typename...> class T, typename... A>
using is_detected = detect<void, T, A...>;

template <typename T>
using detect_IsRelocatable = typename T::IsRelocatable;

template <typename T>
struct get_IsRelocatable
    : std::is_same<typename T::IsRelocatable, std::true_type> {};
} // namespace detail

template <typename T>
struct IsRelocatable
    : std::conditional<
          detail::is_detected<detail::detect_IsRelocatable, T>::value,
          detail::get_IsRelocatable<T>,
          std::integral_constant<
              bool, std::is_trivially_move_constructible<T>::value &&
                        std::is_trivially_destructible<T>::value>>::type {};
#endif // CXXBRIDGE1_RELOCATABLE
} // namespace cxxbridge1
} // namespace rust

static_assert(
    ::rust::IsRelocatable<::rxx::Matrix3d>::value,
    "type rxx::Matrix3d should be trivially move constructible and trivially destructible in C++ to be used as a return value of `toRotationMatrix` in Rust");
static_assert(
    ::rust::IsRelocatable<::rxx::MatrixXd>::value,
    "type rxx::MatrixXd should be trivially move constructible and trivially destructible in C++ to be used as a return value of `MatrixXd_new`, `MatrixXd_clone` in Rust");
static_assert(
    ::rust::IsRelocatable<::rxx::VectorXd>::value,
    "type rxx::VectorXd should be trivially move constructible and trivially destructible in C++ to be used as a return value of `VectorXd_new`, `VectorXd_clone` in Rust");
static_assert(
    ::rust::IsRelocatable<::rxx::MapMut_Matrix2d>::value,
    "type rxx::MapMut_Matrix2d should be trivially move constructible and trivially destructible in C++ to be used as a return value of `MapMut_Matrix2d_new` in Rust");
static_assert(
    ::rust::IsRelocatable<::rxx::Map_Matrix2d>::value,
    "type rxx::Map_Matrix2d should be trivially move constructible and trivially destructible in C++ to be used as a return value of `Map_Matrix2d_new` in Rust");
static_assert(
    ::rust::IsRelocatable<::rxx::MapMut_Matrix2i>::value,
    "type rxx::MapMut_Matrix2i should be trivially move constructible and trivially destructible in C++ to be used as a return value of `MapMut_Matrix2i_new` in Rust");
static_assert(
    ::rust::IsRelocatable<::rxx::Map_Matrix2i>::value,
    "type rxx::Map_Matrix2i should be trivially move constructible and trivially destructible in C++ to be used as a return value of `Map_Matrix2i_new` in Rust");
static_assert(
    ::rust::IsRelocatable<::rxx::MapMut_Matrix3d>::value,
    "type rxx::MapMut_Matrix3d should be trivially move constructible and trivially destructible in C++ to be used as a return value of `MapMut_Matrix3d_new` in Rust");
static_assert(
    ::rust::IsRelocatable<::rxx::Map_Matrix3d>::value,
    "type rxx::Map_Matrix3d should be trivially move constructible and trivially destructible in C++ to be used as a return value of `Map_Matrix3d_new` in Rust");
static_assert(
    ::rust::IsRelocatable<::rxx::MapMut_Matrix4d>::value,
    "type rxx::MapMut_Matrix4d should be trivially move constructible and trivially destructible in C++ to be used as a return value of `MapMut_Matrix4d_new` in Rust");
static_assert(
    ::rust::IsRelocatable<::rxx::Map_Matrix4d>::value,
    "type rxx::Map_Matrix4d should be trivially move constructible and trivially destructible in C++ to be used as a return value of `Map_Matrix4d_new` in Rust");
static_assert(
    ::rust::IsRelocatable<::rxx::MapMut_Matrix2x3d>::value,
    "type rxx::MapMut_Matrix2x3d should be trivially move constructible and trivially destructible in C++ to be used as a return value of `MapMut_Matrix2x3d_new` in Rust");
static_assert(
    ::rust::IsRelocatable<::rxx::Map_Matrix2x3d>::value,
    "type rxx::Map_Matrix2x3d should be trivially move constructible and trivially destructible in C++ to be used as a return value of `Map_Matrix2x3d_new` in Rust");
static_assert(
    ::rust::IsRelocatable<::rxx::MapMut_MatrixXd>::value,
    "type rxx::MapMut_MatrixXd should be trivially move constructible and trivially destructible in C++ to be used as a return value of `MapMut_MatrixXd_new` in Rust");
static_assert(
    ::rust::IsRelocatable<::rxx::Map_MatrixXd>::value,
    "type rxx::Map_MatrixXd should be trivially move constructible and trivially destructible in C++ to be used as a return value of `Map_MatrixXd_new` in Rust");
static_assert(
    ::rust::IsRelocatable<::rxx::MapMut_MatrixXd_stride>::value,
    "type rxx::MapMut_MatrixXd_stride should be trivially move constructible and trivially destructible in C++ to be used as a return value of `MapMut_MatrixXd_stride_new` in Rust");
static_assert(
    ::rust::IsRelocatable<::rxx::Map_MatrixXd_stride>::value,
    "type rxx::Map_MatrixXd_stride should be trivially move constructible and trivially destructible in C++ to be used as a return value of `Map_MatrixXd_stride_new` in Rust");
static_assert(
    ::rust::IsRelocatable<::rxx::MapMut_Vector2d>::value,
    "type rxx::MapMut_Vector2d should be trivially move constructible and trivially destructible in C++ to be used as a return value of `MapMut_Vector2d_new` in Rust");
static_assert(
    ::rust::IsRelocatable<::rxx::Map_Vector2d>::value,
    "type rxx::Map_Vector2d should be trivially move constructible and trivially destructible in C++ to be used as a return value of `Map_Vector2d_new` in Rust");
static_assert(
    ::rust::IsRelocatable<::rxx::MapMut_Vector3d>::value,
    "type rxx::MapMut_Vector3d should be trivially move constructible and trivially destructible in C++ to be used as a return value of `MapMut_Vector3d_new` in Rust");
static_assert(
    ::rust::IsRelocatable<::rxx::Map_Vector3d>::value,
    "type rxx::Map_Vector3d should be trivially move constructible and trivially destructible in C++ to be used as a return value of `Map_Vector3d_new` in Rust");
static_assert(
    ::rust::IsRelocatable<::rxx::MapMut_Vector4d>::value,
    "type rxx::MapMut_Vector4d should be trivially move constructible and trivially destructible in C++ to be used as a return value of `MapMut_Vector4d_new` in Rust");
static_assert(
    ::rust::IsRelocatable<::rxx::Map_Vector4d>::value,
    "type rxx::Map_Vector4d should be trivially move constructible and trivially destructible in C++ to be used as a return value of `Map_Vector4d_new` in Rust");
static_assert(
    ::rust::IsRelocatable<::rxx::MapMut_VectorXd>::value,
    "type rxx::MapMut_VectorXd should be trivially move constructible and trivially destructible in C++ to be used as a return value of `MapMut_VectorXd_new` in Rust");
static_assert(
    ::rust::IsRelocatable<::rxx::Map_VectorXd>::value,
    "type rxx::Map_VectorXd should be trivially move constructible and trivially destructible in C++ to be used as a return value of `Map_VectorXd_new` in Rust");
static_assert(
    ::rust::IsRelocatable<::rxx::MapMut_VectorXd_stride>::value,
    "type rxx::MapMut_VectorXd_stride should be trivially move constructible and trivially destructible in C++ to be used as a return value of `MapMut_VectorXd_stride_new` in Rust");
static_assert(
    ::rust::IsRelocatable<::rxx::Map_VectorXd_stride>::value,
    "type rxx::Map_VectorXd_stride should be trivially move constructible and trivially destructible in C++ to be used as a return value of `Map_VectorXd_stride_new` in Rust");
static_assert(
    ::rust::IsRelocatable<::rxx::Quaterniond>::value,
    "type rxx::Quaterniond should be trivially move constructible and trivially destructible in C++ to be used as a return value of `normalized`, `inverse`, `Quaterniond_mul` in Rust");
static_assert(
    ::rust::IsRelocatable<::rxx::MapMut_Quaterniond>::value,
    "type rxx::MapMut_Quaterniond should be trivially move constructible and trivially destructible in C++ to be used as a return value of `MapMut_Quaterniond_new` in Rust");
static_assert(
    ::rust::IsRelocatable<::rxx::Map_Quaterniond>::value,
    "type rxx::Map_Quaterniond should be trivially move constructible and trivially destructible in C++ to be used as a return value of `Map_Quaterniond_new` in Rust");
static_assert(
    ::rust::IsRelocatable<::rxx::AngleAxisd>::value,
    "type rxx::AngleAxisd should be trivially move constructible and trivially destructible in C++ to be used as a return value of `inverse` in Rust");

namespace rxx {
extern "C" {
void rxx$cxxbridge1$MapMut_Matrix2d_new(double *data, ::rxx::MapMut_Matrix2d *return$) noexcept {
  ::rxx::MapMut_Matrix2d (*MapMut_Matrix2d_new$)(double *) = ::rxx::MapMut_fixed_new;
  new (return$) ::rxx::MapMut_Matrix2d(MapMut_Matrix2d_new$(data));
}

void rxx$cxxbridge1$Map_Matrix2d_new(const double *data, ::rxx::Map_Matrix2d *return$) noexcept {
  ::rxx::Map_Matrix2d (*Map_Matrix2d_new$)(const double *) = ::rxx::Map_fixed_new;
  new (return$) ::rxx::Map_Matrix2d(Map_Matrix2d_new$(data));
}

void rxx$cxxbridge1$MapMut_Matrix2i_new(::std::int32_t *data, ::rxx::MapMut_Matrix2i *return$) noexcept {
  ::rxx::MapMut_Matrix2i (*MapMut_Matrix2i_new$)(::std::int32_t *) = ::rxx::MapMut_fixed_new;
  new (return$) ::rxx::MapMut_Matrix2i(MapMut_Matrix2i_new$(data));
}

void rxx$cxxbridge1$Map_Matrix2i_new(const ::std::int32_t *data, ::rxx::Map_Matrix2i *return$) noexcept {
  ::rxx::Map_Matrix2i (*Map_Matrix2i_new$)(const ::std::int32_t *) = ::rxx::Map_fixed_new;
  new (return$) ::rxx::Map_Matrix2i(Map_Matrix2i_new$(data));
}

void rxx$cxxbridge1$MapMut_Matrix3d_new(double *data, ::rxx::MapMut_Matrix3d *return$) noexcept {
  ::rxx::MapMut_Matrix3d (*MapMut_Matrix3d_new$)(double *) = ::rxx::MapMut_fixed_new;
  new (return$) ::rxx::MapMut_Matrix3d(MapMut_Matrix3d_new$(data));
}

void rxx$cxxbridge1$Map_Matrix3d_new(const double *data, ::rxx::Map_Matrix3d *return$) noexcept {
  ::rxx::Map_Matrix3d (*Map_Matrix3d_new$)(const double *) = ::rxx::Map_fixed_new;
  new (return$) ::rxx::Map_Matrix3d(Map_Matrix3d_new$(data));
}

void rxx$cxxbridge1$MapMut_Matrix4d_new(double *data, ::rxx::MapMut_Matrix4d *return$) noexcept {
  ::rxx::MapMut_Matrix4d (*MapMut_Matrix4d_new$)(double *) = ::rxx::MapMut_fixed_new;
  new (return$) ::rxx::MapMut_Matrix4d(MapMut_Matrix4d_new$(data));
}

void rxx$cxxbridge1$Map_Matrix4d_new(const double *data, ::rxx::Map_Matrix4d *return$) noexcept {
  ::rxx::Map_Matrix4d (*Map_Matrix4d_new$)(const double *) = ::rxx::Map_fixed_new;
  new (return$) ::rxx::Map_Matrix4d(Map_Matrix4d_new$(data));
}

void rxx$cxxbridge1$MapMut_Matrix2x3d_new(double *data, ::rxx::MapMut_Matrix2x3d *return$) noexcept {
  ::rxx::MapMut_Matrix2x3d (*MapMut_Matrix2x3d_new$)(double *) = ::rxx::MapMut_fixed_new;
  new (return$) ::rxx::MapMut_Matrix2x3d(MapMut_Matrix2x3d_new$(data));
}

void rxx$cxxbridge1$Map_Matrix2x3d_new(const double *data, ::rxx::Map_Matrix2x3d *return$) noexcept {
  ::rxx::Map_Matrix2x3d (*Map_Matrix2x3d_new$)(const double *) = ::rxx::Map_fixed_new;
  new (return$) ::rxx::Map_Matrix2x3d(Map_Matrix2x3d_new$(data));
}

void rxx$cxxbridge1$MatrixXd_new(::std::size_t rows, ::std::size_t cols, ::rxx::MatrixXd *return$) noexcept {
  ::rxx::MatrixXd (*MatrixXd_new$)(::std::size_t, ::std::size_t) = ::rxx::MatrixXd_new;
  new (return$) ::rxx::MatrixXd(MatrixXd_new$(rows, cols));
}

void rxx$cxxbridge1$MatrixXd_clone(const ::rxx::MatrixXd &v, ::rxx::MatrixXd *return$) noexcept {
  ::rxx::MatrixXd (*MatrixXd_clone$)(const ::rxx::MatrixXd &) = ::rxx::MatrixXd_clone;
  new (return$) ::rxx::MatrixXd(MatrixXd_clone$(v));
}

void rxx$cxxbridge1$MapMut_MatrixXd_new(double *data, ::std::size_t rows, ::std::size_t cols, ::rxx::MapMut_MatrixXd *return$) noexcept {
  ::rxx::MapMut_MatrixXd (*MapMut_MatrixXd_new$)(double *, ::std::size_t, ::std::size_t) = ::rxx::MapMut_MatrixXd_new;
  new (return$) ::rxx::MapMut_MatrixXd(MapMut_MatrixXd_new$(data, rows, cols));
}

void rxx$cxxbridge1$Map_MatrixXd_new(const double *data, ::std::size_t rows, ::std::size_t cols, ::rxx::Map_MatrixXd *return$) noexcept {
  ::rxx::Map_MatrixXd (*Map_MatrixXd_new$)(const double *, ::std::size_t, ::std::size_t) = ::rxx::Map_MatrixXd_new;
  new (return$) ::rxx::Map_MatrixXd(Map_MatrixXd_new$(data, rows, cols));
}

void rxx$cxxbridge1$MapMut_MatrixXd_stride_new(double *data, ::std::size_t rows, ::std::size_t cols, ::std::size_t s0, ::std::size_t s1, ::rxx::MapMut_MatrixXd_stride *return$) noexcept {
  ::rxx::MapMut_MatrixXd_stride (*MapMut_MatrixXd_stride_new$)(double *, ::std::size_t, ::std::size_t, ::std::size_t, ::std::size_t) = ::rxx::MapMut_MatrixXd_stride_new;
  new (return$) ::rxx::MapMut_MatrixXd_stride(MapMut_MatrixXd_stride_new$(data, rows, cols, s0, s1));
}

void rxx$cxxbridge1$Map_MatrixXd_stride_new(const double *data, ::std::size_t rows, ::std::size_t cols, ::std::size_t s0, ::std::size_t s1, ::rxx::Map_MatrixXd_stride *return$) noexcept {
  ::rxx::Map_MatrixXd_stride (*Map_MatrixXd_stride_new$)(const double *, ::std::size_t, ::std::size_t, ::std::size_t, ::std::size_t) = ::rxx::Map_MatrixXd_stride_new;
  new (return$) ::rxx::Map_MatrixXd_stride(Map_MatrixXd_stride_new$(data, rows, cols, s0, s1));
}

void rxx$cxxbridge1$MapMut_Vector2d_new(double *data, ::rxx::MapMut_Vector2d *return$) noexcept {
  ::rxx::MapMut_Vector2d (*MapMut_Vector2d_new$)(double *) = ::rxx::MapMut_fixed_new;
  new (return$) ::rxx::MapMut_Vector2d(MapMut_Vector2d_new$(data));
}

void rxx$cxxbridge1$Map_Vector2d_new(const double *data, ::rxx::Map_Vector2d *return$) noexcept {
  ::rxx::Map_Vector2d (*Map_Vector2d_new$)(const double *) = ::rxx::Map_fixed_new;
  new (return$) ::rxx::Map_Vector2d(Map_Vector2d_new$(data));
}

void rxx$cxxbridge1$MapMut_Vector3d_new(double *data, ::rxx::MapMut_Vector3d *return$) noexcept {
  ::rxx::MapMut_Vector3d (*MapMut_Vector3d_new$)(double *) = ::rxx::MapMut_fixed_new;
  new (return$) ::rxx::MapMut_Vector3d(MapMut_Vector3d_new$(data));
}

void rxx$cxxbridge1$Map_Vector3d_new(const double *data, ::rxx::Map_Vector3d *return$) noexcept {
  ::rxx::Map_Vector3d (*Map_Vector3d_new$)(const double *) = ::rxx::Map_fixed_new;
  new (return$) ::rxx::Map_Vector3d(Map_Vector3d_new$(data));
}

void rxx$cxxbridge1$MapMut_Vector4d_new(double *data, ::rxx::MapMut_Vector4d *return$) noexcept {
  ::rxx::MapMut_Vector4d (*MapMut_Vector4d_new$)(double *) = ::rxx::MapMut_fixed_new;
  new (return$) ::rxx::MapMut_Vector4d(MapMut_Vector4d_new$(data));
}

void rxx$cxxbridge1$Map_Vector4d_new(const double *data, ::rxx::Map_Vector4d *return$) noexcept {
  ::rxx::Map_Vector4d (*Map_Vector4d_new$)(const double *) = ::rxx::Map_fixed_new;
  new (return$) ::rxx::Map_Vector4d(Map_Vector4d_new$(data));
}

void rxx$cxxbridge1$VectorXd_new(::std::size_t size, ::rxx::VectorXd *return$) noexcept {
  ::rxx::VectorXd (*VectorXd_new$)(::std::size_t) = ::rxx::VectorXd_new;
  new (return$) ::rxx::VectorXd(VectorXd_new$(size));
}

void rxx$cxxbridge1$VectorXd_clone(const ::rxx::VectorXd &v, ::rxx::VectorXd *return$) noexcept {
  ::rxx::VectorXd (*VectorXd_clone$)(const ::rxx::VectorXd &) = ::rxx::VectorXd_clone;
  new (return$) ::rxx::VectorXd(VectorXd_clone$(v));
}

void rxx$cxxbridge1$MapMut_VectorXd_new(double *data, ::std::size_t size, ::rxx::MapMut_VectorXd *return$) noexcept {
  ::rxx::MapMut_VectorXd (*MapMut_VectorXd_new$)(double *, ::std::size_t) = ::rxx::MapMut_VectorXd_new;
  new (return$) ::rxx::MapMut_VectorXd(MapMut_VectorXd_new$(data, size));
}

void rxx$cxxbridge1$Map_VectorXd_new(const double *data, ::std::size_t size, ::rxx::Map_VectorXd *return$) noexcept {
  ::rxx::Map_VectorXd (*Map_VectorXd_new$)(const double *, ::std::size_t) = ::rxx::Map_VectorXd_new;
  new (return$) ::rxx::Map_VectorXd(Map_VectorXd_new$(data, size));
}

void rxx$cxxbridge1$MapMut_VectorXd_stride_new(double *data, ::std::size_t size, ::std::size_t s, ::rxx::MapMut_VectorXd_stride *return$) noexcept {
  ::rxx::MapMut_VectorXd_stride (*MapMut_VectorXd_stride_new$)(double *, ::std::size_t, ::std::size_t) = ::rxx::MapMut_VectorXd_stride_new;
  new (return$) ::rxx::MapMut_VectorXd_stride(MapMut_VectorXd_stride_new$(data, size, s));
}

void rxx$cxxbridge1$Map_VectorXd_stride_new(const double *data, ::std::size_t size, ::std::size_t s, ::rxx::Map_VectorXd_stride *return$) noexcept {
  ::rxx::Map_VectorXd_stride (*Map_VectorXd_stride_new$)(const double *, ::std::size_t, ::std::size_t) = ::rxx::Map_VectorXd_stride_new;
  new (return$) ::rxx::Map_VectorXd_stride(Map_VectorXd_stride_new$(data, size, s));
}

void rxx$cxxbridge1$Quaterniond$normalized(const ::rxx::Quaterniond &self, ::rxx::Quaterniond *return$) noexcept {
  ::rxx::Quaterniond (::rxx::Quaterniond::*normalized$)() const = &::rxx::Quaterniond::normalized;
  new (return$) ::rxx::Quaterniond((self.*normalized$)());
}

void rxx$cxxbridge1$Quaterniond$normalize(::rxx::Quaterniond &self) noexcept {
  void (::rxx::Quaterniond::*normalize$)() = &::rxx::Quaterniond::normalize;
  (self.*normalize$)();
}

void rxx$cxxbridge1$Quaterniond$inverse(const ::rxx::Quaterniond &self, ::rxx::Quaterniond *return$) noexcept {
  ::rxx::Quaterniond (::rxx::Quaterniond::*inverse$)() const = &::rxx::Quaterniond::inverse;
  new (return$) ::rxx::Quaterniond((self.*inverse$)());
}

void rxx$cxxbridge1$Quaterniond_mul(const ::rxx::Quaterniond &self_, const ::rxx::Quaterniond &other, ::rxx::Quaterniond *return$) noexcept {
  ::rxx::Quaterniond (*Quaterniond_mul$)(const ::rxx::Quaterniond &, const ::rxx::Quaterniond &) = ::rxx::op_mul;
  new (return$) ::rxx::Quaterniond(Quaterniond_mul$(self_, other));
}

void rxx$cxxbridge1$Quaterniond$toRotationMatrix(const ::rxx::Quaterniond &self, ::rxx::Matrix3d *return$) noexcept {
  ::rxx::Matrix3d (::rxx::Quaterniond::*toRotationMatrix$)() const = &::rxx::Quaterniond::toRotationMatrix;
  new (return$) ::rxx::Matrix3d((self.*toRotationMatrix$)());
}

void rxx$cxxbridge1$MapMut_Quaterniond_new(double *data, ::rxx::MapMut_Quaterniond *return$) noexcept {
  ::rxx::MapMut_Quaterniond (*MapMut_Quaterniond_new$)(double *) = ::rxx::MapMut_fixed_new;
  new (return$) ::rxx::MapMut_Quaterniond(MapMut_Quaterniond_new$(data));
}

void rxx$cxxbridge1$Map_Quaterniond_new(const double *data, ::rxx::Map_Quaterniond *return$) noexcept {
  ::rxx::Map_Quaterniond (*Map_Quaterniond_new$)(const double *) = ::rxx::Map_fixed_new;
  new (return$) ::rxx::Map_Quaterniond(Map_Quaterniond_new$(data));
}

void rxx$cxxbridge1$AngleAxisd$inverse(const ::rxx::AngleAxisd &self, ::rxx::AngleAxisd *return$) noexcept {
  ::rxx::AngleAxisd (::rxx::AngleAxisd::*inverse$)() const = &::rxx::AngleAxisd::inverse;
  new (return$) ::rxx::AngleAxisd((self.*inverse$)());
}

void rxx$cxxbridge1$AngleAxisd_mul(const ::rxx::AngleAxisd &self_, const ::rxx::AngleAxisd &other, ::rxx::Quaterniond *return$) noexcept {
  ::rxx::Quaterniond (*AngleAxisd_mul$)(const ::rxx::AngleAxisd &, const ::rxx::AngleAxisd &) = ::rxx::op_mul;
  new (return$) ::rxx::Quaterniond(AngleAxisd_mul$(self_, other));
}

void rxx$cxxbridge1$AngleAxisd$toRotationMatrix(const ::rxx::AngleAxisd &self, ::rxx::Matrix3d *return$) noexcept {
  ::rxx::Matrix3d (::rxx::AngleAxisd::*toRotationMatrix$)() const = &::rxx::AngleAxisd::toRotationMatrix;
  new (return$) ::rxx::Matrix3d((self.*toRotationMatrix$)());
}
} // extern "C"
} // namespace rxx

#pragma once
#include <assert.h>
#include <memory>
#include <vector>
#include <iostream>
#include <Eigen/Core>
#include <Eigen/Geometry>
#include <rxx/wrapper.hh>

using namespace rxx;

using stride_t = Eigen::Stride<Eigen::Dynamic, Eigen::Dynamic>;
using stride_x1_t = Eigen::Stride<Eigen::Dynamic, 1>;

namespace Eigen {
using Matrix2x3d = Matrix<double, 2, 3>;
using Matrix2x3f = Matrix<float, 2, 3>;
using Matrix2x3i = Matrix<int, 2, 3>;
}

#define use_reloc(T)                                                    \
  namespace eigen_rxx {                                                       \
  using T = Eigen::T;                                                   \
  using MapMut_##T = Eigen::Map<T>;                                        \
  using Map_##T = Eigen::Map<const T>;                          \
  }                                                                     \

#define use_reloc_matx_stride(T)                                        \
  use_reloc(T)                                                          \
  namespace eigen_rxx {                                                       \
    using MapMut_##T##_stride=Eigen::Map<T, 0, stride_t>;                  \
    using Map_##T##_stride=Eigen::Map<const T, 0, stride_t>;      \
  }                                                                     \

#define use_reloc_vecx_stride(T)                                        \
  use_reloc(T)                                                          \
  namespace eigen_rxx {                                                       \
    using MapMut_##T##_stride=Eigen::Map<T, 0, stride_x1_t>;               \
    using Map_##T##_stride=Eigen::Map<const T, 0, stride_x1_t>;   \
  }                                                                     \

use_reloc(Vector2d)
use_reloc(Vector2f)
use_reloc(Vector2i)

use_reloc(Vector3d)
use_reloc(Vector3f)
use_reloc(Vector3i)

use_reloc(Vector4d)
use_reloc(Vector4f)
use_reloc(Vector4i)

use_reloc(Matrix2d)
use_reloc(Matrix2f)
use_reloc(Matrix2i)

use_reloc(Matrix3d)
use_reloc(Matrix3f)
use_reloc(Matrix3i)

use_reloc(Matrix4d)
use_reloc(Matrix4f)
use_reloc(Matrix4i)

use_reloc(Matrix2x3d)
use_reloc(Matrix2x3f)
use_reloc(Matrix2x3i)

use_reloc_vecx_stride(VectorXd)
use_reloc_matx_stride(MatrixXd)

use_reloc(Quaterniond)
use_reloc(Quaternionf)

use_reloc(AngleAxisd)
use_reloc(AngleAxisf)

namespace eigen_rxx {

template<typename T1, typename T2, typename T3>
T3 op_add(const T1 &self_, const T2 &other) {return self_ + other;}
template<typename T1, typename T2, typename T3>
T3 op_sub(const T1 &self_, const T2 &other) {return self_ - other;}
template<typename T1, typename T2, typename T3>
T3 op_mul(const T1 &self_, const T2 &other) {return self_ * other;}

VectorXd VectorXd_new(size_t size) {return VectorXd(size);}
VectorXd VectorXd_clone(const VectorXd &v) {return VectorXd(v);}

MapMut_VectorXd MapMut_VectorXd_new(double *data, size_t size) {
  return MapMut_VectorXd(data, size);
}
Map_VectorXd Map_VectorXd_new(const double *data, size_t size) {
  return Map_VectorXd(data, size);
}

MapMut_VectorXd_stride MapMut_VectorXd_stride_new(double *data, size_t size, size_t s)
{
  return MapMut_VectorXd_stride(data, size, stride_x1_t(s, 1));
}
Map_VectorXd_stride Map_VectorXd_stride_new(const double *data, size_t size, size_t s)
{
  return Map_VectorXd_stride(data, size, stride_x1_t(s, 1));
}

MatrixXd MatrixXd_new(size_t rows, size_t cols) {return MatrixXd(rows, cols);}
MatrixXd MatrixXd_clone(const MatrixXd &v) {return MatrixXd(v);}

MapMut_MatrixXd MapMut_MatrixXd_new(double *data, size_t rows, size_t cols) {
  return MapMut_MatrixXd(data, rows, cols);
}
Map_MatrixXd Map_MatrixXd_new(const double *data, size_t rows, size_t cols) {
  return Map_MatrixXd(data, rows, cols);
}

MapMut_MatrixXd_stride MapMut_MatrixXd_stride_new(double *data, size_t rows, size_t cols, size_t s0, size_t s1)
{
  return MapMut_MatrixXd_stride(data, rows, cols, stride_t(s0, s1));
}
Map_MatrixXd_stride Map_MatrixXd_stride_new(const double *data, size_t rows, size_t cols, size_t s0, size_t s1)
{
  return Map_MatrixXd_stride(data, rows, cols, stride_t(s0, s1));
}

template<typename T, typename T2>
Eigen::Map<T> MapMut_fixed_new(T2 *data) {return Eigen::Map<T>(data);}
template<typename T, typename T2>
Eigen::Map<const T> Map_fixed_new(const T2 *data) {return Eigen::Map<const T>(data);}

template<typename T>
std::unique_ptr<std::string> to_string(const T &v) {
  std::stringstream ss;
  ss << v;
  return std::make_unique<std::string>(ss.str());
}

} // namespace eigen_rxx

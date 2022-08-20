#pragma once

#include <sophus/so2.hpp>
#include <sophus/so3.hpp>
#include <sophus/rxso2.hpp>
#include <sophus/rxso3.hpp>
#include <sophus/se2.hpp>
#include <sophus/se3.hpp>
#include <sophus/sim2.hpp>
#include <sophus/sim3.hpp>

namespace sophus_rxx {

using namespace Sophus;

template<typename T1, typename T2, typename T3>
T3 op_mul(const T1 &self_, const T2 &other) {return self_ * other;}

} // namespace

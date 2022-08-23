#pragma once

#include <NvInfer.h>

namespace tensorrt_rxx {

using namespace nvinfer1;

extern "C" using log_fn_t = void(void *obj, ILogger::Severity severity, const char *msg);

struct RustLogger: ILogger {

  RustLogger(void *obj, log_fn_t *log_fn): obj_(obj), log_fn_(log_fn) {}

  virtual void log(Severity severity, const char *msg) noexcept override {
    log_fn_(obj_, severity, msg);
  }

  static ILogger *create(void *obj, log_fn_t *log_fn) {
    return new RustLogger(obj, log_fn);
  }

  void *obj_;
  log_fn_t *log_fn_;
};

void log(ILogger *logger, ILogger::Severity severity, const char *msg);

} // namespace tensorrt_rxx

#include <tensorrt_rxx/wrapper.hh>
#include <stdio.h>

namespace tensorrt_rxx {

void log(ILogger *logger, ILogger::Severity severity, const char *msg)
{
  logger->log(severity, msg);
}

}

#include "DisplayOutput.h"

DisplayOutput::DisplayOutput(IDXGIAdapter* adapter, IDXGIOutput* output)
    : adapter(adapter), output(output) {}

DisplayOutput::~DisplayOutput() {
  SAFE_RELEASE(output);
  SAFE_RELEASE(adapter);
}

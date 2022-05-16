#ifndef DISPLAY_OUTPUT_H
#define DISPLAY_OUTPUT_H

#include "common.h"

class DisplayOutput {
 public:
  DisplayOutput(IDXGIAdapter* adapter, IDXGIOutput* output);
  ~DisplayOutput();

  IDXGIAdapter* adapter;
  IDXGIOutput* output;
};

#endif  // DISPLAY_OUTPUT_H

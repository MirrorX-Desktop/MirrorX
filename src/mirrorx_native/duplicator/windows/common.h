#ifndef COMMON_H
#define COMMON_H

#include <d3d11.h>
#include <dxgi1_2.h>
#include <atomic>
#include <thread>
#include <vector>
#include "../../ffi_log/ffi_log.h"
#include "../include/callback.h"
#include "DirectXMath.h"

#pragma comment(lib, "d3d11.lib")
#pragma comment(lib, "dxgi.lib")

#define NUM_VERTICES 6

#define SAFE_RELEASE(p) \
  if (p) {              \
    (p)->Release();     \
    (p) = nullptr;      \
  }

static const char* GetD3DFeatureLevelName(D3D_FEATURE_LEVEL level) {
  switch (level) {
    case D3D_FEATURE_LEVEL_1_0_CORE:
      return "D3D_FEATURE_LEVEL_1_0_CORE";
    case D3D_FEATURE_LEVEL_9_1:
      return "D3D_FEATURE_LEVEL_9_1";
    case D3D_FEATURE_LEVEL_9_2:
      return "D3D_FEATURE_LEVEL_9_2";
    case D3D_FEATURE_LEVEL_9_3:
      return "D3D_FEATURE_LEVEL_9_3";
    case D3D_FEATURE_LEVEL_10_0:
      return "D3D_FEATURE_LEVEL_10_0";
    case D3D_FEATURE_LEVEL_10_1:
      return "D3D_FEATURE_LEVEL_10_1";
    case D3D_FEATURE_LEVEL_11_0:
      return "D3D_FEATURE_LEVEL_11_0";
    case D3D_FEATURE_LEVEL_11_1:
      return "D3D_FEATURE_LEVEL_11_1";
    case D3D_FEATURE_LEVEL_12_0:
      return "D3D_FEATURE_LEVEL_12_0";
    case D3D_FEATURE_LEVEL_12_1:
      return "D3D_FEATURE_LEVEL_12_1";
  }

  return "";
}

#endif  // COMMON_H

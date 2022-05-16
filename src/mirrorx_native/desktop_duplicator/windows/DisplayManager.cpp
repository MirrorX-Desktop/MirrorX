#include "DisplayManager.h"

DisplayManager::DisplayManager() {
  IDXGIFactory1* factory = nullptr;

  auto hr = CreateDXGIFactory1(__uuidof(IDXGIFactory1), (void**)&factory);
  if (FAILED(hr)) {
    return;
  }

  UINT adapter_index = 0;
  while (true) {
    IDXGIAdapter* adapter = nullptr;
    hr = factory->EnumAdapters(adapter_index, &adapter);
    if (FAILED(hr)) {
      break;
    }

    UINT output_index = 0;
    while (true) {
      IDXGIOutput* output = nullptr;
      hr = adapter->EnumOutputs(output_index, &output);
      if (FAILED(hr)) {
        break;
      }

      auto display_output = new DisplayOutput(adapter, output);
      m_displays.push_back(display_output);

      output_index++;
    }

    adapter_index++;
  }

  SAFE_RELEASE(factory);
}

DisplayManager::~DisplayManager() {
  for (auto display : m_displays) {
    delete display;
  }
}

std::vector<DisplayOutput*> DisplayManager::ListDisplays() {
  return m_displays;
}

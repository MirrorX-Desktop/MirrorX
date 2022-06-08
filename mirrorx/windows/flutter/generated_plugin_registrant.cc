//
//  Generated file. Do not edit.
//

// clang-format off

#include "generated_plugin_registrant.h"

#include <texture_render/texture_render_plugin_c_api.h>
#include <window_size/window_size_plugin.h>

void RegisterPlugins(flutter::PluginRegistry* registry) {
  TextureRenderPluginCApiRegisterWithRegistrar(
      registry->GetRegistrarForPlugin("TextureRenderPluginCApi"));
  WindowSizePluginRegisterWithRegistrar(
      registry->GetRegistrarForPlugin("WindowSizePlugin"));
}

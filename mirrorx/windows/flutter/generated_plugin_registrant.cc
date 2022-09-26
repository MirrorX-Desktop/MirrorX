//
//  Generated file. Do not edit.
//

// clang-format off

#include "generated_plugin_registrant.h"

#include <sentry_flutter/sentry_flutter_plugin.h>
#include <texture_render/texture_render_plugin_c_api.h>

void RegisterPlugins(flutter::PluginRegistry* registry) {
  SentryFlutterPluginRegisterWithRegistrar(
      registry->GetRegistrarForPlugin("SentryFlutterPlugin"));
  TextureRenderPluginCApiRegisterWithRegistrar(
      registry->GetRegistrarForPlugin("TextureRenderPluginCApi"));
}

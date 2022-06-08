#include "include/texture_render/texture_render_plugin_c_api.h"

#include <flutter/plugin_registrar_windows.h>

#include "texture_render_plugin.h"

void TextureRenderPluginCApiRegisterWithRegistrar(
    FlutterDesktopPluginRegistrarRef registrar) {
  texture_render::TextureRenderPlugin::RegisterWithRegistrar(
      flutter::PluginRegistrarManager::GetInstance()
          ->GetRegistrar<flutter::PluginRegistrarWindows>(registrar));
}

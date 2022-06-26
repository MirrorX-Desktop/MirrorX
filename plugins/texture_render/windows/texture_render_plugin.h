#ifndef FLUTTER_PLUGIN_TEXTURE_RENDER_PLUGIN_H_
#define FLUTTER_PLUGIN_TEXTURE_RENDER_PLUGIN_H_

#include <flutter/method_channel.h>
#include <flutter/plugin_registrar_windows.h>

#include "video_texture.h"
#include <memory>

namespace texture_render {

class TextureRenderPlugin : public flutter::Plugin {
public:
  static void RegisterWithRegistrar(flutter::PluginRegistrarWindows *registrar);

  TextureRenderPlugin(flutter::TextureRegistrar *texture_registrar);

  virtual ~TextureRenderPlugin();

  // Disallow copy and assign.
  TextureRenderPlugin(const TextureRenderPlugin &) = delete;
  TextureRenderPlugin &operator=(const TextureRenderPlugin &) = delete;

private:
  flutter::TextureRegistrar *texture_registrar;

  // Called when a method is called on this plugin's channel from Dart.
  void HandleMethodCall(
      const flutter::MethodCall<flutter::EncodableValue> &method_call,
      std::unique_ptr<flutter::MethodResult<flutter::EncodableValue>> result);
};

} // namespace texture_render

#endif // FLUTTER_PLUGIN_TEXTURE_RENDER_PLUGIN_H_

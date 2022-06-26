#include "texture_render_plugin.h"

// This must be included before many other Windows headers.
#include <windows.h>

#include <flutter/method_channel.h>
#include <flutter/plugin_registrar_windows.h>
#include <flutter/standard_method_codec.h>

#include <memory>
#include <sstream>

void UpdateFrameCallback(int64_t texture_id, void *video_texture_ptr,
                         void *new_frame_ptr) {
  auto video_texture = reinterpret_cast<VideoTexture *>(video_texture_ptr);
  video_texture->UpdateFrame(new_frame_ptr);
}

namespace texture_render {

// static
void TextureRenderPlugin::RegisterWithRegistrar(
    flutter::PluginRegistrarWindows *registrar) {
  auto channel =
      std::make_unique<flutter::MethodChannel<flutter::EncodableValue>>(
          registrar->messenger(), "texture_render",
          &flutter::StandardMethodCodec::GetInstance());

  auto plugin =
      std::make_unique<TextureRenderPlugin>(registrar->texture_registrar());

  channel->SetMethodCallHandler(
      [plugin_pointer = plugin.get()](const auto &call, auto result) {
        plugin_pointer->HandleMethodCall(call, std::move(result));
      });

  registrar->AddPlugin(std::move(plugin));
}

TextureRenderPlugin::TextureRenderPlugin(
    flutter::TextureRegistrar *texture_registrar)
    : texture_registrar(texture_registrar) {}

TextureRenderPlugin::~TextureRenderPlugin() {}

void TextureRenderPlugin::HandleMethodCall(
    const flutter::MethodCall<flutter::EncodableValue> &method_call,
    std::unique_ptr<flutter::MethodResult<flutter::EncodableValue>> result) {
  if (method_call.method_name().compare("register_texture") == 0) {
    auto video_texture = new VideoTexture(texture_registrar);

    auto texture_id = video_texture->RegisterTexture();

    auto update_frame_callback_pointer =
        reinterpret_cast<int64_t>(UpdateFrameCallback);

    auto video_texture_pointer = reinterpret_cast<int64_t>(video_texture);

    result->Success(flutter::EncodableMap{
        {
            flutter::EncodableValue("texture_id"),
            flutter::EncodableValue(texture_id),
        },
        {
            flutter::EncodableValue("video_texture_ptr"),
            flutter::EncodableValue(video_texture_pointer),
        },
        {
            flutter::EncodableValue("update_frame_callback_ptr"),
            flutter::EncodableValue(update_frame_callback_pointer),
        },
    });
  } else if (method_call.method_name().compare("deregister_texture") == 0) {
    const auto *args =
        std::get_if<flutter::EncodableMap>(method_call.arguments());

    if (!args) {
      result->Error("args is null");
      return;
    }

    // auto *texture_id = std::get_if<int64_t>(
    //     &(args->find(flutter::EncodableValue("texture_id"))->second));
    // if (!texture_id) {
    //   result->Error("arg 'texture_id' is null");
    //   return;
    // }

    // texture_registrar->UnregisterTexture(*texture_id);

    auto *video_texture_ptr = std::get_if<int64_t>(
        &(args->find(flutter::EncodableValue("video_texture_ptr"))->second));
    if (!video_texture_ptr) {
      result->Error("arg 'video_texture_ptr' is null");
      return;
    }

    auto video_texture = reinterpret_cast<VideoTexture *>(*video_texture_ptr);
    delete video_texture;

    result->Success();
  } else {
    result->NotImplemented();
  }
}

} // namespace texture_render

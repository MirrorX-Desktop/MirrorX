#include "texture_render_plugin.h"

// This must be included before many other Windows headers.
#include <windows.h>

#include <flutter/method_channel.h>
#include <flutter/plugin_registrar_windows.h>
#include <flutter/standard_method_codec.h>

#include <memory>
#include <sstream>

void UpdateFrameCallback(VideoTexture *video_texture, uint8_t *frame_buffer,
                         size_t frame_width, size_t frame_height) {
  if (!video_texture) {
    printf("video_texture is null\r\n");
    return;
  }

  if (!frame_buffer) {
    printf("frame_buffer is null\r\n");
    return;
  }

  video_texture->UpdateFrame(frame_buffer, frame_width, frame_height);
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

    auto update_frame_callback_pointer =
        reinterpret_cast<int64_t>(UpdateFrameCallback);

    auto video_texture_pointer = reinterpret_cast<int64_t>(video_texture);

    result->Success(flutter::EncodableMap{
        {
            flutter::EncodableValue("texture_id"),
            flutter::EncodableValue(video_texture->texture_id),
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

    auto *video_texture_ptr = std::get_if<int64_t>(
        &(args->find(flutter::EncodableValue("video_texture_ptr"))->second));
    if (!video_texture_ptr) {
      result->Error("arg 'video_texture_ptr' is null");
      return;
    }

    auto video_texture = reinterpret_cast<VideoTexture *>(*video_texture_ptr);
    delete video_texture;
    video_texture = nullptr;

    result->Success();
  } else {
    result->NotImplemented();
  }
}

} // namespace texture_render

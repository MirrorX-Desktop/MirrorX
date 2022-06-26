#ifndef VIDEO_TEXTURE_H_
#define VIDEO_TEXTURE_H_

#include <windows.h>

#include <flutter/texture_registrar.h>

class VideoTexture {
public:
  VideoTexture::VideoTexture(flutter::TextureRegistrar *texture_registrar)
      : texture_registrar(texture_registrar) {}
  virtual ~VideoTexture();

  int64_t RegisterTexture();
  void UpdateFrame(void *frame_pointer);

  VideoTexture(VideoTexture const &) = delete;
  VideoTexture &operator=(VideoTexture const &) = delete;

private:
  int64_t texture_id = -1;
  flutter::TextureRegistrar *texture_registrar = nullptr;
  std::unique_ptr<flutter::TextureVariant> texture = nullptr;
  std::unique_ptr<FlutterDesktopPixelBuffer> flutter_desktop_pixel_buffer_ =
      nullptr;
  HANDLE semaphore = nullptr;
  uint8_t *pixel_buffer = nullptr;

  const FlutterDesktopPixelBuffer *ConvertPixelBufferForFlutter(size_t width,
                                                                size_t height);
};

#endif // VIDEO_TEXTURE_H_
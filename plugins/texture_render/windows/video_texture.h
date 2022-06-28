#ifndef VIDEO_TEXTURE_H_
#define VIDEO_TEXTURE_H_

#include <windows.h>

#include <flutter/texture_registrar.h>

class VideoTexture {
public:
  VideoTexture::VideoTexture(flutter::TextureRegistrar *texture_registrar);

  virtual ~VideoTexture();

  void UpdateFrame(uint8_t *frame_buffer, size_t frame_width,
                   size_t frame_height);

  VideoTexture(VideoTexture const &) = delete;
  VideoTexture &operator=(VideoTexture const &) = delete;

public:
  int64_t texture_id = -1;

private:
  flutter::TextureRegistrar *texture_registrar_ = nullptr;
  flutter::TextureVariant texture_;
  FlutterDesktopPixelBuffer *pixel_buffer_ = nullptr;

  HANDLE copy_pixel_buffer_semaphore = nullptr;

  const FlutterDesktopPixelBuffer *CopyPixelBuffer(size_t width, size_t height);
};

#endif // VIDEO_TEXTURE_H_
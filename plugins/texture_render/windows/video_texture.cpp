#include "video_texture.h"

VideoTexture::VideoTexture(flutter::TextureRegistrar *texture_registrar)
    : texture_registrar_(texture_registrar),
      texture_(flutter::PixelBufferTexture(
          std::bind(&VideoTexture::CopyPixelBuffer, this, std::placeholders::_1,
                    std::placeholders::_2))),
      pixel_buffer_(new FlutterDesktopPixelBuffer()) {
  texture_id = texture_registrar->RegisterTexture(&texture_);
  copy_pixel_buffer_semaphore = CreateSemaphore(nullptr, 0, 1, nullptr);
  pixel_buffer_->release_context = copy_pixel_buffer_semaphore;
  pixel_buffer_->release_callback = [](void *release_context) {
    ReleaseSemaphore(release_context, 1, nullptr);
  };
}

VideoTexture::~VideoTexture() {
  ReleaseSemaphore(copy_pixel_buffer_semaphore, 1, nullptr);
  texture_registrar_->UnregisterTexture(texture_id);
  CloseHandle(copy_pixel_buffer_semaphore);

  if (pixel_buffer_) {
    delete pixel_buffer_;
    pixel_buffer_ = nullptr;
  }
}

void VideoTexture::UpdateFrame(uint8_t *frame_buffer, size_t frame_width,
                               size_t frame_height) {
  pixel_buffer_->buffer = frame_buffer;
  pixel_buffer_->width = frame_width;
  pixel_buffer_->height = frame_height;
  texture_registrar_->MarkTextureFrameAvailable(texture_id);
  WaitForSingleObject(copy_pixel_buffer_semaphore, INFINITE);
}

const FlutterDesktopPixelBuffer *VideoTexture::CopyPixelBuffer(size_t width,
                                                               size_t height) {
  return pixel_buffer_;
}
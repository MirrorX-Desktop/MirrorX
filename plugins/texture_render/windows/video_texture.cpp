#include "video_texture.h"

VideoTexture::~VideoTexture() {
  if (texture_id > 0) {
    texture_registrar->UnregisterTexture(texture_id);
  }

  if (semaphore) {
    CloseHandle(semaphore);
  }

  texture_id = -1;
  texture = nullptr;
  texture_registrar = nullptr;
  pixel_buffer = nullptr;
}

int64_t VideoTexture::RegisterTexture() {
  if (!texture_registrar) {
    return -1;
  }

  semaphore = CreateSemaphore(NULL, 1, 1, L"VideoTextureSemaphore");
  if (semaphore == 0) {
    return -1;
  }

  texture =
      std::make_unique<flutter::TextureVariant>(flutter::PixelBufferTexture(
          [this](size_t width,
                 size_t height) -> const FlutterDesktopPixelBuffer * {
            return this->ConvertPixelBufferForFlutter(width, height);
          }));

  texture_id = texture_registrar->RegisterTexture(texture.get());
  return texture_id;
}

void VideoTexture::UpdateFrame(void *frame_pointer) {
  if (texture_id == -1) {
    return;
  }

  if (texture_id > 0) {
    pixel_buffer = reinterpret_cast<uint8_t *>(frame_pointer);
    texture_registrar->MarkTextureFrameAvailable(texture_id);
    WaitForSingleObject(semaphore, INFINITE);
  }
}

const FlutterDesktopPixelBuffer *
VideoTexture::ConvertPixelBufferForFlutter(size_t width, size_t height) {
  if (texture_id <= 0) {
    return nullptr;
  }

  // libyuv::NV12ToABGRMatrix(frame_y_buffer, y_stride, frame_uv_buffer,
  // uv_stride,
  //                          rgba_frame_ptr, y_stride,
  //                          &libyuv::kYuvF709Constants, width, height);

  if (!flutter_desktop_pixel_buffer_) {
    flutter_desktop_pixel_buffer_ =
        std::make_unique<FlutterDesktopPixelBuffer>();

    flutter_desktop_pixel_buffer_->release_callback =
        [](void *release_context) {
          auto semaphore_handle = reinterpret_cast<HANDLE>(release_context);
          ReleaseSemaphore(semaphore_handle, 1, nullptr);
        };
  }

  flutter_desktop_pixel_buffer_->buffer = pixel_buffer;
  flutter_desktop_pixel_buffer_->width = width;
  flutter_desktop_pixel_buffer_->height = height;
  flutter_desktop_pixel_buffer_->release_context = semaphore;

  return flutter_desktop_pixel_buffer_.get();
}
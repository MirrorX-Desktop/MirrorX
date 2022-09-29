import 'package:flutter/foundation.dart';

import 'texture_render_platform_interface.dart';

class TextureRender {
  static final TextureRender _instance = TextureRender();

  static TextureRender get instance => _instance;

  Future<int?> registerTexture() {
    return TextureRenderPlatform.instance.registerTexture();
  }

  Future<void> deregisterTexture(int textureId) {
    return TextureRenderPlatform.instance.deregisterTexture(textureId);
  }

  Future<void> sendVideoFrameBuffer(Uint8List videoFrameBuffer) {
    return TextureRenderPlatform.instance
        .sendVideoFrameBuffer(videoFrameBuffer);
  }
}

import 'package:flutter/foundation.dart';

import 'model.dart';
import 'texture_render_platform_interface.dart';

class TextureRender {
  static final TextureRender _instance = TextureRender();

  static TextureRender get instance => _instance;

  Future<RegisterTextureResponse> registerTexture() {
    return TextureRenderPlatform.instance.registerTexture();
  }

  Future<void> deregisterTexture(int textureID) {
    return TextureRenderPlatform.instance.deregisterTexture(textureID);
  }

  Future<void> sendVideoFrameBuffer(Uint8List videoFrameBuffer) {
    return TextureRenderPlatform.instance
        .sendVideoFrameBuffer(videoFrameBuffer);
  }
}

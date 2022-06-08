import 'model.dart';
import 'texture_render_platform_interface.dart';

class TextureRender {
  Future<RegisterTextureResponse> registerTexture() {
    return TextureRenderPlatform.instance.registerTexture();
  }

  Future<void> deregisterTexture(int textureID, int videoTexturePointer) {
    return TextureRenderPlatform.instance
        .deregisterTexture(textureID, videoTexturePointer);
  }
}

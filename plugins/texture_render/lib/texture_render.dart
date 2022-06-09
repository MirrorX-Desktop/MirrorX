import 'model.dart';
import 'texture_render_platform_interface.dart';

class TextureRender {
  static final TextureRender _instance = TextureRender();

  static TextureRender get instance => _instance;

  Future<RegisterTextureResponse> registerTexture() {
    return TextureRenderPlatform.instance.registerTexture();
  }

  Future<void> deregisterTexture(int textureID, int videoTexturePointer) {
    return TextureRenderPlatform.instance
        .deregisterTexture(textureID, videoTexturePointer);
  }
}

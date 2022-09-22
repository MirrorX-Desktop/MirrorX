import 'package:flutter/foundation.dart';
import 'package:plugin_platform_interface/plugin_platform_interface.dart';

import 'model.dart';
import 'texture_render_method_channel.dart';

abstract class TextureRenderPlatform extends PlatformInterface {
  /// Constructs a TextureRenderPlatform.
  TextureRenderPlatform() : super(token: _token);

  static final Object _token = Object();

  static TextureRenderPlatform _instance = MethodChannelTextureRender();

  /// The default instance of [TextureRenderPlatform] to use.
  ///
  /// Defaults to [MethodChannelTextureRender].
  static TextureRenderPlatform get instance => _instance;

  /// Platform-specific implementations should set this with their own
  /// platform-specific class that extends [TextureRenderPlatform] when
  /// they register themselves.
  static set instance(TextureRenderPlatform instance) {
    PlatformInterface.verifyToken(instance, _token);
    _instance = instance;
  }

  Future<RegisterTextureResponse> registerTexture() {
    throw UnimplementedError(
        'registerVideoTexture() has not been implemented.');
  }

  Future<void> deregisterTexture(int textureId) {
    throw UnimplementedError(
        'deregisterVideoTexture() has not been implemented.');
  }

  void sendVideoFrameBuffer(Uint8List videoFrameBuffer) {
    throw UnimplementedError('sendVideoBuffer() has not been implemented.');
  }
}

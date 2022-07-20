import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';

import 'model.dart';
import 'texture_render_platform_interface.dart';

/// An implementation of [TextureRenderPlatform] that uses method channels.
class MethodChannelTextureRender extends TextureRenderPlatform {
  /// The method channel used to interact with the native platform.
  @visibleForTesting
  final methodChannel = const MethodChannel('texture_render');

  @override
  Future<RegisterTextureResponse> registerTexture() async {
    Map? result = await methodChannel.invokeMethod<Map>('register_texture');
    if (result == null) {
      return Future.error("registerTexture: method call returns null");
    }
    return RegisterTextureResponse.fromMap(result);
  }

  @override
  Future<void> deregisterTexture(int textureID, int videoTexturePointer) async {
    await methodChannel.invokeMethod('deregister_texture',
        DeregisterTextureRequest(textureID, videoTexturePointer).toMap());
  }
}

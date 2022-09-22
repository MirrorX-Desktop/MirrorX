import 'dart:typed_data';

import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';

import 'model.dart';
import 'texture_render_platform_interface.dart';

/// An implementation of [TextureRenderPlatform] that uses method channels.
class MethodChannelTextureRender extends TextureRenderPlatform {
  /// The method channel used to interact with the native platform.
  @visibleForTesting
  final methodChannel = const MethodChannel('texture_render_method_channel');

  @visibleForTesting
  final binaryChannel =
      const BasicMessageChannel('texture_render_binary_channel', BinaryCodec());

  @override
  Future<RegisterTextureResponse> registerTexture() async {
    Map? result = await methodChannel.invokeMethod<Map>('register_texture');
    if (result == null) {
      return Future.error("registerTexture: method call returns null");
    }
    return RegisterTextureResponse.fromMap(result);
  }

  @override
  Future<void> deregisterTexture(int textureId) async {
    await methodChannel.invokeMethod(
        'deregister_texture', DeregisterTextureRequest(textureId).toMap());
  }

  @override
  void sendVideoFrameBuffer(Uint8List videoFrameBuffer) {
    return binaryChannel.send(ByteData.view(videoFrameBuffer.buffer)).ignore();
  }
}

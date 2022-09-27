import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';

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
  Future<int?> registerTexture() async {
    return await methodChannel.invokeMethod<int>('register_texture');
  }

  @override
  Future<void> deregisterTexture(int textureId) async {
    await methodChannel.invokeMethod('deregister_texture', textureId);
  }

  @override
  void sendVideoFrameBuffer(Uint8List videoFrameBuffer) {
    return binaryChannel.send(ByteData.view(videoFrameBuffer.buffer)).ignore();
  }
}

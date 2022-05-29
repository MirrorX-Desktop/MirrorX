import 'dart:async';

import 'package:app_plugin/method_channel_model.dart';
import 'package:flutter/services.dart';

class AppPlugin {
  static const MethodChannel _channel = MethodChannel('app_plugin');

  static Future<RegisterVideoTextureModel> registerVideoTexture() async {
    try {
      Map? result = await _channel.invokeMethod('register_video_texture');
      if (result == null) {
        return Future.error("registerVideoTexture: method call returns null");
      }

      return RegisterVideoTextureModel.fromMap(result);
    } catch (error) {
      return Future.error("registerVideoTexture: call error($error)");
    }
  }

  static Future<void> deregisterVideoTexture(
      int textureID, int videoTexturePointer) async {
    try {
      await _channel.invokeMethod('deregister_video_texture', <String, int>{
        'texture_id': textureID,
        'video_texture_ptr': videoTexturePointer
      });
    } catch (error) {
      return Future.error("deregisterVideoTexture: call error($error)");
    }
  }
}

// DynamicLibrary _openLibrary() {
//   if (Platform.isMacOS || Platform.isIOS) {
//     return DynamicLibrary.executable();
//   } else if (Platform.isWindows) {
//     return DynamicLibrary.open("mirrorx_core.dll");
//   } else {
//     throw UnsupportedError("Not supported platform yet");
//   }
// }

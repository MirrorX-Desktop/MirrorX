// You have generated a new plugin project without
// specifying the `--platforms` flag. A plugin project supports no platforms is generated.
// To add platforms, run `flutter create -t plugin --platforms <platforms> .` under the same
// directory. You can also find a detailed instruction on how to add platforms in the `pubspec.yaml` at https://flutter.dev/docs/development/packages-and-plugins/developing-packages#plugin-platforms.

import 'dart:async';

import 'package:flutter/services.dart';

class VideoTexture {
  static const MethodChannel _channel = MethodChannel('video_texture');

  static Future<int?> registerTextureID() async {
    final int? textureID = await _channel.invokeMethod('register_texture');
    return textureID;
  }

  static Future<void> deregisterTextureID(int textureID) async {
    await _channel.invokeMethod(
        'dispose_texture', <String, int>{'texture_id': textureID});
  }
}

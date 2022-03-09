// You have generated a new plugin project without
// specifying the `--platforms` flag. A plugin project supports no platforms is generated.
// To add platforms, run `flutter create -t plugin --platforms <platforms> .` under the same
// directory. You can also find a detailed instruction on how to add platforms in the `pubspec.yaml` at https://flutter.dev/docs/development/packages-and-plugins/developing-packages#plugin-platforms.

import 'dart:async';
import 'dart:developer' as dev;
import 'dart:ffi';
import 'dart:io';

import 'package:mirrorx_sdk/bridge_generated.dart';

class MirrorXSDK {
  static final _core = MirrorXCoreImpl(_openLibrary());

  static Future<String> requestDeviceToken() => _core.requestDeviceToken();
}

DynamicLibrary _openLibrary() {
  if (Platform.isMacOS || Platform.isIOS) {
    return DynamicLibrary.executable();
  } else {
    throw UnsupportedError("Not supported platform yet");
  }
}

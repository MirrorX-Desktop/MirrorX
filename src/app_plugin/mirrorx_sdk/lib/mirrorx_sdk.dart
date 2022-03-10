// You have generated a new plugin project without
// specifying the `--platforms` flag. A plugin project supports no platforms is generated.
// To add platforms, run `flutter create -t plugin --platforms <platforms> .` under the same
// directory. You can also find a detailed instruction on how to add platforms in the `pubspec.yaml` at https://flutter.dev/docs/development/packages-and-plugins/developing-packages#plugin-platforms.

import 'dart:async';
import 'dart:developer' as dev;
import 'dart:ffi';
import 'dart:io';

import 'package:mirrorx_sdk/bridge_generated.dart';
import 'package:mirrorx_sdk/config.dart';
import 'package:path_provider/path_provider.dart';

class MirrorXSDK {
  MirrorXSDK._(MirrorXCore core)
      : _core = core,
        _config = Config(core);

  static Completer<MirrorXSDK>? _completer;

  final MirrorXCore _core;
  final Config _config;

  Config get config => _config;

  static Future<MirrorXSDK> getInstance() async {
    if (_completer == null) {
      final completer = Completer<MirrorXSDK>();

      try {
        final applicationSupportDir = await getApplicationSupportDirectory();
        dev.log("application support dir: $applicationSupportDir");

        final MirrorXCore core = MirrorXCoreImpl(_openLibrary());
        final success =
            await core.initSdk(configDbPath: applicationSupportDir.path);

        if (success) {
          completer.complete(MirrorXSDK._(core));
        } else {
          completer.completeError(Exception("init sdk failed"));
        }
      } on Exception catch (e) {
        completer.completeError(e);
        return completer.future;
      }

      _completer = completer;
    }
    return _completer!.future;
  }

  Future<String> requestDeviceToken() => _core.requestDeviceToken();
}

DynamicLibrary _openLibrary() {
  if (Platform.isMacOS || Platform.isIOS) {
    return DynamicLibrary.executable();
  } else {
    throw UnsupportedError("Not supported platform yet");
  }
}

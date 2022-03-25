// You have generated a new plugin project without
// specifying the `--platforms` flag. A plugin project supports no platforms is generated.
// To add platforms, run `flutter create -t plugin --platforms <platforms> .` under the same
// directory. You can also find a detailed instruction on how to add platforms in the `pubspec.yaml` at https://flutter.dev/docs/development/packages-and-plugins/developing-packages#plugin-platforms.

import 'dart:async';
import 'dart:developer' as dev;
import 'dart:ffi';
import 'dart:io';

import 'package:mirrorx_sdk/bridge_generated.dart';
import 'package:path_provider/path_provider.dart';

Future<MirrorXCore?> initSDK() async {
  try {
    final applicationSupportDir = await getApplicationSupportDirectory();
    dev.log("application support dir: $applicationSupportDir");

    final MirrorXCore core = MirrorXCoreImpl(_openLibrary());
    final success =
        await core.initSdk(configDbPath: applicationSupportDir.path);
    if (success) {
      return core;
    } else {
      dev.log("init sdk failed");
      return null;
    }
  } catch (err) {
    dev.log("init sdk failed with error", error: err);
    return null;
  }
}

DynamicLibrary _openLibrary() {
  if (Platform.isMacOS || Platform.isIOS) {
    return DynamicLibrary.executable();
  } else if (Platform.isWindows) {
    return DynamicLibrary.open("mirrorx_core.dll");
  } else {
    throw UnsupportedError("Not supported platform yet");
  }
}

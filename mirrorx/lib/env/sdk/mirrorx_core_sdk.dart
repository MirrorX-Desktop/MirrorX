import 'dart:ffi';
import 'dart:io';
import 'dart:typed_data';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

import 'mirrorx_core.dart';

const String _libName = 'mirrorx_core';

final DynamicLibrary _dylib = () {
  if (Platform.isIOS) {
    return DynamicLibrary.process();
  }

  if (Platform.isMacOS) {
    return DynamicLibrary.executable();
  }

  if (Platform.isAndroid || Platform.isLinux) {
    return DynamicLibrary.open('lib$_libName.so');
  }

  if (Platform.isWindows) {
    return DynamicLibrary.open('$_libName.dll');
  }

  throw UnsupportedError('Unknown platform: ${Platform.operatingSystem}');
}();

class MirrorXCoreSDK {
  static MirrorXCoreImpl? _instance;

  static MirrorXCoreImpl get instance => _getInstance();

  static MirrorXCoreImpl _getInstance() {
    if (_instance == null) {
      _instance = MirrorXCoreImpl(_dylib);
      _instance!.initLogger();
    }
    return _instance!;
  }
}

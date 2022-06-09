import 'dart:ffi';
import 'dart:io';

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
  static final MirrorXCoreImpl _instance = MirrorXCoreImpl(_dylib);

  static MirrorXCoreImpl get instance => _instance;
}

import 'dart:developer';
import 'dart:ffi';
import 'dart:io';

import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:mirrorx_sdk/bridge_generated.dart';
import 'package:path_provider/path_provider.dart';

class SDKController extends GetxController with StateMixin<MirrorXCore> {
  @override
  void onReady() async {
    super.onReady();
    initMirrorXSDK();
  }

  Future<void> initMirrorXSDK() async {
    change(null, status: RxStatus.loading());

    try {
      final MirrorXCore core = MirrorXCoreImpl(_openLibrary());

      final applicationSupportDir = await getApplicationSupportDirectory();
      log("application support dir: $applicationSupportDir");

      await core.init(configDbPath: applicationSupportDir.path);

      listenStream(core.initFlutterCommandStreamSink());

      await core.deviceGoesOnline();

      change(core, status: RxStatus.success());
    } catch (e) {
      change(null, status: RxStatus.error(e.toString()));
    }
  }

  MirrorXCore getSDKInstance() {
    if (state == null) {
      throw Exception("get sdk instance but it's null");
    }
    return state!;
  }

  Future<void> listenStream(Stream<FlutterCommand> stream) async {
    stream.listen((event) {
      switch (event) {
        case FlutterCommand.PopupDesktopConnectInputPasswordDialog:
          break;
      }
    });
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

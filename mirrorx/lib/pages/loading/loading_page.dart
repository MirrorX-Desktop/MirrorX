import 'dart:developer';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:mirrorx/env/sdk/mirrorx_core_sdk.dart';
import 'package:mirrorx/pages/main/main_page.dart';
import 'package:path_provider/path_provider.dart';

class LoadingPage extends StatelessWidget {
  const LoadingPage({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return FutureBuilder(
      future: _initMirrorXCore(),
      builder: (context, snapshot) {
        switch (snapshot.connectionState) {
          case ConnectionState.none:
          case ConnectionState.waiting:
          case ConnectionState.active:
            return const Center(child: CircularProgressIndicator());
          case ConnectionState.done:
            if (snapshot.hasError) {
              log('Error: ${snapshot.error}');
              return Center(child: Text(snapshot.error.toString()));
            } else {
              return const MainPage();
            }
        }
      },
    );
  }

  Future<void> _initMirrorXCore() async {
    try {
      final applicationSupportDir = await getApplicationSupportDirectory();
      log("prepare init: applicationSupportDir='${applicationSupportDir.path}' osName='${Platform.operatingSystem}' osVersion='${Platform.operatingSystemVersion}'");

      await MirrorXCoreSDK.instance.init(
        osName: Platform.operatingSystem,
        osVersion: Platform.operatingSystemVersion,
        configDir: applicationSupportDir.path,
      );

      await MirrorXCoreSDK.instance.signalingHandshake();
    } catch (error) {
      return Future.error(error);
    }
  }
}

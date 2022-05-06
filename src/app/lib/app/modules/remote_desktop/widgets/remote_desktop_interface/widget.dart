import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:mirrorx/app/modules/remote_desktop/widgets/remote_desktop_interface/controller.dart';

class RemoteDesktopInterface extends GetView<RemoteDesktopInterfaceController> {
  const RemoteDesktopInterface({Key? key, required String remoteID})
      : _remoteID = remoteID,
        super(key: key);

  final String _remoteID;

  @override
  String? get tag => _remoteID;

  @override
  Widget build(BuildContext context) {
    var _controller =
        Get.put(RemoteDesktopInterfaceController(_remoteID), tag: _remoteID);

    return FutureBuilder(
        future: _controller.registerTexture(),
        builder: (context, snapshot) {
          if (snapshot.connectionState != ConnectionState.done) {
            return const CircularProgressIndicator();
          } else {
            if (snapshot.hasError) {
              return Text(snapshot.error.toString());
            } else {
              return RepaintBoundary(
                  child: Texture(textureId: snapshot.data as int));
            }
          }
        });
  }
}

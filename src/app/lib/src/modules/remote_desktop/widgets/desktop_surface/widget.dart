import 'package:flutter/material.dart';
import 'package:get/get.dart';

import 'controller.dart';

class DesktopSurface extends GetView<DesktopSurfaceController> {
  const DesktopSurface({Key? key, required String remoteID})
      : _remoteID = remoteID,
        super(key: key);

  final String _remoteID;

  @override
  String? get tag => _remoteID;

  @override
  Widget build(BuildContext context) {
    var _controller =
        Get.put(DesktopSurfaceController(_remoteID), tag: _remoteID);

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
              child: Center(
                child: AspectRatio(
                  aspectRatio: 16.0 / 9.0,
                  child: Texture(
                    textureId: snapshot.data as int,
                    freeze: true,
                    filterQuality: FilterQuality.none,
                  ),
                ),
              ),
            );
          }
        }
      },
    );
  }
}

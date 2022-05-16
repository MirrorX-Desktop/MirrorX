import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:get/get.dart';
import 'package:app/src/modules/remote_desktop/widgets/remote_desktop_interface/widget.dart';

import 'controller.dart';

class RemoteDesktopPage extends GetView<RemoteDesktopController> {
  const RemoteDesktopPage({
    Key? key,
    required String remoteID,
    required String osName,
    required String osVersion,
  })  : _remoteID = remoteID,
        _osName = osName,
        _osVersion = osVersion,
        super(key: key);

  final String _remoteID;
  final String _osName;
  final String _osVersion;

  @override
  String? get tag => _remoteID;

  String get osName => _osName;
  String get osVersion => _osVersion;

  @override
  Widget build(BuildContext context) {
    // final textureID = await _channel.invokeMethod<int>("get_texture_id");

    return Column(
      children: [
        Row(
          children: [
            TextButton(onPressed: () {}, child: Text("AAA")),
            TextButton(onPressed: () {}, child: Text("BBB"))
          ],
        ),
        Expanded(
            child: Container(
          child: RemoteDesktopInterface(remoteID: _remoteID),
          // child: RepaintBoundary(
          //     child: Texture(
          //   textureId: textureID!,
          //   filterQuality: FilterQuality.none,
          // )),
        ))
      ],
    );
  }
}

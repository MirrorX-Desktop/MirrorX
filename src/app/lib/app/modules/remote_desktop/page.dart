import 'package:flutter/material.dart';
import 'package:get/get.dart';

import 'controller.dart';

class RemoteDesktopPage extends GetView<RemoteDesktopController> {
  const RemoteDesktopPage({
    Key? key,
    required String remoteID,
  })  : _remoteID = remoteID,
        super(key: key);

  final String _remoteID;

  @override
  String? get tag => _remoteID;

  @override
  Widget build(BuildContext context) {
    return const Text("RemoteDesktop");
  }
}

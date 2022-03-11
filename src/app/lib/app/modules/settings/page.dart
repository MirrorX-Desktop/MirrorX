import 'package:flutter/material.dart';
import 'package:get/get.dart';

import 'controller.dart';

class SettingsPage extends GetView<SettingsController> {
  const SettingsPage({Key? key, required String staticTag})
      : _staticTag = staticTag,
        super(key: key);

  final String _staticTag;

  @override
  String? get tag => _staticTag;

  @override
  Widget build(BuildContext context) {
    return const Text('SettingsPage');
  }
}

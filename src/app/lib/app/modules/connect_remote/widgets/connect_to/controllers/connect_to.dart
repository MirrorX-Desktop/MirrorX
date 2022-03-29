import 'dart:developer';

import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:mirrorx/app/controllers/sdk.dart';
import 'package:mirrorx/app/core/utils/dialog.dart';
import 'package:mirrorx/app/modules/connect_remote/widgets/connect_to/controllers/chars_input.dart';

class ConnectToController extends GetxController {
  late CharacterInputController _digitInputController;
  late SDKController _sdk;

  bool _isLoading = false;

  bool get isLoading => _isLoading;

  @override
  void onInit() {
    _digitInputController = Get.put(CharacterInputController());
    _sdk = Get.find();
    super.onInit();
  }

  Future<void> connectTo() async {
    final deviceID = _digitInputController.inputText;
    if (deviceID == null || deviceID.isEmpty) {
      popupErrorDialog(content: "connect_to_remote.dialog.empty_input".tr);
      return;
    }

    if (deviceID.length != 8) {
      popupErrorDialog(content: "connect_to_remote.dialog.invalid_length".tr);
      return;
    }

    if (!RegExp(r'^[1-9a-hjkmnp-zA-HJKMNP-Z]+$').hasMatch(deviceID)) {
      popupErrorDialog(content: "connect_to_remote.dialog.invalid_char".tr);
      return;
    }

    final deviceRunesList = deviceID.runes.toList();

    for (var ch in deviceRunesList) {
      if (deviceRunesList.indexOf(ch) != deviceRunesList.lastIndexOf(ch)) {
        popupErrorDialog(
            content: "connect_to_remote.dialog.invalid_format.repeat_char".tr);
        return;
      }
    }

    try {
      _isLoading = true;
      update();

      final allow =
          await _sdk.getSDKInstance().desktopConnectTo(askDeviceId: deviceID);

      if (allow) {
        _popupDesktopConnectInputPasswordDialog(deviceID);
      }

      log(allow.toString());
    } catch (err) {
      log(err.toString());
    } finally {
      _isLoading = false;
      update();
    }
  }
}

void _popupDesktopConnectInputPasswordDialog(String deviceID) {
  Get.defaultDialog(
      title: "MirrorX",
      titleStyle: const TextStyle(fontSize: 16),
      content: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Padding(
            padding: const EdgeInsets.only(bottom: 8.0),
            child: Text(
              "请输入设备[$deviceID]的访问密码",
              textAlign: TextAlign.center,
            ),
          ),
          const CupertinoTextField(
            textAlign: TextAlign.center,
            maxLength: 16,
            maxLines: 1,
          ),
        ],
      ),
      contentPadding: const EdgeInsets.fromLTRB(16, 8, 16, 8),
      barrierDismissible: false,
      radius: 12,
      actions: [
        TextButton(
            onPressed: () {
              Get.back(closeOverlays: true);
            },
            child: Text("dialog.ok".tr)),
        TextButton(
            onPressed: () {
              Get.back(closeOverlays: true);
            },
            child: Text("dialog.cancel".tr))
      ]);
}

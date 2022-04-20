import 'dart:developer';

import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:mirrorx/app/controllers/mirrorx_core.dart';
import 'package:mirrorx/app/core/utils/dialog.dart';
import 'package:mirrorx/app/modules/connect_remote/widgets/connect_to/controllers/chars_input.dart';

class ConnectToController extends GetxController {
  late CharacterInputController _digitInputController;
  late MirrorXCoreController _sdk;

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

      await _sdk.getInstance().serviceDesktopConnect(askDeviceId: deviceID);
      _popupDesktopConnectInputPasswordDialog(deviceID);
    } catch (err) {
      popupErrorDialog(content: "connect_to_remote.dialog.disallow".tr);
    } finally {
      _isLoading = false;
      update();
    }
  }

  Future<void> authPassword(
      TextEditingController controller, String deviceID) async {
    if (controller.text.isEmpty) {
      return;
    }

    try {
      await _sdk.getInstance().serviceDesktopKeyExchangeAndPasswordVerify(
          askDeviceId: deviceID, password: controller.text);

      // log("password: $passwordCorrect");
    } catch (err) {
      popupErrorDialog(
          content: "connect_to_remote.dialog.incorrect_password".tr);
    }
  }

  void _popupDesktopConnectInputPasswordDialog(String deviceID) {
    final passwordTextController = TextEditingController();

    Get.defaultDialog(
        title: "MirrorX",
        titleStyle: const TextStyle(fontSize: 18),
        content: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Padding(
              padding: const EdgeInsets.only(bottom: 8.0),
              child: Text(
                "请输入设备[$deviceID]的访问密码",
                textAlign: TextAlign.center,
                style: const TextStyle(fontSize: 16),
              ),
            ),
            CupertinoTextField(
              controller: passwordTextController,
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
              onPressed: () async {
                Get.back(closeOverlays: true);
                await authPassword(passwordTextController, deviceID);
              },
              child: Text("dialog.ok".tr)),
          TextButton(
              onPressed: () {
                Get.back(closeOverlays: true);
              },
              child: Text("dialog.cancel".tr))
        ]);
  }
}

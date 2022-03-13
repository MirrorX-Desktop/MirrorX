import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:mirrorx/app/controllers/sdk.dart';
import 'package:mirrorx/app/core/utils/dialog.dart';

class DevicePasswordController extends GetxController {
  var _visable = false;
  var _editing = false;
  var _password = "";
  final _sdkController = Get.find<SDKController>();
  final _textController = TextEditingController();

  bool get passwordVisable => _visable;
  bool get isEditing => _editing;
  String get password => _password;
  TextEditingController get textController => _textController;

  @override
  void onReady() async {
    await fetchDevicePassword();
    super.onReady();
  }

  Future<void> fetchDevicePassword() async {
    String password;

    try {
      final storedPassword = await _sdkController
          .getSDKInstance()
          .readConfig(key: "device_password");

      if (storedPassword == null || storedPassword == "") {
        password = await _generateAndSaveNewDevicePassword();
      }

      password = storedPassword!;
    } catch (err) {
      password = await _generateAndSaveNewDevicePassword();
    }

    _password = password;
    _textController.text = password;
    update();
  }

  void changeVisable() {
    _visable = !_visable;
    update();
  }

  Future<void> editOrCommitPassword() async {
    if (_editing) {
      if (_textController.text.length >= 8 &&
          _textController.text.length <= 16) {
        if (_textController.text == _password) {
          return;
        }

        try {
          await _sdkController
              .getSDKInstance()
              .storeConfig(key: "device_password", value: _textController.text);
          _password = _textController.text;
        } catch (err) {
          Get.defaultDialog(
              title: "MirrorX",
              content: Text("An error occurs when save password: $err"));
        }
      } else {
        return;
      }
    }

    _visable = false;
    _editing = !_editing;
    update();
  }

  void cancelEditing() {
    _visable = false;
    _editing = false;
    update();
  }

  void generateNewRandomPassword() async {
    if (!_editing) {
      /// popup a dialog to alert
      popupAskDialog(
          content: "device_password_field.dialog.confirm_regenerate".tr,
          yesAction: () async {
            try {
              _password = await _generateAndSaveNewDevicePassword();
              textController.text = _password;
              update();
            } catch (err) {
              popupErrorDialog(content: "An error occurred.");
            }
          });
    } else {
      textController.text =
          await _sdkController.getSDKInstance().generateDevicePassword();
      update();
    }
  }

  Future<String> _generateAndSaveNewDevicePassword() async {
    final password =
        await _sdkController.getSDKInstance().generateDevicePassword();
    await _sdkController
        .getSDKInstance()
        .storeConfig(key: "device_password", value: password);
    return password;
  }

  @override
  void onClose() {
    _textController.dispose();
    super.onClose();
  }
}

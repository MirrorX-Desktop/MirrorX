import 'dart:developer';

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

      final res =
          await _sdk.getSDKInstance().desktopConnectTo(askDeviceId: deviceID);

      log(res.toString());
    } catch (err) {
      log(err.toString());
    } finally {
      _isLoading = false;
      update();
    }
  }
}

import 'package:flutter/material.dart';
import 'package:get/get.dart';

void popupAskDialog({required String content, Function()? yesAction}) {
  Get.defaultDialog(
      title: "MirrorX",
      titleStyle: const TextStyle(fontSize: 18),
      middleText: content,
      middleTextStyle: const TextStyle(fontSize: 16),
      contentPadding: const EdgeInsets.fromLTRB(16, 8, 16, 8),
      barrierDismissible: false,
      radius: 12,
      actions: [
        TextButton(
            onPressed: () {
              if (yesAction != null) {
                yesAction();
              }
              Get.back(closeOverlays: true);
            },
            child: Text("dialog.yes".tr)),
        TextButton(
            onPressed: () {
              Get.back(closeOverlays: true);
            },
            child: Text("dialog.no".tr))
      ]);
}

void popupErrorDialog({required String content}) {
  Get.defaultDialog(
      title: "MirrorX Error",
      titleStyle: const TextStyle(fontSize: 18),
      middleText: content,
      middleTextStyle: const TextStyle(fontSize: 16),
      contentPadding: const EdgeInsets.fromLTRB(16, 8, 16, 8),
      barrierDismissible: false,
      radius: 12,
      actions: [
        TextButton(
            onPressed: () {
              Get.back(closeOverlays: true);
            },
            child: Text("dialog.ok".tr))
      ]);
}

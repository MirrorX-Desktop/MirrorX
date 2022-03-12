import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:mirrorx/app/modules/connect_remote/widgets/device_password/controller.dart';

class DevicePassword extends StatelessWidget {
  const DevicePassword({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    Get.put(DevicePasswordController());

    return SizedBox(
      height: 48,
      child: GetBuilder<DevicePasswordController>(
        builder: (controller) => Row(
          children: [
            Expanded(
              child: _buildEditingOrReadonly(controller),
            ),
            SizedBox(
              height: 32,
              width: 32,
              child: IconButton(
                tooltip: controller.passwordVisable
                    ? "connect_to_remote.device_password_hide".tr
                    : "connect_to_remote.device_password_show".tr,
                onPressed:
                    controller.isEditing ? null : controller.changeVisable,
                splashRadius: 14,
                splashColor: Colors.transparent,
                padding: EdgeInsets.zero,
                hoverColor: const Color.fromARGB(240, 220, 220, 220),
                icon: Icon(
                  controller.passwordVisable
                      ? Icons.visibility_off
                      : Icons.visibility,
                  size: 16,
                ),
              ),
            ),
            SizedBox(
              height: 32,
              width: 32,
              child: IconButton(
                tooltip: controller.isEditing
                    ? "connect_to_remote.device_password_edit_confirm".tr
                    : "connect_to_remote.device_password_edit".tr,
                onPressed: controller.editing,
                splashRadius: 14,
                splashColor: Colors.transparent,
                padding: EdgeInsets.zero,
                hoverColor: const Color.fromARGB(240, 220, 220, 220),
                icon: Icon(
                  controller.isEditing ? Icons.check : Icons.edit,
                  size: 16,
                ),
              ),
            ),
            SizedBox(
              height: 32,
              width: 32,
              child: IconButton(
                tooltip: "connect_to_remote.device_password_random_generate".tr,
                onPressed: () {},
                splashRadius: 14,
                splashColor: Colors.transparent,
                padding: EdgeInsets.zero,
                hoverColor: const Color.fromARGB(240, 220, 220, 220),
                icon: const Icon(
                  Icons.lock_reset,
                  size: 16,
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildEditingOrReadonly(DevicePasswordController controller) {
    if (!controller.isEditing) {
      return Center(
        child: Text(
          controller.passwordVisable ? controller.password : "＊＊＊＊＊＊",
          style: const TextStyle(fontSize: 16),
        ),
      );
    } else {
      return TextField(
        maxLength: 16,
        maxLines: 1,
        decoration: InputDecoration(
          counterText: "",
        ),
        scrollPhysics: NeverScrollableScrollPhysics(),
        textAlign: TextAlign.center,
        style: TextStyle(fontSize: 16),
      );
    }
  }
}

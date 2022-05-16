import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:app/src/modules/connect_remote/widgets/device_id/controller.dart';

class DeviceID extends GetView<DeviceIDController> {
  const DeviceID({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final controller = Get.put(DeviceIDController());

    return controller.obx(
      _buildDeviceIDField,
      onLoading: _buildLoading(),
      onError: (error) => _buildRetryButton(),
    );
  }

  Widget _buildDeviceIDField(String? deviceId) {
    return SizedBox(
      height: 48,
      child: Row(
        children: [
          Expanded(child: DigitsBoard(deviceId)),
          SizedBox(
            height: 32,
            width: 32,
            child: IconButton(
              onPressed: () {
                Get.showSnackbar(
                  GetSnackBar(
                    duration: const Duration(milliseconds: 1500),
                    message: "connect_to_remote.device_id_copy_tooltip".tr,
                  ),
                );
              },
              padding: EdgeInsets.zero,
              tooltip: "connect_to_remote.device_id_copy".tr,
              splashRadius: 14,
              splashColor: Colors.transparent,
              hoverColor: const Color.fromARGB(240, 220, 220, 220),
              iconSize: 16,
              icon: const Icon(Icons.content_copy),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildRetryButton() {
    return SizedBox(
      height: 48,
      child: Row(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          IconButton(
            padding: EdgeInsets.zero,
            onPressed: controller.loadDeviceId,
            tooltip: "device_id_field.load_failed_tooltip".tr,
            color: Colors.red,
            splashRadius: 18,
            splashColor: Colors.transparent,
            hoverColor: const Color.fromARGB(240, 220, 220, 220),
            iconSize: 22,
            icon: const Icon(Icons.warning),
          ),
        ],
      ),
    );
  }

  Widget _buildLoading() {
    return SizedBox(
      height: 48,
      child: Row(
        mainAxisAlignment: MainAxisAlignment.center,
        children: const [
          SizedBox(
            width: 36,
            height: 36,
            child: CircularProgressIndicator(),
          ),
        ],
      ),
    );
  }
}

class DigitsBoard extends StatelessWidget {
  const DigitsBoard(String? digitsStr, {Key? key})
      : _digitsStr = digitsStr,
        super(key: key);

  final String? _digitsStr;

  @override
  Widget build(BuildContext context) {
    final displayDigits = (_digitsStr ?? "").padLeft(8, '0');

    return Row(
      mainAxisAlignment: MainAxisAlignment.center,
      children: displayDigits.characters
          .map((e) => _buildDigitPanel(context, e))
          .toList(),
    );
  }

  Widget _buildDigitPanel(BuildContext context, String digitChar) {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 3.0),
      child: SizedBox(
        width: 24,
        child: TextField(
          controller: TextEditingController.fromValue(
              TextEditingValue(text: digitChar)),
          maxLength: 1,
          textAlign: TextAlign.center,
          textAlignVertical: TextAlignVertical.center,
          autocorrect: false,
          keyboardType: TextInputType.number,
          decoration: const InputDecoration(
              counterText: "",
              disabledBorder:
                  UnderlineInputBorder(borderSide: BorderSide(width: 2))),
          enabled: false,
          enableIMEPersonalizedLearning: false,
          enableSuggestions: false,
          enableInteractiveSelection: false,
          style: const TextStyle(fontSize: 24),
        ),
      ),
    );
  }
}

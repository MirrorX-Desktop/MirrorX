import 'dart:math';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:get/get.dart';
import 'package:mirrorx/app/controllers/page_view.dart';
import 'package:mirrorx/app/core/values/colors.dart';
import 'package:mirrorx/app/modules/connect_remote/widgets/connect_to/controller.dart';

class ConnectTo extends StatelessWidget {
  const ConnectTo({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 48,
      child: Row(
        children: [
          const Expanded(child: DigitsInputField()),
          SizedBox(
            height: 32,
            width: 32,
            child: IconButton(
                onPressed: _connectTo,
                tooltip: "connect_to_remote.connect".tr,
                splashRadius: 14,
                splashColor: Colors.transparent,
                padding: EdgeInsets.zero,
                hoverColor: const Color.fromARGB(240, 220, 220, 220),
                iconSize: 16,
                icon: const Icon(Icons.login)),
          ),
        ],
      ),
    );
  }

  void _connectTo() {
    final id = 100000 + Random().nextInt(899999);
    final controller = Get.find<PageViewController>();
    controller.addNewRemoteDesktopPage(id.toString());
  }
}

class DigitsInputField extends StatelessWidget {
  const DigitsInputField({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    Get.put(DigitInputController());

    return GetBuilder<DigitInputController>(
      builder: (controller) => FocusScope(
        node: controller.focusScopeNode,
        child: Row(
            mainAxisAlignment: MainAxisAlignment.center,
            children: _buildDigitInputFields(controller)),
      ),
    );
  }

  List<Widget> _buildDigitInputFields(DigitInputController controller) {
    return controller.textEditingControllers
        .map((e) =>
            _buildDigitInputField(controller.textEditingControllers.indexOf(e)))
        .toList();
  }

  Widget _buildDigitInputField(int controllerIndex) {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 3.0),
      child: SizedBox(
        width: 24,
        child: GetBuilder<DigitInputController>(
          builder: (controller) {
            final textEditingController =
                controller.textEditingControllers.elementAt(controllerIndex);

            return TextField(
              controller: textEditingController,
              maxLength: 1,
              textAlign: TextAlign.center,
              autocorrect: false,
              keyboardType: TextInputType.number,
              inputFormatters: [
                FilteringTextInputFormatter.digitsOnly,
              ],
              textAlignVertical: TextAlignVertical.center,
              decoration: InputDecoration(
                counterText: "",
                enabledBorder: UnderlineInputBorder(
                    borderSide: BorderSide(
                        color: textEditingController.text.length == 1
                            ? ColorValues.primaryColor
                            : Colors.grey,
                        width: 2)),
                focusedBorder: const UnderlineInputBorder(
                    borderSide:
                        BorderSide(color: ColorValues.primaryColor, width: 2)),
              ),
              enableIMEPersonalizedLearning: false,
              enableSuggestions: false,
              enableInteractiveSelection: false,
              cursorColor: ColorValues.primaryColor,
              style: const TextStyle(fontSize: 24),
              scrollPhysics: const NeverScrollableScrollPhysics(),
            );
          },
        ),
      ),
    );
  }
}

import 'dart:math';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:get/get.dart';
import 'package:mirrorx/app/controllers/page_view.dart';
import 'package:mirrorx/app/core/values/colors.dart';
import 'package:mirrorx/app/modules/connect_remote/widgets/connect_to/controllers/connect_to.dart';
import 'package:mirrorx/app/modules/connect_remote/widgets/connect_to/controllers/chars_input.dart';

class ConnectTo extends StatelessWidget {
  const ConnectTo({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final controller = Get.put(ConnectToController());

    return SizedBox(
      height: 48,
      child: Row(
        children: [
          const Expanded(child: CharacterInputField()),
          SizedBox(
            height: 32,
            width: 32,
            child: GetBuilder<ConnectToController>(
              builder: (controller) => Visibility(
                visible: !controller.isLoading,
                child: IconButton(
                  onPressed: controller.connectTo,
                  tooltip: "connect_to_remote.connect".tr,
                  splashRadius: 14,
                  splashColor: Colors.transparent,
                  padding: EdgeInsets.zero,
                  hoverColor: const Color.fromARGB(240, 220, 220, 220),
                  iconSize: 16,
                  icon: const Icon(Icons.login),
                ),
                replacement: const CircularProgressIndicator(),
              ),
            ),
          ),
        ],
      ),
    );
  }
}

class CharacterInputField extends StatelessWidget {
  const CharacterInputField({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return GetBuilder<CharacterInputController>(
      builder: (controller) => FocusScope(
        node: controller.focusScopeNode,
        child: Row(
            mainAxisAlignment: MainAxisAlignment.center,
            children: _buildDigitInputFields(controller)),
      ),
    );
  }

  List<Widget> _buildDigitInputFields(CharacterInputController controller) {
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
        child: GetBuilder<CharacterInputController>(
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
                FilteringTextInputFormatter.allow(
                    RegExp(r'[1-9a-hjkmnp-zA-HJKMNP-Z]')),
                UpperCaseTextFormatter(),
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

class UpperCaseTextFormatter extends TextInputFormatter {
  @override
  TextEditingValue formatEditUpdate(
      TextEditingValue oldValue, TextEditingValue newValue) {
    return TextEditingValue(
      text: newValue.text.toUpperCase(),
      selection: newValue.selection,
    );
  }
}

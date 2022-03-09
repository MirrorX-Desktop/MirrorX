import 'dart:math';

import 'package:easy_localization/easy_localization.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:mirrorx/components/navigator/navigator.dart';
import 'package:mirrorx/constants.dart';
import 'package:provider/provider.dart';

final _allowInputKey = [
  LogicalKeyboardKey.numpad0,
  LogicalKeyboardKey.numpad1,
  LogicalKeyboardKey.numpad2,
  LogicalKeyboardKey.numpad3,
  LogicalKeyboardKey.numpad4,
  LogicalKeyboardKey.numpad5,
  LogicalKeyboardKey.numpad6,
  LogicalKeyboardKey.numpad7,
  LogicalKeyboardKey.numpad8,
  LogicalKeyboardKey.numpad9,
];

class ConnectRemoteField extends StatefulWidget {
  const ConnectRemoteField({Key? key}) : super(key: key);

  @override
  _ConnectRemoteFieldState createState() => _ConnectRemoteFieldState();
}

class _ConnectRemoteFieldState extends State<ConnectRemoteField> {
  late AppNavigator _appNavigator;

  @override
  void initState() {
    _appNavigator = Provider.of(context, listen: false);
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 40,
      child: Row(
        children: [
          const Expanded(child: NumericInput()),
          IconButton(
              onPressed: _mouseClick,
              tooltip: tr("connect_to_remote.connect"),
              splashRadius: 14,
              splashColor: Colors.transparent,
              hoverColor: const Color.fromARGB(240, 220, 220, 220),
              icon: const Icon(
                Icons.login,
                size: 16,
              )),
        ],
      ),
    );
  }

  void _mouseClick() {
    final id = 100000 + Random().nextInt(899999);
    _appNavigator.addAndJumpRemoteDesktopPage(id.toString());
  }

  @override
  void dispose() {
    super.dispose();
  }
}

class NumericInput extends StatefulWidget {
  const NumericInput({Key? key}) : super(key: key);

  @override
  State<StatefulWidget> createState() => _NumericInputState();
}

class _NumericInputState extends State<NumericInput> {
  late FocusScopeNode _foucsScopeNode;
  final _controllers = <TextEditingController>[];

  @override
  void initState() {
    for (int i = 0; i < 8; i++) {
      final controller = TextEditingController();
      _controllers.add(controller);
    }

    _foucsScopeNode = FocusScopeNode(onKeyEvent: (node, event) {
      if (event is KeyDownEvent) {
        // for delete
        if (event.logicalKey == LogicalKeyboardKey.delete ||
            event.logicalKey == LogicalKeyboardKey.backspace) {
          for (int i = 0; i < 8; i++) {
            if (_foucsScopeNode.focusedChild ==
                _foucsScopeNode.children.elementAt(i)) {
              final controller = _controllers[i];
              if (controller.text.isEmpty &&
                  _foucsScopeNode.focusedChild !=
                      _foucsScopeNode.children.first) {
                _foucsScopeNode.previousFocus();
                return KeyEventResult.handled;
              }
              break;
            }
          }
        }
      } else if (event is KeyUpEvent) {
        // for input
        if (_allowInputKey.any((element) => element == event.logicalKey)) {
          for (int i = 0; i < 8; i++) {
            if (_foucsScopeNode.focusedChild ==
                _foucsScopeNode.children.elementAt(i)) {
              final controller = _controllers[i];
              if (controller.text.isNotEmpty &&
                  _foucsScopeNode.focusedChild !=
                      _foucsScopeNode.children.last) {
                _foucsScopeNode.nextFocus();
                return KeyEventResult.handled;
              }
              break;
            }
          }
        }
      }

      return KeyEventResult.ignored;
    });

    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    var inputFields = <Widget>[];

    for (int i = 0; i < 8; i++) {
      final inputField =
          _singleNumericInput(context, _controllers.elementAt(i));
      inputFields.add(inputField);
    }

    return FocusScope(
      node: _foucsScopeNode,
      child: Row(
          mainAxisAlignment: MainAxisAlignment.center, children: inputFields),
    );
  }

  Widget _singleNumericInput(
      BuildContext context, TextEditingController controller) {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 2.0),
      child: SizedBox(
        width: 24,
        height: 37,
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            CupertinoTextField(
              controller: controller,
              maxLength: 1,
              textAlign: TextAlign.center,
              autocorrect: false,
              keyboardType: TextInputType.number,
              inputFormatters: [
                FilteringTextInputFormatter.digitsOnly,
              ],
              cursorColor: primaryColor,
              style: const TextStyle(fontSize: 18),
            ),
          ],
        ),
      ),
    );
  }

  @override
  void dispose() {
    _foucsScopeNode.dispose();
    super.dispose();
  }
}

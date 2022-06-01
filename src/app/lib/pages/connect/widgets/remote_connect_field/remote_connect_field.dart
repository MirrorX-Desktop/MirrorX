import 'package:app/pages/connect/widgets/remote_connect_field/digit_input.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

class RemoteConnectField extends StatefulWidget {
  const RemoteConnectField({Key? key}) : super(key: key);

  @override
  _RemoteConnectFieldState createState() => _RemoteConnectFieldState();
}

class _RemoteConnectFieldState extends State<RemoteConnectField> {
  final List<TextEditingController> _textControllers = [];
  late FocusScopeNode _focusScopeNode;

  @override
  void initState() {
    super.initState();
    _focusScopeNode = FocusScopeNode(
      onKeyEvent: ((node, event) {
        if (event.logicalKey == LogicalKeyboardKey.delete ||
            event.logicalKey == LogicalKeyboardKey.backspace) {
          final scopeNode = node as FocusScopeNode;
          if (scopeNode.focusedChild != null) {
            final index =
                scopeNode.children.toList().indexOf(scopeNode.focusedChild!);
            if (index > 0 && _textControllers[index].text.isEmpty) {
              scopeNode.previousFocus();
            }
          }
        }

        return KeyEventResult.ignored;
      }),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 12.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            "Connect Remote",
            style: TextStyle(fontSize: 27),
          ),
          FocusScope(
            node: _focusScopeNode,
            child: Row(
              children: [
                _createField(),
                const VerticalDivider(width: 6),
                _createField(),
                const VerticalDivider(width: 6),
                const Text("-", style: TextStyle(fontSize: 36)),
                const VerticalDivider(width: 6),
                _createField(),
                const VerticalDivider(width: 6),
                _createField(),
                const VerticalDivider(width: 6),
                _createField(),
                const VerticalDivider(width: 6),
                const Text("-", style: TextStyle(fontSize: 36)),
                const VerticalDivider(width: 6),
                _createField(),
                const VerticalDivider(width: 6),
                _createField(),
                const VerticalDivider(width: 6),
                _createField(),
              ],
            ),
          )
        ],
      ),
    );
  }

  Widget _createField() {
    final controller = TextEditingController();
    _textControllers.add(controller);

    return SizedBox(
        width: 38,
        child: DigitInput(
          textEditingController: controller,
          focusNode: _focusScopeNode,
        ));
  }
}

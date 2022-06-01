import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';

class DigitInput extends StatefulWidget {
  const DigitInput(
      {Key? key, required this.textEditingController, required this.focusNode})
      : super(key: key);

  final TextEditingController textEditingController;
  final FocusScopeNode focusNode;

  @override
  _DigitInputState createState() => _DigitInputState();
}

class _DigitInputState extends State<DigitInput> {
  Color _borderColor = Colors.black;

  @override
  Widget build(BuildContext context) {
    return CupertinoTextField(
      controller: widget.textEditingController,
      cursorColor: Colors.yellow,
      decoration: BoxDecoration(
        border: Border(
          bottom: BorderSide(
            color: _borderColor,
            width: 2,
          ),
        ),
      ),
      onChanged: (text) {
        setState(() {
          if (text.isEmpty) {
            _borderColor = Colors.black;
          } else {
            _borderColor = Colors.yellow;
            if (widget.focusNode.focusedChild !=
                widget.focusNode.children.last) {
              widget.focusNode.nextFocus();
            }
          }
        });
      },
      keyboardType: TextInputType.number,
      textInputAction: TextInputAction.next,
      textAlign: TextAlign.center,
      textAlignVertical: TextAlignVertical.center,
      style: const TextStyle(fontSize: 36),
      enableSuggestions: false,
      maxLength: 1,
    );
  }
}

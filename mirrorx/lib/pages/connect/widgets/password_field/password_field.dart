import 'package:mirrorx/business/mirrorx_core/mirrorx_core_bloc.dart';
import 'package:mirrorx/env/langs/tr.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

class PasswordField extends StatefulWidget {
  const PasswordField({Key? key}) : super(key: key);

  @override
  _PasswordFieldState createState() => _PasswordFieldState();
}

class _PasswordFieldState extends State<PasswordField> {
  bool _isVisible = false;
  bool _isEditing = false;
  final _controller = TextEditingController();

  @override
  Widget build(BuildContext context) {
    return Container(
      height: 110,
      width: 360,
      decoration: const BoxDecoration(
        border: Border(left: BorderSide(color: Colors.yellow, width: 4)),
      ),
      child: Padding(
        padding: const EdgeInsets.only(left: 12.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Text(
                  Tr.of(context).connectPagePasswordTitle,
                  style: const TextStyle(fontSize: 27),
                ),
                _buildTopButton(),
              ],
            ),
            Expanded(
              child: Row(
                crossAxisAlignment: CrossAxisAlignment.center,
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  BlocBuilder<MirrorXCoreBloc, MirrorXCoreState>(
                      builder: (context, state) =>
                          _buildPasswordField(context, state)),
                  _buildBottomButton(),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildPasswordField(BuildContext context, MirrorXCoreState state) {
    if (state.password == null) {
      context
          .read<MirrorXCoreBloc>()
          .add(MirrorXCoreConfigReadDevicePassword());
      return const Center(child: CircularProgressIndicator());
    }

    if (_isEditing) {
      _controller.text = state.password!;
      return SizedBox(
        width: 300,
        child: CupertinoTextField(
          controller: _controller,
          cursorColor: Colors.yellow,
          decoration: const BoxDecoration(
            border: Border(
              bottom: BorderSide(
                color: Colors.black,
                width: 2,
              ),
            ),
          ),
          inputFormatters: [
            FilteringTextInputFormatter.allow(RegExp(r'[a-zA-Z0-9]')),
          ],
          keyboardType: TextInputType.visiblePassword,
          textInputAction: TextInputAction.next,
          textAlign: TextAlign.center,
          textAlignVertical: TextAlignVertical.center,
          style: const TextStyle(fontSize: 36),
          enableSuggestions: false,
          maxLength: 8,
          maxLines: 1,
          autocorrect: false,
        ),
      );
    } else {
      if (_isVisible) {
        return Text(
          state.password!,
          style: const TextStyle(fontSize: 45),
        );
      } else {
        return const Text(
          "＊＊＊＊＊＊",
          style: TextStyle(fontSize: 45),
        );
      }
    }
  }

  Widget _buildTopButton() {
    final bloc = context.read<MirrorXCoreBloc>();
    return IconButton(
      onPressed: () {
        setState(() {
          if (_isEditing) {
            bloc.add(MirrorXCoreConfigSaveDevicePassword(
                devicePassword: _controller.text));
            _isVisible = false;
          }
          _isEditing = !_isEditing;
        });
      },
      icon: Icon(_isEditing ? Icons.check : Icons.edit),
      splashRadius: 20,
      hoverColor: Colors.yellow,
      tooltip: _isEditing
          ? Tr.of(context).connectPagePasswordButtonCommitTooltip
          : Tr.of(context).connectPagePasswordButtonEditTooltip,
    );
  }

  Widget _buildBottomButton() {
    final bloc = context.read<MirrorXCoreBloc>();
    return IconButton(
      onPressed: () {
        setState(() {
          if (!_isEditing) {
            _isVisible = !_isVisible;
          } else {
            bloc.add(MirrorXCoreGenerateDevicePassword());
            _isVisible = false;
          }
        });
      },
      splashRadius: 20,
      hoverColor: Colors.yellow,
      icon: Icon(_isEditing
          ? Icons.lock_reset
          : _isVisible
              ? Icons.visibility_off
              : Icons.visibility),
      tooltip: _isEditing
          ? Tr.of(context).connectPagePasswordButtonRandomGenerateTooltip
          : _isVisible
              ? Tr.of(context).connectPagePasswordVisibilityToggleHideTooltip
              : Tr.of(context).connectPagePasswordVisibilityToggleShowTooltip,
    );
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }
}

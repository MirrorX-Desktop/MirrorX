import 'dart:developer';

import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:font_awesome_flutter/font_awesome_flutter.dart';
import 'package:mirrorx/state/signaling_manager/signaling_manager_cubit.dart';

class DevicePasswordField extends StatefulWidget {
  const DevicePasswordField({Key? key}) : super(key: key);

  @override
  _DevicePasswordFieldState createState() => _DevicePasswordFieldState();
}

class _DevicePasswordFieldState extends State<DevicePasswordField> {
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
                  AppLocalizations.of(context)!.connectPagePasswordTitle,
                  style: const TextStyle(fontSize: 27),
                ),
                Expanded(
                  child: Row(
                    mainAxisSize: MainAxisSize.min,
                    mainAxisAlignment: MainAxisAlignment.end,
                    children: [
                      SizedBox(
                        width: 50,
                        child: _buildEditButton(),
                      ),
                      SizedBox(
                        width: 50,
                        child: _buildVisibilityOrGenPasswordButton(),
                      )
                    ],
                  ),
                )
              ],
            ),
            Expanded(
              child: _buildDevicePasswordField(),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildDevicePasswordField() {
    return BlocBuilder<SignalingManagerCubit, SignalingManagerState>(
      builder: (context, state) {
        if (state.domainConfig?.devicePassword == null) {
          return IconButton(
            onPressed: () {
              context.read<SignalingManagerCubit>().updateDevicePassword(null);
            },
            icon: const FaIcon(
              FontAwesomeIcons.arrowsRotate,
              size: 24,
            ),
          );
        }

        if (_isEditing) {
          _controller.text = state.domainConfig!.devicePassword;
          return TextFormField(
            controller: _controller,
            cursorColor: Colors.yellow,
            inputFormatters: [
              FilteringTextInputFormatter.allow(
                  RegExp(r'[a-zA-Z0-9@#$%^*?!=+<>(){}]')),
            ],
            decoration: const InputDecoration(
              isDense: true,
              focusedBorder: UnderlineInputBorder(
                borderSide: BorderSide(width: 2, color: Colors.yellow),
              ),
            ),
            style: const TextStyle(fontSize: 18),
            keyboardType: TextInputType.visiblePassword,
            textInputAction: TextInputAction.next,
            textAlign: TextAlign.center,
            textAlignVertical: TextAlignVertical.center,
            enableSuggestions: false,
            maxLength: 24,
            maxLines: 1,
            autocorrect: false,
            autovalidateMode: AutovalidateMode.always,
            validator: (text) {
              if (text == null || text.isEmpty || text.length < 8) {
                return AppLocalizations.of(context)!
                    .connectPagePasswordValidationErrorLength;
              }

              if (!RegExp(r'[A-Z]').hasMatch(text)) {
                return AppLocalizations.of(context)!
                    .connectPagePasswordValidationErrorUpper;
              }

              if (!RegExp(r'[@#$%^*?!=+<>(){}]').hasMatch(text)) {
                return AppLocalizations.of(context)!
                    .connectPagePasswordValidationErrorSpecial(
                  r'@#$%^*?!=+<>(){}',
                );
              }

              return null;
            },
          );
        } else {
          if (_isVisible) {
            return FittedBox(
              fit: BoxFit.fitWidth,
              child: SelectableText(
                state.domainConfig!.devicePassword,
                maxLines: 1,
                minLines: 1,
                scrollPhysics: const NeverScrollableScrollPhysics(),
              ),
            );
          } else {
            return Row(
              mainAxisAlignment: MainAxisAlignment.spaceEvenly,
              children: const [
                FaIcon(
                  FontAwesomeIcons.starOfLife,
                  size: 24,
                ),
                FaIcon(
                  FontAwesomeIcons.starOfLife,
                  size: 24,
                ),
                FaIcon(
                  FontAwesomeIcons.starOfLife,
                  size: 24,
                ),
                FaIcon(
                  FontAwesomeIcons.starOfLife,
                  size: 24,
                ),
                FaIcon(
                  FontAwesomeIcons.starOfLife,
                  size: 24,
                ),
                FaIcon(
                  FontAwesomeIcons.starOfLife,
                  size: 24,
                ),
                FaIcon(
                  FontAwesomeIcons.starOfLife,
                  size: 24,
                ),
              ],
            );
          }
        }
      },
    );
  }

  Widget _buildEditButton() {
    return IconButton(
      onPressed: () {
        setState(() {
          if (_isEditing) {
            context
                .read<SignalingManagerCubit>()
                .updateDevicePassword(_controller.text);
            _isVisible = false;
          }
          _isEditing = !_isEditing;
        });
      },
      icon: FaIcon(
        _isEditing ? FontAwesomeIcons.check : FontAwesomeIcons.penToSquare,
        size: 24,
      ),
      tooltip: _isEditing
          ? AppLocalizations.of(context)!.connectPagePasswordButtonCommitTooltip
          : AppLocalizations.of(context)!.connectPagePasswordButtonEditTooltip,
    );
  }

  Widget _buildVisibilityOrGenPasswordButton() {
    final icon = _isEditing
        ? FontAwesomeIcons.arrowsRotate
        : _isVisible
            ? FontAwesomeIcons.eyeSlash
            : FontAwesomeIcons.eye;

    final tooltip = _isEditing
        ? AppLocalizations.of(context)!
            .connectPagePasswordButtonRandomGenerateTooltip
        : _isVisible
            ? AppLocalizations.of(context)!
                .connectPagePasswordVisibilityToggleHideTooltip
            : AppLocalizations.of(context)!
                .connectPagePasswordVisibilityToggleShowTooltip;

    return IconButton(
      onPressed: () {
        setState(() {
          if (!_isEditing) {
            _isVisible = !_isVisible;
          } else {
            context.read<SignalingManagerCubit>().updateDevicePassword(null);
            _isVisible = false;
          }
        });
      },
      icon: FaIcon(icon, size: 24),
      tooltip: tooltip,
    );
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }
}

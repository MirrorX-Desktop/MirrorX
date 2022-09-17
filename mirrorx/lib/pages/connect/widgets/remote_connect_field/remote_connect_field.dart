import 'dart:developer';

import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:font_awesome_flutter/font_awesome_flutter.dart';
import 'package:mirrorx/env/utility/error_notifier.dart';
import 'package:mirrorx/pages/connect/widgets/remote_connect_field/digit_input.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:mirrorx/state/signaling_manager/signaling_manager_cubit.dart';

class RemoteConnectField extends StatefulWidget {
  const RemoteConnectField({Key? key}) : super(key: key);

  @override
  _RemoteConnectFieldState createState() => _RemoteConnectFieldState();
}

class _RemoteConnectFieldState extends State<RemoteConnectField> {
  final List<TextEditingController> _textControllers = [];
  late SnackBarNotifier _notifier;
  late FocusScopeNode _focusScopeNode;
  bool _connectButtonDisabled = true;
  bool _isVisitRequesting = false;

  @override
  void initState() {
    super.initState();
    _notifier = SnackBarNotifier(context);
    _focusScopeNode = FocusScopeNode(
      onKeyEvent: ((node, event) {
        if (event.logicalKey == LogicalKeyboardKey.delete ||
            event.logicalKey == LogicalKeyboardKey.backspace) {
          final scopeNode = node as FocusScopeNode;
          if (scopeNode.focusedChild != null) {
            final index =
                scopeNode.children.toList().indexOf(scopeNode.focusedChild!);

            if (_textControllers[index].text.isEmpty) {
              if (index > 0) {
                scopeNode.previousFocus();
              }
            } else {
              _textControllers[index].clear();
            }
            return KeyEventResult.handled;
          }
        }

        return KeyEventResult.ignored;
      }),
    );

    for (var i = 0; i < 10; i++) {
      final controller = TextEditingController();
      controller.addListener(() {
        if (mounted) {
          setState(() {
            _connectButtonDisabled =
                _textControllers.any((element) => element.text.isEmpty);
          });
        }
      });
      _textControllers.add(controller);
    }
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      height: 110,
      width: 498,
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
                  AppLocalizations.of(context)!.connectPageConnectRemoteTitle,
                  style: const TextStyle(fontSize: 27),
                ),
                SizedBox(
                  width: 50,
                  child: Center(
                    child: _isVisitRequesting
                        ? const SizedBox(
                            width: 24,
                            height: 24,
                            child: CircularProgressIndicator(),
                          )
                        : IconButton(
                            onPressed: _connectButtonDisabled
                                ? null
                                : () => _connect(
                                      context.read<SignalingManagerCubit>(),
                                    ),
                            icon: const FaIcon(
                              FontAwesomeIcons.arrowRightToBracket,
                              size: 24,
                            ),
                            disabledColor: Colors.grey,
                            tooltip: AppLocalizations.of(context)!
                                .connectPageConnectRemoteButtonConnectTooltip,
                          ),
                  ),
                ),
              ],
            ),
            Expanded(
              child: Row(
                crossAxisAlignment: CrossAxisAlignment.center,
                children: [
                  FocusScope(
                    node: _focusScopeNode,
                    child: Row(
                      children: [
                        _createField(0, 3, _textControllers[0]),
                        _createField(3, 6, _textControllers[1]),
                        const Text("-", style: TextStyle(fontSize: 36)),
                        _createField(6, 3, _textControllers[2]),
                        _createField(3, 3, _textControllers[3]),
                        _createField(3, 3, _textControllers[4]),
                        _createField(3, 6, _textControllers[5]),
                        const Text("-", style: TextStyle(fontSize: 36)),
                        _createField(6, 3, _textControllers[6]),
                        _createField(3, 3, _textControllers[7]),
                        _createField(3, 3, _textControllers[8]),
                        _createField(3, 0, _textControllers[9]),
                      ],
                    ),
                  ),
                ],
              ),
            )
          ],
        ),
      ),
    );
  }

  Widget _createField(double leftPadding, double rightPadding,
      TextEditingController controller) {
    return Padding(
      padding: EdgeInsets.fromLTRB(leftPadding, 0, rightPadding, 0),
      child: SizedBox(
        width: 38,
        child: DigitInput(
          textEditingController: controller,
          focusNode: _focusScopeNode,
        ),
      ),
    );
  }

  void _connect(SignalingManagerCubit cubit) async {
    try {
      _updateVisitRequestingState(true);

      final remoteDeviceId = _textControllers.map((e) => e.text).join();

      final resp = await cubit.visit(int.parse(remoteDeviceId));

      if (!resp.allow) {
        _notifier.notifyError(
            (context) => "remote device rejects your visit request");
      } else {
        _notifier
            .notifyError((context) => "remote device allow your visit request");
      }
    } catch (err) {
      log("$err");
      if (err.toString().contains("not found")) {
        _notifier.notifyError((context) => "remote device is offline");
        return;
      }

      _notifier.notifyError((context) => "an error occurs when request visit",
          error: err);
    } finally {
      _updateVisitRequestingState(false);
    }

    // try {
    //   final success = await context.read<SignalingManagerCubit>().visit (remoteDeviceId: remoteDeviceId);

    //   if (!success) {
    //     _updateHandshakeState(false);
    //     if (mounted) {
    //       popupDialog(
    //         context,
    //         contentBuilder: (_) =>
    //             Text(AppLocalizations.of(context)!.dialogConnectRemoteOffline),
    //         actionBuilder: (navigatorState) => [
    //           TextButton(
    //             onPressed: navigatorState.pop,
    //             child: Text(AppLocalizations.of(context)!.dialogOK),
    //           )
    //         ],
    //       );
    //     }
    //     return;
    //   }
    // } catch (e) {
    //   _updateHandshakeState(false);
    //   popupDialog(
    //     context,
    //     contentBuilder: (_) => Column(
    //       mainAxisAlignment: MainAxisAlignment.center,
    //       children: [
    //         Text(AppLocalizations.of(context)!.dialogConnectRemoteError),
    //         Text(e.toString()),
    //       ],
    //     ),
    //     actionBuilder: (navigatorState) => [
    //       TextButton(
    //         onPressed: navigatorState.pop,
    //         child: Text(AppLocalizations.of(context)!.dialogOK),
    //       )
    //     ],
    //   );
    //   return;
    // }

    // _updateHandshakeState(false);

    // showGeneralDialog(
    //   context: context,
    //   pageBuilder: (context, animationValue1, animationValue2) {
    //     return StatefulBuilder(builder: (context, setter) {
    //       return ConnectProgressStateDialog(remoteDeviceId: remoteDeviceId);
    //     });
    //   },
    //   barrierDismissible: false,
    //   transitionBuilder: (context, animationValue1, animationValue2, child) {
    //     return Transform.scale(
    //       scale: animationValue1.value,
    //       child: Opacity(
    //         opacity: animationValue1.value,
    //         child: child,
    //       ),
    //     );
    //   },
    //   transitionDuration: kThemeAnimationDuration * 2,
    // );
  }

  void _updateVisitRequestingState(bool isVisitRequesting) {
    setState(() {
      _isVisitRequesting = isVisitRequesting;
    });
  }

  @override
  void dispose() {
    _focusScopeNode.dispose();

    for (var controller in _textControllers) {
      controller.dispose();
    }

    super.dispose();
  }
}

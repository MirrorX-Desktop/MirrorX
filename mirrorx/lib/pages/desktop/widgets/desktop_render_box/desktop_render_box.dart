import 'dart:developer';

import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mirrorx/env/sdk/mirrorx_core.dart';
import 'package:mirrorx/env/utility/key_map.dart';
import 'package:mirrorx/pages/desktop/widgets/desktop_render_box/desktop_render_box_scrollbar.dart';
import 'package:mirrorx/state/desktop_manager/desktop_manager_cubit.dart';

class DesktopRenderBox extends StatefulWidget {
  const DesktopRenderBox(
    this.desktopId, {
    Key? key,
  }) : super(key: key);

  final DesktopId desktopId;

  @override
  _DesktopRenderBoxState createState() => _DesktopRenderBoxState();
}

class _DesktopRenderBoxState extends State<DesktopRenderBox> {
  double _offsetY = 0.0;
  double _offsetX = 0.0;
  final Map<int, int> _downButtons = {};

  @override
  void initState() {
    super.initState();
    HardwareKeyboard.instance.addHandler(_handleKeyboardEvent);
  }

  @override
  void dispose() {
    HardwareKeyboard.instance.removeHandler(_handleKeyboardEvent);
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<DesktopManagerCubit, DesktopManagerState>(
      builder: (context, state) {
        final desktopInfo = state.desktopInfoLists[widget.desktopId];
        if (desktopInfo == null) {
          return const Center(child: Text("DesktopInfo not exists"));
        }

        return desktopInfo.boxFit == BoxFit.none
            ? _buildWithFitBox(desktopInfo)
            : _buildWithoutFitBox(desktopInfo);
      },
    );
  }

  Widget _buildWithFitBox(DesktopInfo desktopInfo) {
    return Stack(
      children: [
        Positioned(
          top: _offsetY,
          left: _offsetX,
          child: _buildWithoutFitBox(desktopInfo),
        ),
        LayoutBuilder(builder: (context, constraints) {
          return DesktopRenderBoxScrollBar(
            maxTrunkWidth: desktopInfo.monitorHeight.floorToDouble(),
            axis: Axis.vertical,
            trunkWidth: constraints.maxHeight,
            onScroll: (offset) {
              setState(() {
                _offsetY = -offset;
                if ((_offsetY + constraints.maxHeight) >
                    desktopInfo.monitorHeight) {
                  _offsetY = desktopInfo.monitorHeight - constraints.maxHeight;
                }
              });
            },
          );
        }),
        LayoutBuilder(builder: (context, constraints) {
          return DesktopRenderBoxScrollBar(
            maxTrunkWidth: desktopInfo.monitorWidth.floorToDouble(),
            axis: Axis.horizontal,
            trunkWidth: constraints.maxWidth,
            onScroll: (offset) {
              setState(() {
                _offsetX = -offset;
                if ((_offsetX + constraints.maxWidth) >
                    desktopInfo.monitorWidth) {
                  _offsetX = desktopInfo.monitorWidth - constraints.maxWidth;
                }
              });
            },
          );
        })
      ],
    );
  }

  Widget _buildWithoutFitBox(DesktopInfo desktopInfo) {
    return FittedBox(
      fit: desktopInfo.boxFit,
      child: Listener(
        behavior: HitTestBehavior.opaque,
        onPointerDown: _handlePointerDown,
        onPointerUp: _handlePointerUp,
        onPointerHover: _handlePointerHover,
        onPointerMove: _handlePointerMove,
        onPointerSignal: _handlePointerSignal,
        child: RepaintBoundary(
          child: SizedBox(
            width: desktopInfo.monitorWidth.floorToDouble(),
            height: desktopInfo.monitorHeight.floorToDouble(),
            child: Center(
              child: AspectRatio(
                aspectRatio: desktopInfo.monitorWidth.toDouble() /
                    desktopInfo.monitorHeight.toDouble(),
                child: Texture(
                  textureId: desktopInfo.textureId,
                  freeze: true,
                  filterQuality: desktopInfo.filterQuality,
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }

  void _handlePointerDown(PointerDownEvent event) {
    log("pointer down ${event.buttons} ${event.pointer}");

    var mouseKey = MouseKey.None;

    switch (event.buttons) {
      case kPrimaryMouseButton:
        mouseKey = MouseKey.Left;
        break;
      case kSecondaryMouseButton:
        mouseKey = MouseKey.Right;
        break;
      case kMiddleMouseButton:
        mouseKey = MouseKey.Wheel;
    }

    context.read<DesktopManagerCubit>().deviceInput(
        widget.desktopId,
        InputEvent.mouse(MouseEvent.down(
          mouseKey,
          event.localPosition.dx,
          event.localPosition.dy,
        )));

    _downButtons[event.pointer] = event.buttons;
  }

  void _handlePointerUp(PointerUpEvent event) {
    // log("pointer up ${event.buttons} ${event.pointer}");

    final button = _downButtons.remove(event.pointer);
    if (button == null) {
      return;
    }

    var mouseKey = MouseKey.None;

    switch (button) {
      case kPrimaryMouseButton:
        mouseKey = MouseKey.Left;
        break;
      case kSecondaryMouseButton:
        mouseKey = MouseKey.Right;
        break;
      case kMiddleMouseButton:
        mouseKey = MouseKey.Wheel;
    }

    context.read<DesktopManagerCubit>().deviceInput(
        widget.desktopId,
        InputEvent.mouse(MouseEvent.up(
          mouseKey,
          event.localPosition.dx,
          event.localPosition.dy,
        )));
  }

  void _handlePointerMove(PointerMoveEvent event) {
    // log("pointer move ${event.buttons} ${event.localPosition}");

    var mouseKey = MouseKey.None;

    switch (event.buttons) {
      case kPrimaryMouseButton:
        mouseKey = MouseKey.Left;
        break;
      case kSecondaryMouseButton:
        mouseKey = MouseKey.Right;
        break;
      case kMiddleMouseButton:
        mouseKey = MouseKey.Wheel;
        break;
      default:
        return;
    }

    context.read<DesktopManagerCubit>().deviceInput(
        widget.desktopId,
        InputEvent.mouse(MouseEvent.move(
          mouseKey,
          event.localPosition.dx,
          event.localPosition.dy,
        )));
  }

  void _handlePointerHover(PointerHoverEvent event) {
    // log("pointer hover ${event.buttons} ${event.localPosition}");

    context.read<DesktopManagerCubit>().deviceInput(
        widget.desktopId,
        InputEvent.mouse(MouseEvent.move(
          MouseKey.None,
          event.localPosition.dx,
          event.localPosition.dy,
        )));
  }

  void _handlePointerSignal(PointerSignalEvent event) {
    if (event is PointerScrollEvent) {
      context.read<DesktopManagerCubit>().deviceInput(widget.desktopId,
          InputEvent.mouse(MouseEvent.scrollWheel(event.scrollDelta.dy)));
    }
  }

  bool _handleKeyboardEvent(KeyEvent event) {
    if (event is KeyRepeatEvent) {
      return true;
    }

    final key = mapLogicalKey(event.logicalKey);
    if (key == null) {
      return true;
    }

    KeyboardEvent keyboardEvent;
    if (event is KeyDownEvent) {
      keyboardEvent = KeyboardEvent.keyDown(key);
    } else if (event is KeyUpEvent) {
      keyboardEvent = KeyboardEvent.keyUp(key);
    } else if (event is KeyRepeatEvent) {
      keyboardEvent = KeyboardEvent.keyDown(key);
    } else {
      log("unhandled keyboard event ${event.runtimeType.toString()}");
      return true;
    }

    log("press $keyboardEvent");

    context
        .read<DesktopManagerCubit>()
        .deviceInput(widget.desktopId, InputEvent.keyboard(keyboardEvent));

    return true;
  }
}

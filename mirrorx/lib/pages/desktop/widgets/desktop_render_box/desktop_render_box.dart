import 'dart:developer';
import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:mirrorx/env/sdk/mirrorx_core.dart';
import 'package:mirrorx/env/sdk/mirrorx_core_sdk.dart';
import 'package:mirrorx/env/utility/key_map.dart';
import 'package:mirrorx/model/desktop.dart';
import 'package:mirrorx/pages/desktop/widgets/desktop_render_box/desktop_render_box_scrollbar.dart';

class DesktopRenderBox extends StatefulWidget {
  const DesktopRenderBox({
    Key? key,
    required this.model,
    required this.width,
    required this.height,
    required this.fit,
  }) : super(key: key);

  final DesktopModel model;
  final int width;
  final int height;
  final BoxFit fit;

  @override
  _DesktopRenderBoxState createState() => _DesktopRenderBoxState();
}

class _DesktopRenderBoxState extends State<DesktopRenderBox> {
  double _offsetY = 0.0;
  double _offsetX = 0.0;
  final Map<int, int> _downButtons = {};
  final FocusNode _focusNode = FocusNode();

  // @override
  // void initState() {
  //   super.initState();
  //   focusAttachment = focusNode.attach(context, onKey: _handleKeyboardEvent);
  //   focusNode.requestFocus();
  // }
  //

  @override
  void dispose() {
    _focusNode.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return widget.fit == BoxFit.none ? _buildFitBox() : _buildTexture();
  }

  Widget _buildFitBox() {
    return Stack(
      children: [
        Positioned(
          top: _offsetY,
          left: _offsetX,
          child: _buildTexture(),
        ),
        LayoutBuilder(builder: (context, constraints) {
          return DesktopRenderBoxScrollBar(
            maxTrunkWidth: widget.height.floorToDouble(),
            axis: Axis.vertical,
            trunkWidth: constraints.maxHeight,
            onScroll: (offset) {
              setState(() {
                _offsetY = -offset;
                if ((_offsetY + constraints.maxHeight) > widget.height) {
                  _offsetY = widget.height - constraints.maxHeight;
                }
              });
            },
          );
        }),
        LayoutBuilder(builder: (context, constraints) {
          return DesktopRenderBoxScrollBar(
            maxTrunkWidth: widget.width.floorToDouble(),
            axis: Axis.horizontal,
            trunkWidth: constraints.maxWidth,
            onScroll: (offset) {
              setState(() {
                _offsetX = -offset;
                if ((_offsetX + constraints.maxWidth) > widget.width) {
                  _offsetX = widget.width - constraints.maxWidth;
                }
              });
            },
          );
        })
      ],
    );
  }

  Widget _buildTexture() {
    return FittedBox(
      fit: widget.fit,
      child: Listener(
        behavior: HitTestBehavior.opaque,
        onPointerDown: _handlePointerDown,
        onPointerUp: _handlePointerUp,
        onPointerHover: _handlePointerHover,
        onPointerMove: _handlePointerMove,
        onPointerSignal: _handlePointerSignal,
        child: Focus(
          focusNode: _focusNode,
          autofocus: true,
          skipTraversal: true,
          onKey: _handleKeyboardEvent,
          child: RepaintBoundary(
            child: SizedBox(
              width: widget.width.floorToDouble(),
              height: widget.height.floorToDouble(),
              child: Center(
                child: AspectRatio(
                  aspectRatio:
                      widget.width.toDouble() / widget.height.toDouble(),
                  child: Texture(
                    textureId: widget.model.textureID,
                    freeze: true,
                    filterQuality: FilterQuality.high,
                  ),
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

    MirrorXCoreSDK.instance.endpointInput(
      remoteDeviceId: widget.model.remoteDeviceID,
      event: InputEvent.mouse(
        MouseEvent.mouseDown(
          mouseKey,
          event.localPosition.dx,
          event.localPosition.dy,
        ),
      ),
    );

    _downButtons[event.pointer] = event.buttons;
  }

  void _handlePointerUp(PointerUpEvent event) {
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

    MirrorXCoreSDK.instance.endpointInput(
      remoteDeviceId: widget.model.remoteDeviceID,
      event: InputEvent.mouse(
        MouseEvent.mouseUp(
          mouseKey,
          event.localPosition.dx,
          event.localPosition.dy,
        ),
      ),
    );
  }

  void _handlePointerMove(PointerMoveEvent event) {
    log("pointer move ${event.buttons} ${event.localPosition}");

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

    MirrorXCoreSDK.instance.endpointInput(
      remoteDeviceId: widget.model.remoteDeviceID,
      event: InputEvent.mouse(
        MouseEvent.mouseMove(
          mouseKey,
          event.localPosition.dx,
          event.localPosition.dy,
        ),
      ),
    );
  }

  void _handlePointerHover(PointerHoverEvent event) {
    log("pointer hover ${event.buttons} ${event.localPosition}");

    MirrorXCoreSDK.instance.endpointInput(
      remoteDeviceId: widget.model.remoteDeviceID,
      event: InputEvent.mouse(
        MouseEvent.mouseMove(
          MouseKey.None,
          event.localPosition.dx,
          event.localPosition.dy,
        ),
      ),
    );
  }

  void _handlePointerSignal(PointerSignalEvent event) {
    if (event is PointerScrollEvent) {
      MirrorXCoreSDK.instance.endpointInput(
        remoteDeviceId: widget.model.remoteDeviceID,
        event: InputEvent.mouse(
          MouseEvent.mouseScrollWheel(event.scrollDelta.dy),
        ),
      );
    }
  }

  KeyEventResult _handleKeyboardEvent(FocusNode _, RawKeyEvent event) {
    if (event.repeat) {
      return KeyEventResult.handled;
    }

    final key = mapLogicalKey(event.logicalKey);
    if (key == null) {
      log("alt:${event.isAltPressed} control:${event.isControlPressed} meta:${event.isMetaPressed} shift:${event.isShiftPressed} repeat:${event.repeat} logical:${event.logicalKey} character:${event.character}");
      return KeyEventResult.ignored;
    }

    var keyboardEvent = KeyboardEvent.keyUp(key);
    if (event.isKeyPressed(event.logicalKey)) {
      keyboardEvent = KeyboardEvent.keyDown(key);
    }

    log("press ${keyboardEvent}");

    MirrorXCoreSDK.instance.endpointInput(
      remoteDeviceId: widget.model.remoteDeviceID,
      event: InputEvent.keyboard(keyboardEvent),
    );

    return KeyEventResult.handled;
  }
}

import 'dart:developer';
import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:mirrorx/env/sdk/mirrorx_core.dart';
import 'package:mirrorx/env/sdk/mirrorx_core_sdk.dart';
import 'package:mirrorx/model/desktop.dart';
import 'package:mirrorx/pages/desktop/widgets/desktop_render_box/desktop_render_box_scrollbar.dart';

class DesktopRenderBox extends StatefulWidget {
  const DesktopRenderBox({
    Key? key,
    required this.model,
    required this.width,
    required this.height,
  }) : super(key: key);

  final DesktopModel model;
  final int width;
  final int height;

  @override
  _DesktopRenderBoxState createState() => _DesktopRenderBoxState();
}

class _DesktopRenderBoxState extends State<DesktopRenderBox> {
  double offsetY = 0.0;
  double offsetX = 0.0;
  Map<int, int> downButtons = {};

  @override
  Widget build(BuildContext context) {
    return Stack(
      children: [
        Positioned(
          top: offsetY,
          left: offsetX,
          width: widget.width.floorToDouble(),
          height: widget.height.floorToDouble(),
          child: _buildTexture(),
        ),
        // Positioned(
        //   right: 0,
        //   width: 6,
        //   child: LayoutBuilder(builder: (context, constraints) {
        //     return DesktopRenderBoxScrollBar(
        //       maxTrunkWidth: widget.height.floorToDouble(),
        //       axis: Axis.vertical,
        //       trunkWidth: constraints.maxHeight,
        //       onScroll: (offset) {
        //         setState(() {
        //           offsetY = -offset;
        //           if ((offsetY + constraints.maxHeight) > widget.height) {
        //             offsetY = widget.height - constraints.maxHeight;
        //           }
        //         });
        //       },
        //     );
        //   }),
        // ),
        // Positioned(
        //   bottom: 0,
        //   height: 6,
        //   child: LayoutBuilder(builder: (context, constraints) {
        //     return DesktopRenderBoxScrollBar(
        //       maxTrunkWidth: widget.width.floorToDouble(),
        //       axis: Axis.horizontal,
        //       trunkWidth: constraints.maxWidth,
        //       onScroll: (offset) {
        //         setState(() {
        //           offsetX = -offset;
        //           if ((offsetX + constraints.maxWidth) > widget.width) {
        //             offsetX = widget.width - constraints.maxWidth;
        //           }
        //         });
        //       },
        //     );
        //   }),
        // )
      ],
    );
  }

  Widget _buildTexture() {
    return
        // RepaintBoundary(
        // child: Listener(
        //   behavior: HitTestBehavior.opaque,
        //   onPointerDown: _handlePointerDown,
        //   onPointerUp: _handlePointerUp,
        //   onPointerHover: _handlePointerHover,
        //   onPointerMove: _handlePointerMove,
        //   onPointerSignal: _handlePointerSignal,
        // child:
        Container(
            color: Colors.black,
            child: Center(
              child: AspectRatio(
                aspectRatio: widget.width.toDouble() / widget.height.toDouble(),
                child: Texture(
                  textureId: widget.model.textureID,
                  freeze: true,
                  filterQuality: FilterQuality.high,
                ),
              ),
            )
            // ),
            // ),
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

    MirrorXCoreSDK.instance.endpointMouseEvent(
      remoteDeviceId: widget.model.remoteDeviceID,
      event: MouseEvent.down(mouseKey),
      x: event.localPosition.dx,
      y: event.localPosition.dy,
    );

    downButtons[event.pointer] = event.buttons;
  }

  void _handlePointerUp(PointerUpEvent event) {
    final button = downButtons.remove(event.pointer);
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

    MirrorXCoreSDK.instance.endpointMouseEvent(
      remoteDeviceId: widget.model.remoteDeviceID,
      event: MouseEvent.up(mouseKey),
      x: event.localPosition.dx,
      y: event.localPosition.dy,
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

    MirrorXCoreSDK.instance.endpointMouseEvent(
      remoteDeviceId: widget.model.remoteDeviceID,
      event: MouseEvent.move(mouseKey),
      x: event.localPosition.dx,
      y: event.localPosition.dy,
    );
  }

  void _handlePointerHover(PointerHoverEvent event) {
    log("pointer hover ${event.buttons} ${event.localPosition}");

    MirrorXCoreSDK.instance.endpointMouseEvent(
      remoteDeviceId: widget.model.remoteDeviceID,
      event: const MouseEvent.move(MouseKey.None),
      x: event.localPosition.dx,
      y: event.localPosition.dy,
    );
  }

  void _handlePointerSignal(PointerSignalEvent event) {
    if (event is PointerScrollEvent) {
      MirrorXCoreSDK.instance.endpointMouseEvent(
        remoteDeviceId: widget.model.remoteDeviceID,
        event: MouseEvent.scrollWheel(event.scrollDelta.dy),
        x: event.localPosition.dx,
        y: event.localPosition.dy,
      );
    }
  }
}

import 'dart:developer';

import 'package:adaptive_scrollbar/adaptive_scrollbar.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:mirrorx/model/desktop.dart';
import 'package:mirrorx/pages/desktop/widgets/desktop_render_box/horizontal_scroll_bar.dart';
import 'package:mirrorx/pages/desktop/widgets/desktop_render_box/vertical_scroll_bar.dart';

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
  double heightOffset = 0.0;
  double widthOffset = 0.0;

  @override
  void initState() {
    super.initState();
    log("initial width: ${widget.width} height: ${widget.height}");
  }

  @override
  Widget build(BuildContext context) {
    return Stack(
      children: [
        Positioned(
          top: heightOffset.floorToDouble(),
          left: widthOffset.floorToDouble(),
          width: widget.width.toDouble(),
          height: widget.height.toDouble(),
          child: _buildTexture(),
        ),
        VerticalScrollBar(
          maxScrollableValue: widget.height.toDouble(),
          onScroll: (offset) {
            setState(() {
              heightOffset = -offset;
              heightOffset = heightOffset.floorToDouble();
            });
          },
        ),
        HorizontalScrollBar(
          maxScrollableValue: widget.width.toDouble(),
          onScroll: (offset) {
            setState(() {
              widthOffset = -offset;
              widthOffset = widthOffset.floorToDouble();
            });
          },
        ),
      ],
    );
  }

  Widget _buildTexture() {
    return Listener(
      behavior: HitTestBehavior.opaque,
      onPointerDown: _handlePointerDown,
      onPointerUp: _handlePointerUp,
      onPointerHover: _handlePointerHover,
      onPointerSignal: _handlePointerSignal,
      child: RepaintBoundary(
        child: Container(
          color: Colors.black,
          child: Center(
            child: AspectRatio(
              aspectRatio: widget.width.toDouble() / widget.height.toDouble(),
              child: Texture(
                textureId: widget.model.textureID,
                freeze: true,
                filterQuality: FilterQuality.medium,
              ),
            ),
          ),
        ),
      ),
    );
  }

  void _handlePointerDown(PointerDownEvent event) {
    log("pointer down ${event.buttons}");
  }

  void _handlePointerUp(PointerUpEvent event) {
    log("pointer up ${event.buttons}");
  }

  void _handlePointerHover(PointerHoverEvent event) {
    final renderObject = context.findRenderObject() as RenderBox?;
    if (renderObject != null) {
      final position = renderObject.globalToLocal(event.position);
      final x = position.dx + (-widthOffset);
      final y = position.dy + (-heightOffset);

      log("pointer hover $x $y");
    }
  }

  void _handlePointerSignal(PointerSignalEvent event) {
    if (event is PointerScrollEvent) {
      log("pointer scroll ${event.scrollDelta}");
    }
  }
}

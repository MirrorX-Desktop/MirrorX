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
  Widget build(BuildContext context) {
    return Stack(
      children: [
        Positioned(
          top: heightOffset,
          left: widthOffset,
          child: Container(
            width: widget.width.toDouble(),
            height: widget.height.toDouble(),
            decoration: const BoxDecoration(
              gradient: LinearGradient(
                colors: [Colors.blue, Color(0xFFf7418c)],
                begin: Alignment.topLeft,
                end: Alignment.bottomRight,
              ),
            ),
            child: _buildTexture(),
          ),
        ),
        LayoutBuilder(builder: ((context, constraints) {
          return VerticalScrollBar(
            maxScrollableValue: widget.width.toDouble(),
            windowHeight: constraints.maxHeight,
            onScroll: (offset) {
              setState(() {
                heightOffset = -offset;
              });
            },
          );
        })),
        LayoutBuilder(builder: ((context, constraints) {
          return HorizontalScrollBar(
            maxScrollableValue: widget.height.toDouble(),
            windowWidth: constraints.maxWidth,
            onScroll: (offset) {
              setState(() {
                widthOffset = -offset;
              });
            },
          );
        })),
      ],
    );
  }

  Widget _buildTexture() {
    return Listener(
      onPointerDown: _handlePointerDown,
      onPointerUp: _handlePointerUp,
      onPointerMove: _handlePointerMove,
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

  void _handlePointerMove(PointerMoveEvent event) {
    log("pointer move ${event.position}");
  }

  void _handlePointerSignal(PointerSignalEvent event) {
    if (event is PointerScrollEvent) {
      log("pointer scroll ${event.scrollDelta}");
    }
  }
}

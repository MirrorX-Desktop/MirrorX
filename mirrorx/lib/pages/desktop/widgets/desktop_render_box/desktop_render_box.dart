import 'package:adaptive_scrollbar/adaptive_scrollbar.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:mirrorx/model/desktop.dart';
import 'package:mirrorx/pages/desktop/widgets/desktop_render_box/horizontal_scroll_bar.dart';
import 'package:mirrorx/pages/desktop/widgets/desktop_render_box/vertical_scroll_bar.dart';

class DesktopRenderBox extends StatefulWidget {
  const DesktopRenderBox({Key? key, required this.model}) : super(key: key);

  final DesktopModel model;

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
            width: 1920,
            height: 1080,
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
            maxScrollableValue: 1080,
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
            maxScrollableValue: 1920,
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
    return RepaintBoundary(
      child: Container(
        color: Colors.black,
        child: Center(
          child: AspectRatio(
            aspectRatio: 16.0 / 9.0,
            child: Texture(
              textureId: widget.model.textureID,
              filterQuality: FilterQuality.none,
            ),
          ),
        ),
      ),
    );
  }
}

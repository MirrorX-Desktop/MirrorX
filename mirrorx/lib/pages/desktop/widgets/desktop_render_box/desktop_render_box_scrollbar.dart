import 'package:flutter/material.dart';

class DesktopRenderBoxScrollBar extends StatefulWidget {
  const DesktopRenderBoxScrollBar({
    Key? key,
    required this.maxTrunkWidth,
    required this.axis,
    required this.trunkWidth,
    required this.onScroll,
  }) : super(key: key);

  final double maxTrunkWidth;
  final double trunkWidth;
  final Axis axis;
  final Function(double offset) onScroll;

  @override
  _DesktopRenderBoxScrollBarState createState() =>
      _DesktopRenderBoxScrollBarState();
}

class _DesktopRenderBoxScrollBarState extends State<DesktopRenderBoxScrollBar> {
  double _thumbOffset = 0;
  double _thumbWidth = 0;
  double _thumbMaxOffset = 0;
  bool _visible = true;

  @override
  void initState() {
    super.initState();

    _updateThumbWidth();
    _updateTrunkOffset();
    _updateThumbOffset();
  }

  @override
  void didUpdateWidget(covariant DesktopRenderBoxScrollBar oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.trunkWidth != widget.trunkWidth) {
      _updateThumbWidth();
      _updateTrunkOffset();
      _updateThumbOffset();
      WidgetsBinding.instance.scheduleFrameCallback((_) {
        _notifyScrollChange();
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Visibility(
      visible: _visible,
      child: GestureDetector(
        onHorizontalDragUpdate:
            widget.axis == Axis.horizontal ? _onDragUpdate : null,
        onVerticalDragUpdate:
            widget.axis == Axis.vertical ? _onDragUpdate : null,
        child: Container(
          alignment: widget.axis == Axis.horizontal
              ? Alignment.bottomLeft
              : Alignment.topRight,
          margin: EdgeInsets.only(
            left: widget.axis == Axis.horizontal ? _thumbOffset : 0,
            top: widget.axis == Axis.vertical ? _thumbOffset : 0,
          ),
          child: _buildScrollThumb(),
        ),
      ),
    );
  }

  Widget _buildScrollThumb() {
    return Container(
      decoration: BoxDecoration(
        color: Colors.black.withOpacity(0.2),
        borderRadius: BorderRadius.circular(6),
      ),
      height: widget.axis == Axis.horizontal ? 6 : _thumbWidth,
      width: widget.axis == Axis.horizontal ? _thumbWidth : 6,
    );
  }

  void _onDragUpdate(DragUpdateDetails details) {
    _thumbOffset +=
        widget.axis == Axis.horizontal ? details.delta.dx : details.delta.dy;

    _updateThumbOffset();
    _notifyScrollChange();
  }

  void _updateThumbWidth() {
    final thumbFactor = widget.trunkWidth / widget.maxTrunkWidth;
    if (thumbFactor >= 1) {
      _visible = false;
    } else {
      _visible = true;
      _thumbWidth = widget.trunkWidth * thumbFactor;
    }
  }

  void _updateTrunkOffset() {
    _thumbMaxOffset = widget.trunkWidth - _thumbWidth;
    if (_thumbMaxOffset < 0) {
      _thumbMaxOffset = 0;
    }
  }

  void _updateThumbOffset() {
    if (_thumbOffset < 0) {
      _thumbOffset = 0;
    }

    if (_thumbOffset > _thumbMaxOffset) {
      _thumbOffset = _thumbMaxOffset;
    }
  }

  void _notifyScrollChange() {
    widget.onScroll(_thumbOffset / widget.trunkWidth * widget.maxTrunkWidth);
  }
}

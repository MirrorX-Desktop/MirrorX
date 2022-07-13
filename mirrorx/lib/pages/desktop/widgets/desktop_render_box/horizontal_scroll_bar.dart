import 'package:flutter/material.dart';
import 'package:flutter/scheduler.dart';
import 'package:window_manager/window_manager.dart';

class DesktopRenderBoxScrollBar extends StatefulWidget {
  const DesktopRenderBoxScrollBar({
    Key? key,
    required this.maxScrollableValue,
    required this.axis,
    required this.initialWidth,
    required this.onScroll,
  }) : super(key: key);

  final double maxScrollableValue;
  final double initialWidth;
  final Axis axis;
  final Function(double offset) onScroll;

  @override
  _DesktopRenderBoxScrollBarState createState() =>
      _DesktopRenderBoxScrollBarState();
}

class _DesktopRenderBoxScrollBarState extends State<DesktopRenderBoxScrollBar>
    with WindowListener {
  double _barOffset = 0;
  double _thumbWidth = 0;
  double _scrollbarMaxOffsetHeight = 0;
  double _boxWidth = 0;
  bool _visible = true;

  @override
  void initState() {
    windowManager.addListener(this);
    super.initState();

    _boxWidth = widget.initialWidth;
    _updateThumb();
    _updateMaxScrollBarOffset();
  }

  @override
  void onWindowResize() {
    setState(() {
      _updateBoxWidth();
      _updateThumb();
      _updateMaxScrollBarOffset();
      _updateBarOffset();
      _notifyScrollChange();
    });
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
            left: widget.axis == Axis.horizontal ? _barOffset : 0,
            top: widget.axis == Axis.vertical ? _barOffset : 0,
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
    _barOffset +=
        widget.axis == Axis.horizontal ? details.delta.dx : details.delta.dy;

    _updateBarOffset();
    _notifyScrollChange();
  }

  void _updateBoxWidth() {
    final size = context.size;
    if (size != null) {
      if (widget.axis == Axis.horizontal) {
        _boxWidth = size.width;
      } else {
        _boxWidth = size.height;
      }
    }
  }

  void _updateThumb() {
    final thumbFactor = _boxWidth / widget.maxScrollableValue;
    if (thumbFactor >= 1) {
      _visible = false;
    } else {
      _visible = true;
      _thumbWidth = _boxWidth * thumbFactor;
    }
  }

  void _updateMaxScrollBarOffset() {
    _scrollbarMaxOffsetHeight = _boxWidth - _thumbWidth;
    if (_scrollbarMaxOffsetHeight < 0) {
      _scrollbarMaxOffsetHeight = 0;
    }
  }

  void _updateBarOffset() {
    if (_barOffset < 0) {
      _barOffset = 0;
    }

    if (_barOffset > _scrollbarMaxOffsetHeight) {
      _barOffset = _scrollbarMaxOffsetHeight;
    }
  }

  void _notifyScrollChange() {
    widget.onScroll(_barOffset / _boxWidth * widget.maxScrollableValue);
  }

  @override
  void dispose() {
    windowManager.removeListener(this);
    super.dispose();
  }
}

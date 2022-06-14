import 'package:flutter/material.dart';
import 'package:flutter/scheduler.dart';

class HorizontalScrollBar extends StatefulWidget {
  const HorizontalScrollBar({
    Key? key,
    required this.maxScrollableValue,
    required this.windowWidth,
    required this.onScroll,
  }) : super(key: key);

  final double maxScrollableValue;
  final double windowWidth;
  final Function(double offset) onScroll;

  @override
  _HorizontalScrollBarState createState() => _HorizontalScrollBarState();
}

class _HorizontalScrollBarState extends State<HorizontalScrollBar>
    with WidgetsBindingObserver {
  double _barOffset = 0;
  late double _thumbWidth;
  late double _scrollbarMaxOffsetHeight;

  @override
  void initState() {
    super.initState();
    _updateThumb();
    _updateMaxScrollBarOffset();
    WidgetsBinding.instance.addObserver(this);
  }

  @override
  void didChangeMetrics() {
    setState(() {
      _updateThumb();
      _updateMaxScrollBarOffset();
      _updateBarOffset();
      _notifyScrollChange();
    });
  }

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onHorizontalDragUpdate: _onHorizontalDragUpdate,
      child: Container(
        alignment: Alignment.bottomLeft,
        margin: EdgeInsets.only(left: _barOffset),
        child: _buildScrollThumb(),
      ),
    );
  }

  Widget _buildScrollThumb() {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 16),
      child: Container(
        decoration: BoxDecoration(
          color: Colors.blue,
          borderRadius: BorderRadius.circular(6),
        ),
        height: 10,
        width: _thumbWidth,
      ),
    );
  }

  void _onHorizontalDragUpdate(DragUpdateDetails details) {
    _barOffset += details.delta.dx;
    _updateBarOffset();
    _notifyScrollChange();
  }

  void _updateThumb() {
    _thumbWidth =
        widget.windowWidth * widget.windowWidth / widget.maxScrollableValue;
    if (_thumbWidth > widget.maxScrollableValue) {
      _thumbWidth = widget.maxScrollableValue;
    }
  }

  void _updateMaxScrollBarOffset() {
    var height = widget.windowWidth;
    if (height > widget.maxScrollableValue) {
      height = widget.maxScrollableValue;
    }

    _scrollbarMaxOffsetHeight = height - _thumbWidth;
  }

  void _updateBarOffset() {
    if (_barOffset + _thumbWidth > widget.maxScrollableValue) {
      _barOffset = widget.maxScrollableValue - _thumbWidth;
    }

    if (_barOffset < 0) {
      _barOffset = 0;
    }

    if (_barOffset > _scrollbarMaxOffsetHeight) {
      _barOffset = _scrollbarMaxOffsetHeight;
    }
  }

  void _notifyScrollChange() {
    widget
        .onScroll(_barOffset / widget.windowWidth * widget.maxScrollableValue);
  }

  @override
  void dispose() {
    WidgetsBinding.instance.removeObserver(this);
    super.dispose();
  }
}

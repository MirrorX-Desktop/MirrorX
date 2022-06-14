import 'package:flutter/material.dart';

class VerticalScrollBar extends StatefulWidget {
  const VerticalScrollBar({
    Key? key,
    required this.maxScrollableValue,
    required this.windowHeight,
    required this.onScroll,
  }) : super(key: key);

  final double maxScrollableValue;
  final double windowHeight;
  final Function(double offset) onScroll;

  @override
  _VerticalScrollBarState createState() => _VerticalScrollBarState();
}

class _VerticalScrollBarState extends State<VerticalScrollBar>
    with WidgetsBindingObserver {
  double _barOffset = 0;
  late double _thumbHeight;
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
      onVerticalDragUpdate: _onVerticalDragUpdate,
      child: Container(
        alignment: Alignment.topRight,
        margin: EdgeInsets.only(top: _barOffset),
        child: _buildScrollThumb(),
      ),
    );
  }

  Widget _buildScrollThumb() {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 16),
      child: Container(
        decoration: BoxDecoration(
          color: Colors.blue,
          borderRadius: BorderRadius.circular(6),
        ),
        height: _thumbHeight,
        width: 10,
      ),
    );
  }

  void _onVerticalDragUpdate(DragUpdateDetails details) {
    _barOffset += details.delta.dy;
    _updateBarOffset();
    _notifyScrollChange();
  }

  void _updateThumb() {
    _thumbHeight =
        widget.windowHeight * widget.windowHeight / widget.maxScrollableValue;
    if (_thumbHeight > widget.maxScrollableValue) {
      _thumbHeight = widget.maxScrollableValue;
    }
  }

  void _updateMaxScrollBarOffset() {
    var height = widget.windowHeight;
    if (height > widget.maxScrollableValue) {
      height = widget.maxScrollableValue;
    }

    _scrollbarMaxOffsetHeight = height - _thumbHeight;
  }

  void _updateBarOffset() {
    if (_barOffset + _thumbHeight > widget.maxScrollableValue) {
      _barOffset = widget.maxScrollableValue - _thumbHeight;
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
        .onScroll(_barOffset / widget.windowHeight * widget.maxScrollableValue);
  }

  @override
  void dispose() {
    WidgetsBinding.instance.removeObserver(this);
    super.dispose();
  }
}

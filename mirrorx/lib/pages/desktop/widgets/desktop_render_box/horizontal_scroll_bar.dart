import 'package:flutter/material.dart';
import 'package:flutter/scheduler.dart';

class HorizontalScrollBar extends StatefulWidget {
  const HorizontalScrollBar({
    Key? key,
    required this.maxScrollableValue,
    required this.onScroll,
  }) : super(key: key);

  final double maxScrollableValue;
  final Function(double offset) onScroll;

  @override
  _HorizontalScrollBarState createState() => _HorizontalScrollBarState();
}

class _HorizontalScrollBarState extends State<HorizontalScrollBar>
    with WidgetsBindingObserver {
  double _barOffset = 0;
  double _thumbWidth = 0;
  double _scrollbarMaxOffsetHeight = 0;
  double _boxWidth = 0;

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addObserver(this);
    WidgetsBinding.instance.scheduleFrameCallback((_) {
      final size = context.size;
      if (size != null) {
        _boxWidth = size.width;
      }

      _updateThumb();
      _updateMaxScrollBarOffset();
      _updateBarOffset();
      _notifyScrollChange();
    });
  }

  @override
  void didChangeMetrics() {
    setState(() {
      final size = context.size;
      if (size != null) {
        _boxWidth = size.width;
      }

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
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8),
      decoration: BoxDecoration(
        color: Colors.black.withOpacity(0.2),
        borderRadius: BorderRadius.circular(6),
      ),
      height: 6,
      width: _thumbWidth,
    );
  }

  void _onHorizontalDragUpdate(DragUpdateDetails details) {
    _barOffset += details.delta.dx;
    _updateBarOffset();
    _notifyScrollChange();
  }

  void _updateThumb() {
    _thumbWidth = _boxWidth * _boxWidth / widget.maxScrollableValue;
    if (_thumbWidth > widget.maxScrollableValue) {
      _thumbWidth = widget.maxScrollableValue;
    }
  }

  void _updateMaxScrollBarOffset() {
    var height = _boxWidth;
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
    widget.onScroll(_barOffset / _boxWidth * widget.maxScrollableValue);
  }

  @override
  void dispose() {
    WidgetsBinding.instance.removeObserver(this);
    super.dispose();
  }
}

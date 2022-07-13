import 'package:flutter/material.dart';
import 'package:window_manager/window_manager.dart';

class VerticalScrollBar extends StatefulWidget {
  const VerticalScrollBar({
    Key? key,
    required this.maxScrollableValue,
    required this.onScroll,
  }) : super(key: key);

  final double maxScrollableValue;
  final Function(double offset) onScroll;

  @override
  _VerticalScrollBarState createState() => _VerticalScrollBarState();
}

class _VerticalScrollBarState extends State<VerticalScrollBar>
    with WindowListener {
  double _barOffset = 0;
  double _thumbHeight = 0;
  double _scrollbarMaxOffsetHeight = 0;
  double _boxHeight = 0;

  @override
  void initState() {
    windowManager.addListener(this);
    super.initState();
  }

  @override
  void onWindowResize() {
    setState(() {
      final size = context.size;
      if (size != null) {
        _boxHeight = size.height;
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
      onVerticalDragUpdate: _onVerticalDragUpdate,
      child: Container(
        alignment: Alignment.topRight,
        margin: EdgeInsets.only(top: _barOffset),
        child: _buildScrollThumb(),
      ),
    );
  }

  Widget _buildScrollThumb() {
    return Container(
      padding: const EdgeInsets.symmetric(vertical: 8),
      decoration: BoxDecoration(
        color: Colors.black.withOpacity(0.2),
        borderRadius: BorderRadius.circular(6),
      ),
      height: _thumbHeight,
      width: 6,
    );
  }

  void _onVerticalDragUpdate(DragUpdateDetails details) {
    _barOffset += details.delta.dy;
    _updateBarOffset();
    _notifyScrollChange();
  }

  void _updateThumb() {
    final thumbFactor = _boxHeight / widget.maxScrollableValue;
    if (thumbFactor >= 1) {
      // todo disable
    } else {
      _thumbHeight = _boxHeight * thumbFactor;
    }
  }

  void _updateMaxScrollBarOffset() {
    _scrollbarMaxOffsetHeight = _boxHeight - _thumbHeight;
    if (_scrollbarMaxOffsetHeight < 0) {
      _scrollbarMaxOffsetHeight = 0;
    }
  }

  void _updateBarOffset() {
    // if (_barOffset + _thumbHeight > widget.maxScrollableValue) {
    // _barOffset = widget.maxScrollableValue - _thumbHeight;
    // }

    if (_barOffset < 0) {
      _barOffset = 0;
    }

    if (_barOffset > _scrollbarMaxOffsetHeight) {
      _barOffset = _scrollbarMaxOffsetHeight;
    }
  }

  void _notifyScrollChange() {
    widget.onScroll(_barOffset / _boxHeight * widget.maxScrollableValue);
  }

  @override
  void dispose() {
    windowManager.removeListener(this);
    super.dispose();
  }
}

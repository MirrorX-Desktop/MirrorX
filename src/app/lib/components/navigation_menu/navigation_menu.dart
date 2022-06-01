import 'dart:developer';
import 'dart:io';

import 'package:app/business/page_manager/page_manager_bloc.dart';
import 'package:app/components/navigation_menu/navigation_button.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

class Pair<T1, T2> {
  T1 first;
  T2 second;

  Pair(this.first, this.second);
}

class NavigationMenu extends StatelessWidget {
  const NavigationMenu({
    Key? key,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<PageManagerBloc, PageManagerState>(
      builder: (context, state) => Column(
        children: [
          Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              for (int i = 0; i < state.fixedPages.length; i += 1)
                Padding(
                  padding: const EdgeInsets.symmetric(vertical: 2.0),
                  child: NavigationMenuItem(
                      itemIndex: i,
                      icon: state.fixedPages[i].titleIcon,
                      label: state.fixedPages[i].title,
                      onTap: () {
                        BlocProvider.of<PageManagerBloc>(context)
                            .add(PageManagerSwitchPage(pageIndex: i));
                      }),
                ),
            ],
          ),
          Visibility(
              visible: state.dynamicPages.isNotEmpty,
              child: Container(
                  width: 36,
                  margin: const EdgeInsets.symmetric(vertical: 6),
                  decoration: BoxDecoration(
                    border: Border.all(color: Colors.black, width: 0.5),
                    borderRadius: BorderRadius.circular(4),
                  ))),
          Expanded(
            child: SizedBox(
              width: 72,
              child: ListView(
                primary: true,
                physics: const BouncingScrollPhysics(),
                children: <Widget>[
                  for (int i = 0; i < state.dynamicPages.length; i += 1)
                    Padding(
                      padding: const EdgeInsets.symmetric(vertical: 2.0),
                      child: NavigationMenuItem(
                        itemIndex: state.dynamicPages.length + i,
                        icon: state.dynamicPages[i].titleIcon,
                        label: state.dynamicPages[i].title,
                        onTap: () {
                          BlocProvider.of<PageManagerBloc>(context).add(
                              PageManagerSwitchPage(
                                  pageIndex: state.fixedPages.length + i));
                        },
                      ),
                    ),
                ],
              ),
            ),
          )
        ],
      ),
    );
  }
}

class NavigationMenuItem extends StatefulWidget {
  const NavigationMenuItem({
    Key? key,
    required this.itemIndex,
    required this.icon,
    required this.label,
    required this.onTap,
  }) : super(key: key);

  final int itemIndex;
  final IconData icon;
  final String label;
  final VoidCallback onTap;

  @override
  _NavigationMenuItemState createState() => _NavigationMenuItemState();
}

class _NavigationMenuItemState extends State<NavigationMenuItem>
    with TickerProviderStateMixin {
  late _ButtonStatusFSM _buttonStatusFSM;
  late AnimationController _textAnimationController;
  late Animation<double> _textAnimation;
  late AnimationController _indicatorAnimationController;
  late Animation<double> _indicatorAnimation;
  bool _isHover = false;

  @override
  void initState() {
    super.initState();

    final bloc = BlocProvider.of<PageManagerBloc>(context);

    _buttonStatusFSM = _ButtonStatusFSM(bloc.isSelected(widget.itemIndex));

    _textAnimationController =
        AnimationController(duration: kThemeAnimationDuration * 2, vsync: this);

    _textAnimation = CurvedAnimation(
      parent: _textAnimationController,
      curve: Curves.easeInOut,
    );

    _indicatorAnimationController = AnimationController(
        duration: kThemeAnimationDuration * 1, vsync: this, value: 1.0);

    _indicatorAnimation = CurvedAnimation(
      parent: _indicatorAnimationController,
      curve: Curves.easeInOut,
    );
  }

  @override
  Widget build(BuildContext context) {
    final bloc = BlocProvider.of<PageManagerBloc>(context);

    return BlocListener<PageManagerBloc, PageManagerState>(
      listener: ((context, state) {
        final before = _buttonStatusFSM._status;

        state.currentPage.getIndex() == widget.itemIndex
            ? _buttonStatusFSM.goActive()
            : _buttonStatusFSM.goNormal();

        final after = _buttonStatusFSM._status;

        if (before != after) {
          _textAnimationController.reset();
          _textAnimationController.forward();
          _indicatorAnimationController.reset();
          _indicatorAnimationController.forward();
        }
      }),
      child: _addMouseRegion(
        bloc,
        AnimatedBuilder(
            animation: _textAnimation,
            builder: (context, child) {
              final color = _textAnimation.isDismissed
                  ? _buttonStatusFSM.currentColors.second
                  : ColorTween(
                      begin: _buttonStatusFSM.currentColors.first,
                      end: _buttonStatusFSM.currentColors.second,
                    ).transform(
                      CurveTween(curve: Curves.easeInOutCubicEmphasized)
                          .transform(_textAnimation.value));

              return SizedBox(
                  width: 56,
                  height: 56,
                  child:
                      Stack(alignment: AlignmentDirectional.center, children: [
                    child!,
                    Column(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        Icon(widget.icon, color: color),
                        Text(widget.label,
                            style: TextStyle(
                                fontSize: 12, height: 1.33, color: color))
                      ],
                    )
                  ]));
            },
            child: AnimatedBuilder(
              animation: _indicatorAnimation,
              builder: (context, child) {
                final length = 56.0 *
                    (!_isHover && bloc.isSelected(widget.itemIndex)
                        ? _indicatorAnimation.value
                        : 1 - _indicatorAnimation.value);

                return DecoratedBox(
                    decoration: BoxDecoration(
                      color: Colors.yellow,
                      borderRadius: BorderRadius.circular(16),
                    ),
                    child: SizedBox(
                      width: length,
                      height: length,
                    ));
              },
            )),
      ),
    );
  }

  Widget _addMouseRegion(PageManagerBloc bloc, Widget child) {
    return MouseRegion(
      onEnter: (_) {
        if (!bloc.isSelected(widget.itemIndex)) {
          _isHover = true;
          _buttonStatusFSM.goHover();
          _textAnimationController.reset();
          _textAnimationController.forward();
        }
      },
      onExit: (_) {
        if (!bloc.isSelected(widget.itemIndex)) {
          _isHover = true;
          _buttonStatusFSM.goNormal();
          _textAnimationController.reset();
          _textAnimationController.forward();
        }
      },
      child: GestureDetector(
        behavior: HitTestBehavior.opaque,
        onTap: () {
          if (!bloc.isSelected(widget.itemIndex)) {
            _isHover = false;
            widget.onTap();
          }
        },
        child: child,
      ),
    );
  }

  @override
  void dispose() {
    _textAnimationController.dispose();
    _indicatorAnimationController.dispose();
    super.dispose();
  }
}

enum _ButtonStatus {
  normal,
  hover,
  actived,
}

class _ButtonStatusFSM {
  _ButtonStatus _status;
  Pair<Color, Color> _colors;

  Pair<Color, Color> get currentColors => _colors;

  _ButtonStatusFSM(bool initialSelected)
      : _status =
            initialSelected ? _ButtonStatus.actived : _ButtonStatus.normal,
        _colors = Pair(initialSelected ? Colors.white : Colors.black,
            initialSelected ? Colors.white : Colors.black);

  void goHover() {
    if (_status == _ButtonStatus.normal) {
      _status = _ButtonStatus.hover;
      _colors = Pair(Colors.black, Colors.yellow);
    }
  }

  void goNormal() {
    if (_status == _ButtonStatus.hover) {
      _colors = Pair(Colors.yellow, Colors.black);
      _status = _ButtonStatus.normal;
    }

    if (_status == _ButtonStatus.actived) {
      _colors = Pair(Colors.white, Colors.black);
      _status = _ButtonStatus.normal;
    }
  }

  void goActive() {
    if (_status == _ButtonStatus.hover) {
      _status = _ButtonStatus.actived;
      _colors = Pair(Colors.yellow, Colors.white);
    }
  }
}

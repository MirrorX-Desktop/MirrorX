import 'package:marquee/marquee.dart';
import 'package:mirrorx/env/langs/tr.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mirrorx/state/desktop_manager/desktop_manager_cubit.dart';
import 'package:mirrorx/state/page_manager/page_manager_cubit.dart';

class NavigationMenu extends StatelessWidget {
  const NavigationMenu({
    Key? key,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<DesktopManagerCubit, DesktopManagerState>(
      builder: (context, state) => Column(
        children: [
          Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              _NavigationMenuItem(
                pageTag: "Connect",
                icon: Icons.screen_share,
                title: tr.connectPageTitle,
                isStatic: true,
              ),
              _NavigationMenuItem(
                pageTag: "Intranet",
                icon: Icons.lan,
                title: tr.intranetPageTitle,
                isStatic: true,
              ),
              _NavigationMenuItem(
                pageTag: "Files",
                icon: Icons.drive_file_move_rtl,
                title: tr.filesPageTitle,
                isStatic: true,
              ),
              _NavigationMenuItem(
                pageTag: "History",
                icon: Icons.history,
                title: tr.historyPageTitle,
                isStatic: true,
              ),
              _NavigationMenuItem(
                pageTag: "Settings",
                icon: Icons.settings,
                title: tr.settingsPageTitle,
                isStatic: true,
              ),
            ],
          ),
          Visibility(
              visible: state.desktopModels.isNotEmpty,
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
                children: state.desktopModels
                    .map(
                      (model) => Padding(
                        padding: const EdgeInsets.symmetric(vertical: 2.0),
                        child: _NavigationMenuItem(
                          pageTag: model.remoteDeviceId,
                          icon: Icons.apple,
                          title: model.remoteDeviceId,
                          isStatic: false,
                        ),
                      ),
                    )
                    .toList(),
              ),
            ),
          )
        ],
      ),
    );
  }
}

class _NavigationMenuItem extends StatefulWidget {
  const _NavigationMenuItem({
    Key? key,
    required this.pageTag,
    required this.icon,
    required this.title,
    required this.isStatic,
  }) : super(key: key);

  final String pageTag;
  final IconData icon;
  final String title;
  final bool isStatic;

  @override
  _NavigationMenuItemState createState() => _NavigationMenuItemState();
}

class _NavigationMenuItemState extends State<_NavigationMenuItem>
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

    final cubit = context.read<PageManagerCubit>();

    _buttonStatusFSM = _ButtonStatusFSM(cubit.isCurrent(widget.pageTag));

    _textAnimationController = AnimationController(
      duration: kThemeAnimationDuration * 2,
      vsync: this,
    );

    _textAnimation = CurvedAnimation(
      parent: _textAnimationController,
      curve: Curves.easeInOut,
    );

    _indicatorAnimationController = AnimationController(
      duration: kThemeAnimationDuration * 1,
      vsync: this,
      value: cubit.isCurrent(widget.pageTag) ? 0.0 : 1.0,
    );

    _indicatorAnimation = CurvedAnimation(
      parent: _indicatorAnimationController,
      curve: Curves.easeInOut,
    );

    _textAnimationController.forward();
    _indicatorAnimationController.forward();
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<PageManagerCubit, PageManagerState>(
      buildWhen: (previous, current) {
        if (previous.currentPageTag == current.currentPageTag) {
          return false;
        }

        bool update = false;

        if (previous.currentPageTag == widget.pageTag &&
            current.currentPageTag != widget.pageTag) {
          _buttonStatusFSM.goNormal();
          update = true;
        }

        if (previous.currentPageTag != widget.pageTag &&
            current.currentPageTag == widget.pageTag) {
          _buttonStatusFSM.goActive();
          update = true;
        }

        if (update) {
          _textAnimationController.reset();
          _textAnimationController.forward();
          _indicatorAnimationController.reset();
          _indicatorAnimationController.forward();
        }

        return update;
      },
      builder: (context, state) {
        return Padding(
          padding: const EdgeInsets.symmetric(vertical: 2.0),
          child: _addMouseRegion(
            context.read<PageManagerCubit>(),
            AnimatedBuilder(
              animation: _textAnimation,
              builder: (context, child) {
                final color = _textAnimation.isDismissed
                    ? _buttonStatusFSM.currentColors.newColor
                    : ColorTween(
                        begin: _buttonStatusFSM.currentColors.oldColor,
                        end: _buttonStatusFSM.currentColors.newColor,
                      ).transform(
                        CurveTween(curve: Curves.easeInOutCubicEmphasized)
                            .transform(_textAnimation.value));

                return SizedBox(
                    width: 56,
                    height: 56,
                    child: Stack(
                        alignment: AlignmentDirectional.center,
                        children: [
                          child!,
                          Column(
                            mainAxisAlignment: MainAxisAlignment.center,
                            children: [
                              Icon(widget.icon, color: color),
                              widget.isStatic
                                  ? Text(widget.title,
                                      style: TextStyle(
                                        fontSize: 12,
                                        height: 1.33,
                                        color: color,
                                      ))
                                  : SizedBox(
                                      width: 36,
                                      height: 16,
                                      child: Marquee(
                                        text:
                                            "${widget.title.substring(0, 2)}-${widget.title.substring(2, 6)}-${widget.title.substring(6, 10)}",
                                        fadingEdgeStartFraction: 0.2,
                                        fadingEdgeEndFraction: 0.2,
                                        velocity: 10,
                                        blankSpace: 10,
                                        style: TextStyle(
                                          fontSize: 12,
                                          height: 1.33,
                                          color: color,
                                        ),
                                      ),
                                    ),
                            ],
                          )
                        ]));
              },
              child: AnimatedBuilder(
                animation: _indicatorAnimation,
                builder: (context, child) {
                  final length = 56.0 *
                      (!_isHover &&
                              context
                                  .read<PageManagerCubit>()
                                  .isCurrent(widget.pageTag)
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
              ),
            ),
          ),
        );
      },
    );
  }

  Widget _addMouseRegion(PageManagerCubit cubit, Widget child) {
    return MouseRegion(
      onEnter: (_) {
        if (!cubit.isCurrent(widget.pageTag)) {
          _isHover = true;
          _buttonStatusFSM.goHover();
          _textAnimationController.reset();
          _textAnimationController.forward();
        }
      },
      onExit: (_) {
        if (!cubit.isCurrent(widget.pageTag)) {
          _isHover = true;
          _buttonStatusFSM.goNormal();
          _textAnimationController.reset();
          _textAnimationController.forward();
        }
      },
      child: GestureDetector(
        behavior: HitTestBehavior.opaque,
        onTap: () {
          if (!cubit.isCurrent(widget.pageTag)) {
            _isHover = false;
            context.read<PageManagerCubit>().switchPage(widget.pageTag);
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

class _TransitionColorPair {
  Color oldColor;
  Color newColor;

  _TransitionColorPair(this.oldColor, this.newColor);
}

enum _ButtonStatus {
  normal,
  hover,
  actived,
}

class _ButtonStatusFSM {
  _ButtonStatus _status;
  _TransitionColorPair _colors;

  _TransitionColorPair get currentColors => _colors;

  _ButtonStatusFSM(bool actived)
      : _status = actived ? _ButtonStatus.actived : _ButtonStatus.normal,
        _colors = actived
            ? _TransitionColorPair(Colors.black, Colors.white)
            : _TransitionColorPair(Colors.black, Colors.black);

  void goHover() {
    if (_status == _ButtonStatus.normal) {
      _status = _ButtonStatus.hover;
      _colors = _TransitionColorPair(Colors.black, Colors.yellow);
    }
  }

  void goNormal() {
    if (_status == _ButtonStatus.hover) {
      _colors = _TransitionColorPair(Colors.yellow, Colors.black);
      _status = _ButtonStatus.normal;
    }

    if (_status == _ButtonStatus.actived) {
      _colors = _TransitionColorPair(Colors.white, Colors.black);
      _status = _ButtonStatus.normal;
    }
  }

  void goActive() {
    if (_status == _ButtonStatus.hover) {
      _status = _ButtonStatus.actived;
      _colors = _TransitionColorPair(Colors.yellow, Colors.white);
    }

    if (_status == _ButtonStatus.normal) {
      _status = _ButtonStatus.actived;
      _colors = _TransitionColorPair(Colors.black, Colors.white);
    }
  }
}

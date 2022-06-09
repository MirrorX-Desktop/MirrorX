import 'package:mirrorx/env/langs/tr.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mirrorx/pages/main/cubit/main_page_manager_cubit.dart';

class NavigationMenu extends StatelessWidget {
  const NavigationMenu({
    Key? key,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<MainPageManagerCubit, MainPageManagerState>(
      builder: (context, state) => Column(
        children: [
          Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              _NavigationMenuItem(
                  pageTag: "Connect",
                  icon: Icons.screen_share,
                  title: Tr.of(context).connectPageTitle,
                  onTap: () => context
                      .read<MainPageManagerCubit>()
                      .switchPage("Connect")),
              _NavigationMenuItem(
                  pageTag: "Intranet",
                  icon: Icons.lan,
                  title: Tr.of(context).intranetPageTitle,
                  onTap: () => context
                      .read<MainPageManagerCubit>()
                      .switchPage("Intranet")),
              _NavigationMenuItem(
                  pageTag: "Files",
                  icon: Icons.drive_file_move_rtl,
                  title: Tr.of(context).filesPageTitle,
                  onTap: () =>
                      context.read<MainPageManagerCubit>().switchPage("Files")),
              _NavigationMenuItem(
                  pageTag: "History",
                  icon: Icons.history,
                  title: Tr.of(context).historyPageTitle,
                  onTap: () => context
                      .read<MainPageManagerCubit>()
                      .switchPage("History")),
              _NavigationMenuItem(
                  pageTag: "Settings",
                  icon: Icons.settings,
                  title: Tr.of(context).settingsPageTitle,
                  onTap: () => context
                      .read<MainPageManagerCubit>()
                      .switchPage("Settings")),
            ],
          ),
          Visibility(
              visible: state.registerTextures.isNotEmpty,
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
                children: state.registerTextures.entries
                    .map((entry) => Padding(
                          padding: const EdgeInsets.symmetric(vertical: 2.0),
                          child: _NavigationMenuItem(
                              pageTag: entry.key,
                              icon: Icons.apple,
                              title: entry.key,
                              onTap: () => context
                                  .read<MainPageManagerCubit>()
                                  .switchPage(entry.key)),
                        ))
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
    required this.onTap,
  }) : super(key: key);

  final String pageTag;
  final IconData icon;
  final String title;
  final VoidCallback onTap;

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

    _buttonStatusFSM = _ButtonStatusFSM();

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
    return BlocListener<MainPageManagerCubit, MainPageManagerState>(
      listener: ((context, state) {
        final before = _buttonStatusFSM._status;

        state.currentPageTag == widget.pageTag
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
      child: Padding(
        padding: const EdgeInsets.symmetric(vertical: 2.0),
        child: _addMouseRegion(
          context.read<MainPageManagerCubit>(),
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
                              Text(widget.title,
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
                      (!_isHover &&
                              context
                                  .read<MainPageManagerCubit>()
                                  .isPageSelected(widget.pageTag)
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
      ),
    );
  }

  Widget _addMouseRegion(MainPageManagerCubit bloc, Widget child) {
    return MouseRegion(
      onEnter: (_) {
        if (!bloc.isPageSelected(widget.pageTag)) {
          _isHover = true;
          _buttonStatusFSM.goHover();
          _textAnimationController.reset();
          _textAnimationController.forward();
        }
      },
      onExit: (_) {
        if (!bloc.isPageSelected(widget.pageTag)) {
          _isHover = true;
          _buttonStatusFSM.goNormal();
          _textAnimationController.reset();
          _textAnimationController.forward();
        }
      },
      child: GestureDetector(
        behavior: HitTestBehavior.opaque,
        onTap: () {
          if (!bloc.isPageSelected(widget.pageTag)) {
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

  _ButtonStatusFSM()
      : _status = _ButtonStatus.normal,
        _colors = _TransitionColorPair(Colors.black, Colors.black);

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

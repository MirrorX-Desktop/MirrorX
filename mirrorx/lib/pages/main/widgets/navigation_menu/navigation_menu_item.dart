import 'dart:developer';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:font_awesome_flutter/font_awesome_flutter.dart';
import 'package:marquee/marquee.dart';
import 'package:mirrorx/env/sdk/mirrorx_core_sdk.dart';
import 'package:mirrorx/env/utility/dialog.dart';
import 'package:mirrorx/model/desktop.dart';
import 'package:mirrorx/state/desktop_manager/desktop_manager_cubit.dart';
import 'package:mirrorx/state/page_manager/page_manager_cubit.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';

class NavigationMenuItem extends StatefulWidget {
  const NavigationMenuItem({
    Key? key,
    required this.pageTag,
    required this.iconBuilder,
    required this.title,
    required this.system,
    this.desktopClosed,
    this.desktopModel,
  })  : assert(system == false
            ? desktopModel != null
                ? true
                : false
            : true),
        super(key: key);

  final String pageTag;
  final Widget Function(Color?) iconBuilder;
  final String title;
  final bool system;
  final bool? desktopClosed;
  final DesktopModel? desktopModel;

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
                        clipBehavior: Clip.antiAliasWithSaveLayer,
                        children: [
                          child!,
                          Column(
                            mainAxisAlignment: MainAxisAlignment.center,
                            crossAxisAlignment: CrossAxisAlignment.center,
                            children: [
                              SizedBox(
                                width: 32,
                                height: 32,
                                child: Stack(
                                  alignment: AlignmentDirectional.center,
                                  clipBehavior: Clip.none,
                                  children: [
                                    Center(
                                      child: widget.iconBuilder(color),
                                    ),
                                    Align(
                                      alignment: Alignment.bottomRight,
                                      child: Visibility(
                                        visible: !widget.system,
                                        child: _buildOfflineDot(),
                                      ),
                                    ),
                                  ],
                                ),
                              ),
                              Visibility(
                                visible: widget.system,
                                child: Text(
                                  widget.title,
                                  style: TextStyle(
                                    fontSize: 12,
                                    height: 1.33,
                                    color: color,
                                  ),
                                ),
                              ),
                              Visibility(
                                visible: !widget.system,
                                child: SizedBox(
                                  width: 36,
                                  height: 16,
                                  child: Marquee(
                                    text: "${widget.title}",
                                    // "${widget.title.substring(0, 2)}-${widget.title.substring(2, 6)}-${widget.title.substring(6, 10)}",
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
        onSecondaryTap: widget.system
            ? null
            : () async {
                const offset = Offset(72, 0);
                final RenderBox button =
                    context.findRenderObject()! as RenderBox;
                final RenderBox overlay = Navigator.of(context)
                    .overlay!
                    .context
                    .findRenderObject()! as RenderBox;
                final RelativeRect position = RelativeRect.fromRect(
                  Rect.fromPoints(
                    button.localToGlobal(offset, ancestor: overlay),
                    button.localToGlobal(
                      button.size.bottomRight(Offset.zero) + offset,
                      ancestor: overlay,
                    ),
                  ),
                  Offset.zero & overlay.size,
                );

                final menuItem = await showMenu<int>(
                  context: context,
                  elevation: 3,
                  color: Colors.white,
                  items: [
                    PopupMenuItem(
                      value: 1,
                      enabled: false,
                      child: Text(AppLocalizations.of(context)!
                          .navigationPopupMenuItemTitleRemark),
                    ),
                    const PopupMenuDivider(),
                    PopupMenuItem(
                      value: 2,
                      child: Text(
                        AppLocalizations.of(context)!
                            .navigationPopupMenuItemTitleDisconnect,
                        style: const TextStyle(color: Colors.red),
                      ),
                    ),
                  ],
                  position: position,
                  constraints: const BoxConstraints(minWidth: 200),
                  shape: RoundedRectangleBorder(
                    side: const BorderSide(),
                    borderRadius: BorderRadius.circular(8),
                  ),
                );

                switch (menuItem) {
                  case 1:
                    break;
                  case 2:
                    askDisconnect();
                    break;
                  default:
                }
              },
        child: child,
      ),
    );
  }

  Widget _buildOfflineDot() {
    return AnimatedBuilder(
      animation: _indicatorAnimation,
      builder: (context, _) {
        final color = _indicatorAnimation.isDismissed
            ? context.read<PageManagerCubit>().isCurrent(widget.pageTag)
                ? Colors.yellow
                : Colors.white
            : ColorTween(
                begin:
                    context.read<PageManagerCubit>().isCurrent(widget.pageTag)
                        ? Colors.white
                        : Colors.yellow,
                end: context.read<PageManagerCubit>().isCurrent(widget.pageTag)
                    ? Colors.yellow
                    : Colors.white,
              ).transform(CurveTween(curve: Curves.easeInOutCubicEmphasized)
                .transform(_indicatorAnimation.value));

        return Container(
          width: 14,
          height: 14,
          decoration: BoxDecoration(
            color: color,
            shape: BoxShape.circle,
          ),
          child: Center(
            child: Container(
              width: 8.5,
              height: 8.5,
              decoration: BoxDecoration(
                color: widget.desktopClosed == true ? Colors.red : Colors.green,
                shape: BoxShape.circle,
              ),
            ),
          ),
        );
      },
    );
  }

  void askDisconnect() async {
    final remoteDeviceId = widget.desktopModel!.remoteDeviceId;

    if (!mounted) {
      return;
    }

    final desktopManagerCubit = context.read<DesktopManagerCubit>();
    final pageManagerCubit = context.read<PageManagerCubit>();

    popupDialog(
      context,
      contentBuilder: (context) => Text(
        AppLocalizations.of(context)!.dialogContentManuallyClose,
        textAlign: TextAlign.center,
      ),
      actionBuilder: (navigatorState) => [
        TextButton(
          onPressed: () {
            log("press yes");

            desktopManagerCubit.removeDesktop(remoteDeviceId);
            pageManagerCubit.switchPage("Connect");

            MirrorXCoreSDK.instance
                .endpointManuallyClose(remoteDeviceId: remoteDeviceId);

            navigatorState.pop();
          },
          child: Text(AppLocalizations.of(context)!.dialogYes),
        ),
        TextButton(
          onPressed: navigatorState.pop,
          child: Text(AppLocalizations.of(context)!.dialogNo),
        ),
      ],
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

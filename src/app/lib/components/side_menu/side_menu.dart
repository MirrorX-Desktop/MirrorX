import 'package:easy_localization/easy_localization.dart';
import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:font_awesome_flutter/font_awesome_flutter.dart';
import 'package:mirrorx/components/navigator/navigator.dart';
import 'package:mirrorx/constants.dart';
import 'package:mirrorx/pages/remote_desktop/remote_desktop.dart';
import 'package:provider/provider.dart';

class SideMenu extends StatefulWidget {
  const SideMenu({Key? key}) : super(key: key);

  @override
  _SideMenuState createState() => _SideMenuState();
}

class _SideMenuState extends State<SideMenu> {
  late AppNavigator _appNavigator;

  var _list = <RemoteDesktopPage>[];

  @override
  void initState() {
    _appNavigator = Provider.of(context, listen: false);
    _appNavigator.addListener(() {
      setState(() {
        _list = _appNavigator.getRemoteDesktopPages();
      });
    });
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return DecoratedBox(
      decoration: BoxDecoration(
          border:
              Border(right: BorderSide(color: Theme.of(context).dividerColor))),
      child: Padding(
        padding: const EdgeInsets.only(top: 32.0),
        child: SizedBox(
          width: 160,
          height: double.infinity,
          child: Padding(
            padding: const EdgeInsets.all(8.0),
            child: Column(
              children: [
                Column(
                  children: [
                    SideMenuSystemButton(
                        icon: Icons.screen_share,
                        title: tr("side_menu.connect_to_remote"),
                        pageTag: "home"),
                    SideMenuSystemButton(
                        icon: Icons.lan,
                        title: tr("side_menu.lan_discovery"),
                        pageTag: "lan"),
                    SideMenuSystemButton(
                        icon: Icons.drive_file_move_rtl,
                        title: tr("side_menu.file_transfer"),
                        pageTag: "file"),
                    SideMenuSystemButton(
                        icon: Icons.history,
                        title: tr("side_menu.connection_history"),
                        pageTag: "history"),
                    SideMenuSystemButton(
                        icon: Icons.settings,
                        title: tr("side_menu.settings"),
                        pageTag: "settings"),
                    const Divider(height: 8),
                  ],
                ),
                Expanded(
                  child: Scrollbar(
                    child: ListView.builder(
                      // physics: const ClampingScrollPhysics(),
                      // children: _buildDesktopButtons(),
                      addAutomaticKeepAlives: false,
                      itemCount: _list.length,
                      itemBuilder: (context, index) {
                        return _buildDesktopButtons(index);
                      },
                    ),
                  ),
                )
              ],
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildDesktopButtons(int index) {
    return SideMenuDesktopButton(
        icon: FontAwesomeIcons.apple,
        title: _list.elementAt(index).tag,
        pageTag: _list.elementAt(index).tag);
  }
}

class SideMenuSystemButton extends StatefulWidget {
  const SideMenuSystemButton(
      {Key? key,
      required this.icon,
      required this.title,
      required this.pageTag})
      : super(key: key);

  final IconData icon;
  final String title;
  final String pageTag;

  @override
  _SideMenuSystemButtonState createState() => _SideMenuSystemButtonState();
}

class _SideMenuSystemButtonState extends State<SideMenuSystemButton>
    with TickerProviderStateMixin {
  late AppNavigator _appNavigator;

  late AnimationController _titleColorController;
  late Animation<Color?> _titleColorAnimation;

  late AnimationController _backgroundColorController;
  late Animation<Color?> _backgroundColorAnimation;

  bool _isCurrentNaviagtorPage = false;
  Color? _currentTextColor;
  Color? _currentBackgroundColor;

  @override
  void initState() {
    _appNavigator = Provider.of(context, listen: false);
    _appNavigator.addListener(() {
      _currentNavigationPageNotified();
    });

    _titleColorController = AnimationController(
        duration: const Duration(milliseconds: 160), vsync: this);

    _titleColorAnimation = ColorTween().animate(_titleColorController);

    _backgroundColorController = AnimationController(
        duration: const Duration(milliseconds: 160), vsync: this);
    _backgroundColorAnimation =
        ColorTween().animate(_backgroundColorController);

    _currentNavigationPageNotified();
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return Padding(
        padding: const EdgeInsets.symmetric(vertical: 4),
        child: AnimatedBuilder(
            animation: _titleColorAnimation,
            builder: (context, child) {
              return DecoratedBox(
                  decoration: BoxDecoration(
                    color: _backgroundColorAnimation.value,
                    borderRadius: const BorderRadius.all(Radius.circular(10)),
                  ),
                  child: _buildInnerButton());
            }));
  }

  Widget _buildInnerButton() {
    return MouseRegion(
        cursor: SystemMouseCursors.click,
        onEnter: _onMouseHoverEnter,
        onExit: _onMouseHoverExit,
        child: GestureDetector(
          onTap: _onMouseClick,
          behavior: HitTestBehavior.opaque,
          child: Padding(
            padding: const EdgeInsets.all(8.0),
            child: Row(
              children: [
                Padding(
                  padding: const EdgeInsets.only(right: 8.0),
                  child: Icon(widget.icon,
                      size: 24, color: _titleColorAnimation.value),
                ),
                Text(
                  widget.title,
                  style: TextStyle(
                      color: _titleColorAnimation.value,
                      fontSize: 14,
                      fontWeight: FontWeight.w500),
                ),
              ],
            ),
          ),
        ));
  }

  void _updateTextColorAnimation(Color forwardColor) {
    setState(() {
      _titleColorController.reset();
      _titleColorAnimation =
          ColorTween(begin: _currentTextColor, end: forwardColor)
              .animate(_titleColorController);
      _currentTextColor = forwardColor;
      _titleColorController.forward();
    });
  }

  void _updateBackgroundColorAnimation(Color forwardColor) {
    setState(() {
      _titleColorController.reset();
      _backgroundColorAnimation =
          ColorTween(begin: _currentBackgroundColor, end: forwardColor)
              .animate(_titleColorController);
      _currentBackgroundColor = forwardColor;
      _titleColorController.forward();
    });
  }

  void _onMouseHoverEnter(PointerEnterEvent _) {
    if (!_isCurrentNaviagtorPage) {
      _updateTextColorAnimation(primaryColor);
      _updateBackgroundColorAnimation(Colors.white);
    }
  }

  void _onMouseHoverExit(PointerExitEvent _) {
    if (!_isCurrentNaviagtorPage) {
      _updateTextColorAnimation(Colors.black);
      _updateBackgroundColorAnimation(Colors.white);
    }
  }

  void _onMouseClick() {
    if (_appNavigator.currentPageTag == widget.pageTag) {
      return;
    }
    _appNavigator.jumpToPage(widget.pageTag);
  }

  void _currentNavigationPageNotified() {
    if (mounted) {
      setState(() {
        _updateNavigationChanged();
      });
    }
  }

  void _updateNavigationChanged() {
    final myNavigationPageIsNewPage =
        _appNavigator.currentPageTag == widget.pageTag;

    if (_isCurrentNaviagtorPage && !myNavigationPageIsNewPage) {
      // true -> false means this button is inactive
      _updateTextColorAnimation(Colors.black);
      _updateBackgroundColorAnimation(Colors.white);
    } else if (!_isCurrentNaviagtorPage && myNavigationPageIsNewPage) {
      // false -> true means this button is active
      _updateTextColorAnimation(Colors.white);
      _updateBackgroundColorAnimation(primaryColor);
    }

    _isCurrentNaviagtorPage = myNavigationPageIsNewPage;
  }

  @override
  void dispose() {
    _appNavigator.removeListener(_currentNavigationPageNotified);
    _titleColorController.dispose();
    _titleColorController.dispose();
    super.dispose();
  }
}

class SideMenuDesktopButton extends StatefulWidget {
  const SideMenuDesktopButton(
      {Key? key,
      required this.icon,
      required this.title,
      required this.pageTag})
      : super(key: key);

  final IconData icon;
  final String title;
  final String pageTag;

  @override
  _SideMenuDesktopButtonState createState() => _SideMenuDesktopButtonState();
}

class _SideMenuDesktopButtonState extends State<SideMenuDesktopButton>
    with TickerProviderStateMixin {
  late AppNavigator _appNavigator;

  late AnimationController _titleColorController;
  late Animation<Color?> _titleColorAnimation;

  late AnimationController _backgroundColorController;
  late Animation<Color?> _backgroundColorAnimation;

  bool _isCurrentNaviagtorPage = false;
  Color? _currentTextColor;
  Color? _currentBackgroundColor;

  @override
  void initState() {
    _appNavigator = Provider.of(context, listen: false);
    _appNavigator.addListener(() {
      _selectedPageNotified();
    });

    _titleColorController = AnimationController(
        duration: const Duration(milliseconds: 160), vsync: this);

    _titleColorAnimation = ColorTween().animate(_titleColorController);

    _backgroundColorController = AnimationController(
        duration: const Duration(milliseconds: 160), vsync: this);
    _backgroundColorAnimation =
        ColorTween().animate(_backgroundColorController);

    _selectedPageNotified();

    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return Padding(
        padding: const EdgeInsets.symmetric(vertical: 4),
        child: AnimatedBuilder(
            animation: _titleColorAnimation,
            builder: (context, child) {
              return ClipRRect(
                borderRadius: const BorderRadius.all(Radius.circular(10)),
                child: DecoratedBox(
                  decoration: BoxDecoration(
                    color: _backgroundColorAnimation.value,
                    border: Border.all(
                        width: 1,
                        color: (_isCurrentNaviagtorPage
                                ? _backgroundColorAnimation.value
                                : _titleColorAnimation.value) ??
                            Colors.transparent),
                    borderRadius: const BorderRadius.all(Radius.circular(10)),
                  ),
                  child: MouseRegion(
                    cursor: SystemMouseCursors.click,
                    onEnter: _onMouseHoverEnter,
                    onExit: _onMouseHoverExit,
                    child: GestureDetector(
                      onTap: _onMouseClick,
                      behavior: HitTestBehavior.opaque,
                      child: _buildInnerButton(),
                    ),
                  ),
                ),
              );
            }));
  }

  Widget _buildInnerButton() {
    return Stack(
      children: [
        Table(
          defaultVerticalAlignment: TableCellVerticalAlignment.middle,
          children: [
            TableRow(
              children: [
                Padding(
                  padding: const EdgeInsets.all(8.0),
                  child: Text(
                    "ID: ${widget.pageTag}",
                    style: TextStyle(
                        color: _titleColorAnimation.value,
                        fontSize: 14,
                        fontWeight: FontWeight.w500),
                  ),
                ),
              ],
            ),
            TableRow(
              children: [
                Padding(
                  padding: const EdgeInsets.fromLTRB(8, 0, 8, 8),
                  child: Text(
                    "hostName",
                    style: TextStyle(
                        color: _titleColorAnimation.value,
                        fontSize: 14,
                        fontWeight: FontWeight.w500),
                  ),
                ),
              ],
            ),
          ],
        ),
        Positioned(
          right: -6,
          bottom: -12,
          child: FaIcon(
            FontAwesomeIcons.windows,
            color: _titleColorAnimation.value,
            size: 45,
          ),
        ),
      ],
    );
  }

  void _updateTextColorAnimation(Color forwardColor) {
    setState(() {
      _titleColorController.reset();
      _titleColorAnimation =
          ColorTween(begin: _currentTextColor, end: forwardColor)
              .animate(_titleColorController);
      _currentTextColor = forwardColor;
      _titleColorController.forward();
    });
  }

  void _updateBackgroundColorAnimation(Color forwardColor) {
    setState(() {
      _backgroundColorController.reset();
      _backgroundColorAnimation =
          ColorTween(begin: _currentBackgroundColor, end: forwardColor)
              .animate(_backgroundColorController);
      _currentBackgroundColor = forwardColor;
      _backgroundColorController.forward();
    });
  }

  void _onMouseHoverEnter(PointerEnterEvent _) {
    if (!_isCurrentNaviagtorPage) {
      _updateTextColorAnimation(primaryColor);
      _updateBackgroundColorAnimation(Colors.white);
    }
  }

  void _onMouseHoverExit(PointerExitEvent _) {
    if (!_isCurrentNaviagtorPage) {
      _updateTextColorAnimation(Colors.black);
      _updateBackgroundColorAnimation(Colors.white);
    }
  }

  void _onMouseClick() {
    if (_appNavigator.currentPageTag == widget.pageTag) {
      return;
    }

    _appNavigator.jumpToPage(widget.pageTag);
  }

  void _selectedPageNotified() {
    if (mounted) {
      setState(() {
        _updateSelectedState();
      });
    }
  }

  void _updateSelectedState() {
    final myNavigationPageIsNewPage =
        _appNavigator.currentPageTag == widget.pageTag;

    if (_isCurrentNaviagtorPage && !myNavigationPageIsNewPage) {
      // true -> false means this button is inactive
      _updateTextColorAnimation(Colors.black);
      _updateBackgroundColorAnimation(Colors.white);
    } else if (!_isCurrentNaviagtorPage && myNavigationPageIsNewPage) {
      // false -> true means this button is active
      _updateTextColorAnimation(Colors.white);
      _updateBackgroundColorAnimation(primaryColor);
    }

    _isCurrentNaviagtorPage = myNavigationPageIsNewPage;
  }

  @override
  void dispose() {
    _appNavigator.removeListener(_selectedPageNotified);
    _titleColorController.dispose();
    _backgroundColorController.dispose();
    super.dispose();
  }
}

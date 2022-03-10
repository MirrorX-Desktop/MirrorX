import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:mirrorx/pages/connection_history/connection_history.dart';
import 'package:mirrorx/pages/file_transfer/file_transfer.dart';
import 'package:mirrorx/pages/home/home.dart';
import 'package:mirrorx/pages/lan_discovery/lan_discovery.dart';
import 'package:mirrorx/pages/remote_desktop/remote_desktop.dart';
import 'package:mirrorx/pages/settings/settings.dart';

class AppNavigator extends ChangeNotifier {
  final _pageController = PageController();

  PageController get pageController => _pageController;

  final _systemPages = [
    const HomePage(tag: "home"),
    const LanDiscoveryPage(tag: "lan"),
    const FilePage(tag: "file"),
    const HistoryPage(tag: "history"),
    const SettingsPage(tag: "settings"),
  ];

  final _remoteDesktopPages = <RemoteDesktopPage>[];

  String _currentPageTag = "home";

  String get currentPageTag => _currentPageTag;

  void addAndJumpRemoteDesktopPage(String tag) {
    if (_remoteDesktopPages.any((element) => element.tag == tag)) {
      return;
    }

    _remoteDesktopPages.add(RemoteDesktopPage(tag: tag));
    // notifyListeners();

    jumpToPage(tag);
  }

  void removeRemoteDesktopPage(String tag) {
    if (!_remoteDesktopPages.any((element) => element.tag == tag)) {
      return;
    }

    _remoteDesktopPages.removeWhere((element) => element.tag == tag);
    notifyListeners();
  }

  int totalPageCount() {
    return _systemPages.length + _remoteDesktopPages.length;
  }

  void jumpToPage(String tag) {
    int systemPageIndex =
        _systemPages.indexWhere((element) => element.tag == tag);
    if (systemPageIndex >= 0) {
      _currentPageTag = tag;
      _pageController.jumpToPage(systemPageIndex);
      notifyListeners();
      return;
    }

    int remoteDesktopPageIndex =
        _remoteDesktopPages.indexWhere((element) => element.tag == tag);
    if (remoteDesktopPageIndex >= 0) {
      _currentPageTag = tag;
      _pageController.jumpToPage(_systemPages.length + remoteDesktopPageIndex);
      notifyListeners();
      return;
    }

    throw Exception("not exist page tag: $tag");
  }

  List<RemoteDesktopPage> getRemoteDesktopPages() {
    return _remoteDesktopPages;
  }

  Widget buildPage(int index) {
    if (index < _systemPages.length) {
      return _systemPages.elementAt(index);
    }

    final remoteDesktopPageIndex = index - _systemPages.length;

    if (remoteDesktopPageIndex < _remoteDesktopPages.length) {
      return _remoteDesktopPages.elementAt(remoteDesktopPageIndex);
    }

    throw Exception("build page index greater than exist page count");
  }
}

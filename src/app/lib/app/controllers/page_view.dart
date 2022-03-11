import 'dart:developer' as dev;

import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:mirrorx/app/core/values/page_tag.dart';
import 'package:mirrorx/app/modules/connect_remote/page.dart';
import 'package:mirrorx/app/modules/remote_desktop/page.dart';

import '../modules/connection_history/page.dart';
import '../modules/file_transfer/page.dart';
import '../modules/lan_discovery/page.dart';
import '../modules/settings/page.dart';

class PageViewController extends GetxController {
  late int _selectedIndex;

  late PageController _pageController;

  late int _totalPageViewCount;

  late List<GetView> _staticPages;
  late List<RemoteDesktopPage> _remoteDesktopPages;

  @override
  void onInit() {
    _selectedIndex = 0;
    _pageController = PageController();
    _staticPages = [
      const ConnectRemotePage(
        staticTag: PageTags.connectRemote,
      ),
      const LanDiscoveryPage(
        staticTag: PageTags.lanDiscovery,
      ),
      const FileTransferPage(
        staticTag: PageTags.fileTransfer,
      ),
      const ConnectionHistoryPage(
        staticTag: PageTags.connectionHistory,
      ),
      const SettingsPage(
        staticTag: PageTags.settings,
      ),
    ];
    _totalPageViewCount = _staticPages.length;
    _remoteDesktopPages = <RemoteDesktopPage>[];

    super.onInit();
  }

  PageController get pageController => _pageController;

  int get totalPageViewCount => _totalPageViewCount;

  String get selectedTag => _getSelectedTag();

  void addNewRemoteDesktopPage(String remoteID) {
    final exist = _remoteDesktopPages.any((element) => element.tag == remoteID);
    if (exist) {
      dev.log("addNewRemoteDesktopPage: duplicate remoteID: $remoteID");
      return;
    }

    final newRemoteDesktopPage = RemoteDesktopPage(remoteID: remoteID);
    _remoteDesktopPages.add(newRemoteDesktopPage);
    _totalPageViewCount = _staticPages.length + _remoteDesktopPages.length;
    // default select to the new added page
    _selectedIndex = totalPageViewCount - 1;
    pageController.jumpToPage(_selectedIndex);
    update();
  }

  void remoteRemoteDesktopPage(String remoteID) {
    final wantRemovePageIndex =
        _remoteDesktopPages.indexWhere((element) => element.tag == remoteID);
    if (wantRemovePageIndex < 0) {
      dev.log("remoteRemoteDesktopPage: no exist remoteID: $remoteID");
      return;
    }

    _remoteDesktopPages.removeAt(wantRemovePageIndex);
    _totalPageViewCount = _staticPages.length + _remoteDesktopPages.length;
    if (wantRemovePageIndex == 0) {
      // if there is no more remote desktop page can be removed, jump to connect_remote pagel
      _selectedIndex = 0;
    } else {
      _selectedIndex = totalPageViewCount - 1;
    }
    pageController.jumpToPage(_selectedIndex);
    update();
  }

  void jumpToPage(String tag) {
    var index = _staticPages.indexWhere((element) => element.tag == tag);
    if (index < 0) {
      index = _remoteDesktopPages.indexWhere((element) => element.tag == tag);
      if (index < 0) {
        dev.log("jumpToPage: non exist tag: $tag");
        return;
      }
      index += _staticPages.length;
    }

    _selectedIndex = index;
    pageController.jumpToPage(_selectedIndex);
    update();
  }

  Widget getPage(int index) {
    if (index < _staticPages.length) {
      return _staticPages.elementAt(index);
    }

    final remoteDesktopPageIndex = index - _staticPages.length;

    if (remoteDesktopPageIndex < _remoteDesktopPages.length) {
      return _remoteDesktopPages.elementAt(remoteDesktopPageIndex);
    }

    throw Exception("build page index greater than exist page count");
  }

  String _getSelectedTag() {
    if (_selectedIndex < _staticPages.length) {
      return _staticPages.elementAt(_selectedIndex).tag!;
    }

    final remoteDesktopPageIndex = _selectedIndex - _staticPages.length;

    if (remoteDesktopPageIndex < _remoteDesktopPages.length) {
      return _remoteDesktopPages.elementAt(remoteDesktopPageIndex).tag!;
    }

    throw Exception("build page index greater than exist page count");
  }
}

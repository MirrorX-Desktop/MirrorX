import 'dart:developer';

import 'package:equatable/equatable.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mirrorx/pages/connect/connect_page.dart';
import 'package:mirrorx/pages/desktop/desktop_page.dart';
import 'package:mirrorx/pages/files/files_page.dart';
import 'package:mirrorx/pages/history/history_page.dart';
import 'package:mirrorx/pages/intranet/intranet_page.dart';
import 'package:mirrorx/pages/settings/settings_page.dart';
import 'package:texture_render/model.dart';

part 'main_page_manager_state.dart';

class MainPageManagerCubit extends Cubit<MainPageManagerState> {
  MainPageManagerCubit() : super(const MainPageManagerState());

  bool isPageSelected(String pageTag) => state.currentPageTag == pageTag;

  void switchPage(String pageTag) {
    if (pageTag == state.currentPageTag) {
      return;
    }

    switch (pageTag) {
      case "Connect":
        emit(state.copyWith(
            currentPage: const ConnectPage(), currentPageTag: "Connect"));
        break;
      case "Intranet":
        emit(state.copyWith(
            currentPage: const IntranetPage(), currentPageTag: "Intranet"));
        break;
      case "Files":
        emit(state.copyWith(
            currentPage: const FilesPage(), currentPageTag: "Files"));
        break;
      case "History":
        emit(state.copyWith(
            currentPage: const HistoryPage(), currentPageTag: "History"));
        break;
      case "Settings":
        emit(state.copyWith(
            currentPage: const SettingsPage(), currentPageTag: "Settings"));
        break;
      default:
        for (final entry in state.registerTextures.entries) {
          if (entry.key == pageTag) {
            emit(state.copyWith(
                currentPage: DesktopPage(resp: entry.value),
                currentPageTag: pageTag));
            break;
          }
        }
    }
  }

  void addDesktopPage(String remoteDeviceID, RegisterTextureResponse resp) {
    var newRegisterTextures =
        Map<String, RegisterTextureResponse>.from(state.registerTextures);
    newRegisterTextures[remoteDeviceID] = resp;

    emit(state.copyWith(registerTextures: newRegisterTextures));
  }

  void removeDesktopPage(String desktopPageTag) {
    // final removePageIndex = state.registerTextures.keys.toList()
    //     .indexWhere((element) => element == desktopPageTag);
    // if (removePageIndex == -1) {
    //   return;
    // }

    // String switchPageTag;
    // if (state.registerTextures.length == 1) {
    //   switchPageTag = "Connect";
    // } else {
    //   if (removePageIndex == state.registerTextures.length - 1) {
    //     // remove the last page, switch to the previous page
    //     switchPageTag = state.desktopPages[removePageIndex - 1].remoteDeviceID;
    //   } else {
    //     // remove the first or middle page, switch to the next page
    //     switchPageTag = state.desktopPages[removePageIndex + 1].remoteDeviceID;
    //   }
    // }

    // switchPage(switchPageTag);

    // emit(state.copyWith(
    //     desktopPages: List.from(state.desktopPages)
    //       ..removeWhere(
    //           (element) => element.remoteDeviceID == desktopPageTag)));
  }
}

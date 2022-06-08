import 'package:mirrorx/business/page_manager/page.dart';
import 'package:mirrorx/pages/connect/connect_page.dart';
import 'package:mirrorx/pages/files/files_page.dart';
import 'package:mirrorx/pages/history/history_page.dart';
import 'package:mirrorx/pages/intranet/intranet_page.dart';
import 'package:mirrorx/pages/settings/settings_page.dart';
import 'package:bloc/bloc.dart';
import 'package:equatable/equatable.dart';
import 'package:flutter/material.dart';

part 'page_manager_event.dart';
part 'page_manager_state.dart';

class PageManagerBloc extends Bloc<PageManagerEvent, PageManagerState> {
  PageManagerBloc() : super(const PageManagerState()) {
    on<PageManagerInit>(_init);
    on<PageManagerSwitchPage>(_switchPage);
    on<PageManagerAddPage>(_addPage);
    on<PageManagerRemovePage>(_removePage);
  }

  bool isSelected(String pageTag) => state.currentPageTag == pageTag;

  void _init(PageManagerInit event, Emitter<PageManagerState> emit) {
    add(PageManagerSwitchPage(pageTag: "Connect"));
  }

  void _switchPage(
      PageManagerSwitchPage event, Emitter<PageManagerState> emit) {
    switch (event.pageTag) {
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
        for (var page in state.dynamicPages) {
          if (page.uniqueTag == event.pageTag) {
            emit(state.copyWith(
                currentPage: page, currentPageTag: page.uniqueTag));
            break;
          }
        }
    }
  }

  void _addPage(PageManagerAddPage event, Emitter<PageManagerState> emit) {
    emit(state.copyWith(dynamicPages: state.dynamicPages..add(event.page)));
  }

  void _removePage(
      PageManagerRemovePage event, Emitter<PageManagerState> emit) {
    final removePageIndex = state.dynamicPages
        .indexWhere((element) => element.uniqueTag == event.pageTag);
    if (removePageIndex == -1) {
      return;
    }

    String switchPageTag;
    if (state.dynamicPages.length == 1) {
      switchPageTag = "Connect";
    } else {
      if (removePageIndex == state.dynamicPages.length - 1) {
        // remove the last page, switch to the previous page
        switchPageTag = state.dynamicPages[removePageIndex - 1].uniqueTag;
      } else {
        // remove the first or middle page, switch to the next page
        switchPageTag = state.dynamicPages[removePageIndex + 1].uniqueTag;
      }
    }

    add(PageManagerSwitchPage(pageTag: switchPageTag));

    emit(state.copyWith(
        dynamicPages: state.dynamicPages
          ..removeWhere((element) => element.uniqueTag == event.pageTag)));
  }
}

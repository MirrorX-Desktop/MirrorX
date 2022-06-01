import 'package:app/business/page_manager/page.dart';
import 'package:app/pages/connect/connect_page.dart';
import 'package:app/pages/files/files_page.dart';
import 'package:app/pages/history/history_page.dart';
import 'package:app/pages/lan/lan_page.dart';
import 'package:app/pages/settings/settings_page.dart';
import 'package:bloc/bloc.dart';
import 'package:equatable/equatable.dart';
import 'package:flutter/material.dart';
import 'package:meta/meta.dart';

part 'page_manager_event.dart';
part 'page_manager_state.dart';

class PageManagerBloc extends Bloc<PageManagerEvent, PageManagerState> {
  PageManagerBloc() : super(const PageManagerState()) {
    on<PageManagerInit>(_init);
    on<PageManagerSwitchPage>(_switchPage);
    on<PageManagerAddPage>(_addPage);
    on<PageManagerRemovePage>(_removePage);
  }

  bool isSelected(int index) => state.currentPage.getIndex() == index;

  void _init(PageManagerInit event, Emitter<PageManagerState> emit) {
    final fixedPages = [
      const ConnectPage(),
      const LanPage(),
      const FilesPage(),
      const HistoryPage(),
      const SettingsPage()
    ];

    emit(state.copyWith(fixedPages: fixedPages, currentPage: fixedPages[0]));
  }

  void _switchPage(
      PageManagerSwitchPage event, Emitter<PageManagerState> emit) {
    for (var page in state.fixedPages) {
      if (page.getIndex() == event.pageIndex) {
        emit(state.copyWith(currentPage: page));
        return;
      }
    }

    for (var page in state.dynamicPages) {
      if (page.getIndex() == event.pageIndex) {
        emit(state.copyWith(currentPage: page));
        return;
      }
    }
  }

  void _addPage(PageManagerAddPage event, Emitter<PageManagerState> emit) {
    emit(state.copyWith(dynamicPages: state.dynamicPages..add(event.page)));
  }

  void _removePage(
      PageManagerRemovePage event, Emitter<PageManagerState> emit) {
    if (state.currentPage ==
        state.dynamicPages
            .singleWhere((element) => element.getIndex() == event.pageIndex)) {
      // remove current display page
      var newCurrenPage = state.fixedPages[0];
      if (state.dynamicPages.length > 1) {
        var newCurrentPageIndex =
            state.dynamicPages.indexOf(state.currentPage) - 1;

        if (newCurrentPageIndex < 0) {
          newCurrentPageIndex += 1;
        }
        newCurrenPage = state.dynamicPages[newCurrentPageIndex];
      }

      add(PageManagerSwitchPage(pageIndex: newCurrenPage.getIndex()));
    }

    emit(state.copyWith(
        dynamicPages: state.dynamicPages
          ..removeWhere((element) => element.getIndex() == event.pageIndex)));
  }
}

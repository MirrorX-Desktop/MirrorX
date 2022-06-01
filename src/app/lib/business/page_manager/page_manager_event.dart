part of 'page_manager_bloc.dart';

@immutable
abstract class PageManagerEvent {}

class PageManagerInit extends PageManagerEvent {}

class PageManagerSwitchPage extends PageManagerEvent {
  final int pageIndex;

  PageManagerSwitchPage({
    required this.pageIndex,
  });
}

class PageManagerAddPage extends PageManagerEvent {
  final NavigationPage page;

  PageManagerAddPage({
    required this.page,
  });
}

class PageManagerRemovePage extends PageManagerEvent {
  final int pageIndex;

  PageManagerRemovePage({required this.pageIndex});
}

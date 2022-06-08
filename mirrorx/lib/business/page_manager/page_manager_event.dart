part of 'page_manager_bloc.dart';

@immutable
abstract class PageManagerEvent {}

class PageManagerInit extends PageManagerEvent {}

class PageManagerSwitchPage extends PageManagerEvent {
  final String pageTag;

  PageManagerSwitchPage({
    required this.pageTag,
  });
}

class PageManagerAddPage extends PageManagerEvent {
  final NavigationPage page;

  PageManagerAddPage({
    required this.page,
  });
}

class PageManagerRemovePage extends PageManagerEvent {
  final String pageTag;

  PageManagerRemovePage({required this.pageTag});
}

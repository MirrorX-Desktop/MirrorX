part of 'page_manager_bloc.dart';

class PageManagerState extends Equatable {
  const PageManagerState({
    this.fixedPages = const [],
    this.dynamicPages = const [],
    this.currentPage = const ConnectPage(),
  });

  final List<NavigationPage> fixedPages;
  final List<NavigationPage> dynamicPages;
  final NavigationPage currentPage;

  PageManagerState copyWith({
    List<NavigationPage>? fixedPages,
    List<NavigationPage>? dynamicPages,
    NavigationPage? currentPage,
  }) =>
      PageManagerState(
        fixedPages: fixedPages ?? this.fixedPages,
        dynamicPages: dynamicPages ?? this.dynamicPages,
        currentPage: currentPage ?? this.currentPage,
      );

  @override
  List<Object?> get props => [currentPage, dynamicPages];
}

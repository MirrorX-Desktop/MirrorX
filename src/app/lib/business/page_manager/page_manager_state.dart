part of 'page_manager_bloc.dart';

class PageManagerState extends Equatable {
  const PageManagerState(
      {this.dynamicPages = const [], this.currentPage, this.currentPageTag});

  final List<NavigationPage> dynamicPages;
  final Widget? currentPage;
  final String? currentPageTag;

  PageManagerState copyWith({
    List<NavigationPage>? dynamicPages,
    Widget? currentPage,
    String? currentPageTag,
  }) =>
      PageManagerState(
        dynamicPages: dynamicPages ?? this.dynamicPages,
        currentPage: currentPage ?? this.currentPage,
        currentPageTag: currentPageTag ?? this.currentPageTag,
      );

  @override
  List<Object?> get props => [currentPage, dynamicPages];
}

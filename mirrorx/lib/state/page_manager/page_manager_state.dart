part of 'page_manager_cubit.dart';

class PageManagerState extends Equatable {
  const PageManagerState({
    this.currentPageTag = "",
  });

  final String currentPageTag;

  PageManagerState copyWith({
    String? currentPageTag,
  }) =>
      PageManagerState(
        currentPageTag: currentPageTag ?? this.currentPageTag,
      );

  @override
  List<Object?> get props => [currentPageTag];
}

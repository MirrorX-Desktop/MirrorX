part of 'page_manager_cubit.dart';

class PageManagerState extends Equatable {
  const PageManagerState({
    this.currentPageTag = "",
    this.desktopIds = const [],
  });

  final String currentPageTag;
  final List<String> desktopIds;

  PageManagerState copyWith({
    String? currentPageTag,
    List<String>? desktopIds,
  }) =>
      PageManagerState(
        currentPageTag: currentPageTag ?? this.currentPageTag,
        desktopIds: desktopIds ?? this.desktopIds,
      );

  @override
  List<Object?> get props => [currentPageTag, desktopIds];
}

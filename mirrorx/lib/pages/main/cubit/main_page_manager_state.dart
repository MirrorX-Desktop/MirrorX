part of 'main_page_manager_cubit.dart';

class MainPageManagerState extends Equatable {
  const MainPageManagerState({
    this.registerTextures = const <String, RegisterTextureResponse>{},
    this.currentPage,
    this.currentPageTag,
  });

  final Map<String, RegisterTextureResponse> registerTextures;
  final Widget? currentPage;
  final String? currentPageTag;

  MainPageManagerState copyWith({
    Map<String, RegisterTextureResponse>? registerTextures,
    Widget? currentPage,
    String? currentPageTag,
  }) =>
      MainPageManagerState(
        registerTextures: registerTextures ?? this.registerTextures,
        currentPage: currentPage ?? this.currentPage,
        currentPageTag: currentPageTag ?? this.currentPageTag,
      );

  @override
  List<Object?> get props => [currentPage, registerTextures];
}

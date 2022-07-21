part of 'desktop_manager_cubit.dart';

class DesktopManagerState extends Equatable {
  const DesktopManagerState({
    this.desktopModels = const [],
    this.closedDesktops = const [],
  });

  final List<DesktopModel> desktopModels;
  final List<String> closedDesktops;

  DesktopManagerState copyWith({
    List<DesktopModel>? desktopModels,
    List<String>? closedDesktops,
  }) =>
      DesktopManagerState(
        desktopModels: desktopModels ?? this.desktopModels,
        closedDesktops: closedDesktops ?? this.closedDesktops,
      );

  @override
  List<Object?> get props => [desktopModels, closedDesktops];
}

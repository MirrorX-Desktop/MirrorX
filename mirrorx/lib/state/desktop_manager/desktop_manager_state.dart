part of 'desktop_manager_cubit.dart';

class DesktopManagerState extends Equatable {
  const DesktopManagerState({this.desktopModels = const []});

  final List<DesktopModel> desktopModels;

  DesktopManagerState copyWith({
    List<DesktopModel>? desktopModels,
  }) =>
      DesktopManagerState(
        desktopModels: desktopModels ?? this.desktopModels,
      );

  @override
  List<Object?> get props => [desktopModels];
}

import 'package:bloc/bloc.dart';
import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';
import 'package:mirrorx/model/desktop.dart';

part 'desktop_manager_state.dart';

class DesktopManagerCubit extends Cubit<DesktopManagerState> {
  DesktopManagerCubit() : super(const DesktopManagerState());

  void addDesktop(DesktopModel desktopModel) {
    emit(state.copyWith(
        desktopModels: List.from(state.desktopModels)..add(desktopModel)));
  }

  void removeDesktop(String remoteDeviceId) {
    emit(
      state.copyWith(
        desktopModels: List.from(state.desktopModels)
          ..removeWhere((e) => e.remoteDeviceId == remoteDeviceId),
        closedDesktops: List.from(state.closedDesktops)..remove(remoteDeviceId),
      ),
    );
  }

  void markDeskopClosed(String remoteDeviceId) {
    final containDesktop = state.desktopModels
            .indexWhere((m) => m.remoteDeviceId == remoteDeviceId) >=
        0;

    if (!containDesktop) {
      return;
    }

    if (!state.closedDesktops.contains(remoteDeviceId)) {
      emit(state.copyWith(
          closedDesktops: List.from(state.closedDesktops)
            ..add(remoteDeviceId)));
    }
  }
}

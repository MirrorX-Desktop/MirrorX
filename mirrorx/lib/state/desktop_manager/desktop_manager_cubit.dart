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

  void removeDesktop(String remoteDeviceID) {
    emit(
      state.copyWith(
        desktopModels: List.from(state.desktopModels)
          ..removeWhere((e) => e.remoteDeviceId == remoteDeviceID),
        closedDesktops: List.from(state.closedDesktops)..remove(remoteDeviceID),
      ),
    );
  }

  void markDeskopClosed(String remoteDeviceID) {
    if (!state.closedDesktops.contains(remoteDeviceID)) {
      emit(state.copyWith(
          closedDesktops: List.from(state.closedDesktops)
            ..add(remoteDeviceID)));
    }
  }
}

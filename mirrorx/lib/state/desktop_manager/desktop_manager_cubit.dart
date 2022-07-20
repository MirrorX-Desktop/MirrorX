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
    emit(state.copyWith(
        desktopModels: List.from(state.desktopModels)
          ..removeWhere((e) => e.remoteDeviceId != remoteDeviceID)));
  }
}

// final removePageIndex = state.desktopModels
//         .indexWhere((element) => element.remoteDeviceID == desktopPageTag);
//     if (removePageIndex == -1) {
//       return;
//     }

//     String switchPageTag;
//     if (state.desktopModels.length == 1) {
//       switchPageTag = "Connect";
//     } else {
//       if (removePageIndex == state.desktopModels.length - 1) {
//         // remove the last page, switch to the previous page
//         switchPageTag = state.desktopModels[removePageIndex - 1].remoteDeviceID;
//       } else {
//         // remove the first or middle page, switch to the next page
//         switchPageTag = state.desktopModels[removePageIndex + 1].remoteDeviceID;
//       }
//     }

//     switchPage(switchPageTag);

//     emit(state.copyWith(
//         desktopModels: List.from(state.desktopModels)
//           ..removeWhere(
//               (element) => element.remoteDeviceID == desktopPageTag)));
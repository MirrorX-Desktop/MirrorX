import 'package:bloc/bloc.dart';
import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';

part 'page_manager_state.dart';

class PageManagerCubit extends Cubit<PageManagerState> {
  PageManagerCubit() : super(const PageManagerState());

  void switchPage(String pageTag) {
    emit(state.copyWith(currentPageTag: pageTag));
  }

  bool isCurrent(String pageTag) {
    return state.currentPageTag == pageTag;
  }

  void addDesktopPage(int localDeviceId, int remoteDeviceId) {
    final id = "$localDeviceId@$remoteDeviceId";
    if (!state.desktopIds.contains(id)) {
      state.copyWith(
        currentPageTag: remoteDeviceId.toString(),
        desktopIds: state.desktopIds..add(id),
      );
    }
  }
}

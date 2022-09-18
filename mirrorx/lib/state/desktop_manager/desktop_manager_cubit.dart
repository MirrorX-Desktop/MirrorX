import 'dart:typed_data';

import 'package:bloc/bloc.dart';
import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';
import 'package:mirrorx/env/sdk/mirrorx_core.dart';
import 'package:mirrorx/env/sdk/mirrorx_core_sdk.dart';

part 'desktop_manager_state.dart';

class DesktopManagerCubit extends Cubit<DesktopManagerState> {
  DesktopManagerCubit() : super(const DesktopManagerState());

  void prepare(
    int localDeviceId,
    int remoteDeviceId,
    String visitCredentials,
    Uint8List openingKeyBytes,
    Uint8List openingNonceBytes,
    Uint8List sealingKeyBytes,
    Uint8List sealingNonceBytes,
  ) {
    final prepareInfo = DesktopPrepareInfo(
      localDeviceId,
      remoteDeviceId,
      visitCredentials,
      openingKeyBytes,
      openingNonceBytes,
      sealingKeyBytes,
      sealingNonceBytes,
    );

    emit(state.copyWith(
        desktopPrepareInfoLists: List.from(state.desktopPrepareInfoLists)
          ..add(prepareInfo)));
  }

  Future connect(DesktopPrepareInfo prepareInfo) async {
    await MirrorXCoreSDK.instance.endpointConnect(
      req: ConnectRequest(
        localDeviceId: prepareInfo.localDeviceId,
        remoteDeviceId: prepareInfo.remoteDeviceId,
        addr: "192.168.0.101:28001",
      ),
    );

    return await MirrorXCoreSDK.instance.endpointHandshake(
      req: HandshakeRequest(
        activeDeviceId: prepareInfo.localDeviceId,
        passiveDeviceId: prepareInfo.remoteDeviceId,
        visitCredentials: prepareInfo.visitCredentials,
        openingKeyBytes: prepareInfo.openingKeyBytes,
        openingNonceBytes: prepareInfo.openingNonceBytes,
        sealingKeyBytes: prepareInfo.sealingKeyBytes,
        sealingNonceBytes: prepareInfo.sealingNonceBytes,
      ),
    );
  }

  DesktopPrepareInfo? removePrepareInfo(int remoteDeviceId) {
    final prepareInfoIndex = state.desktopPrepareInfoLists
        .indexWhere((element) => element.remoteDeviceId == remoteDeviceId);

    if (prepareInfoIndex == -1) {
      return null;
    }

    final prepareInfo = state.desktopPrepareInfoLists[prepareInfoIndex];

    emit(state.copyWith(
        desktopPrepareInfoLists: List.from(state.desktopPrepareInfoLists)
          ..removeAt(prepareInfoIndex)));

    return prepareInfo;
  }

  void deviceInput(
    int localDeviceId,
    int remoteDeviceId,
    InputEvent event,
  ) async {
    await MirrorXCoreSDK.instance.endpointInput(
      req: InputRequest(
        activeDeviceId: localDeviceId,
        passiveDeviceId: remoteDeviceId,
        event: event,
      ),
    );
  }
}

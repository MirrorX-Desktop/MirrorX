import 'dart:developer';
import 'dart:typed_data';

import 'package:bloc/bloc.dart';
import 'package:equatable/equatable.dart';
import 'package:mirrorx/env/sdk/mirrorx_core.dart';
import 'package:mirrorx/env/sdk/mirrorx_core_sdk.dart';
import 'package:texture_render/model.dart';
import 'package:texture_render/texture_render.dart';

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

  Future connectAndNegotiate(DesktopPrepareInfo prepareInfo) async {
    RegisterTextureResponse? registerTextureResponse;

    try {
      await MirrorXCoreSDK.instance.endpointConnect(
        req: ConnectRequest(
          localDeviceId: prepareInfo.localDeviceId,
          remoteDeviceId: prepareInfo.remoteDeviceId,
          addr: "192.168.0.101:28001",
        ),
      );

      await MirrorXCoreSDK.instance.endpointHandshake(
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

      final negotiateVisitDesktopParamsResponse =
          await MirrorXCoreSDK.instance.endpointNegotiateVisitDesktopParams(
        req: NegotiateVisitDesktopParamsRequest(
          activeDeviceId: prepareInfo.localDeviceId,
          passiveDeviceId: prepareInfo.remoteDeviceId,
        ),
      );

      registerTextureResponse = await TextureRender.instance.registerTexture();

      final mediaStream = MirrorXCoreSDK.instance.endpointNegotiateFinished(
        req: NegotiateFinishedRequest(
          activeDeviceId: prepareInfo.localDeviceId,
          passiveDeviceId: prepareInfo.remoteDeviceId,
          expectFrameRate: 60,
          textureId: registerTextureResponse.textureId,
          // videoTexturePointer: registerTextureResponse.videoTexturePointer,
          // updateFrameCallbackPointer:
          //     registerTextureResponse.updateFrameCallbackPointer,
        ),
      );

      mediaStream.listen(
        (event) => onMediaStreamData(prepareInfo.remoteDeviceId, event),
        onError: (err) => onMediaStreamError(prepareInfo.remoteDeviceId, err),
        onDone: () => onMediaStreamDone(prepareInfo.remoteDeviceId),
      );

      emit(
        state.copyWith(
          desktopInfoLists: List.from(state.desktopInfoLists)
            ..add(
              DesktopInfo(
                prepareInfo.localDeviceId,
                prepareInfo.remoteDeviceId,
                negotiateVisitDesktopParamsResponse.monitorId,
                negotiateVisitDesktopParamsResponse.monitorWidth,
                negotiateVisitDesktopParamsResponse.monitorHeight,
                registerTextureResponse.textureId,
              ),
            ),
        ),
      );
    } catch (err) {
      log("negotiate $err");
      if (registerTextureResponse != null) {
        await TextureRender.instance.deregisterTexture(
          registerTextureResponse.textureId,
        );
      }
    }
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
    // await MirrorXCoreSDK.instance.endpointInput(
    //   req: InputRequest(
    //     activeDeviceId: localDeviceId,
    //     passiveDeviceId: remoteDeviceId,
    //     event: event,
    //   ),
    // );
  }

  void onMediaStreamData(int remoteDeviceId, FlutterMediaMessage message) {
    message.when(
        video: (videoFrameBuffer) async {
          log("on video frame buffer");
          await TextureRender.instance.sendVideoFrameBuffer(videoFrameBuffer);
        },
        audio: (a, b, audioBuffer) {});
  }

  void onMediaStreamError(int remoteDeviceId, Object err) {
    log("onMediaStreamError: $err");
  }

  void onMediaStreamDone(int remoteDeviceId) {
    log("onMediaStreamDone");
  }
}

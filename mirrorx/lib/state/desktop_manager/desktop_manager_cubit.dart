import 'dart:async';
import 'dart:developer';
import 'dart:typed_data';

import 'package:bloc/bloc.dart';
import 'package:equatable/equatable.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:mirrorx/env/sdk/mirrorx_core.dart';
import 'package:mirrorx/env/sdk/mirrorx_core_sdk.dart';
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
    int? textureId;

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

      textureId = await TextureRender.instance.registerTexture();
      if (textureId == null) {
        return Future.error("register textureId failed");
      }

      final mediaStream = MirrorXCoreSDK.instance.endpointNegotiateFinished(
        req: NegotiateFinishedRequest(
          activeDeviceId: prepareInfo.localDeviceId,
          passiveDeviceId: prepareInfo.remoteDeviceId,
          expectFrameRate: 60,
          textureId: textureId,
        ),
      );

      mediaStream.listen(
        (event) => onMediaStreamData(prepareInfo.remoteDeviceId, event),
        onError: (err) => onMediaStreamError(prepareInfo.remoteDeviceId, err),
        onDone: () => onMediaStreamDone(prepareInfo.remoteDeviceId),
      );

      final desktopId =
          DesktopId(prepareInfo.localDeviceId, prepareInfo.remoteDeviceId);

      final desktopInfo = DesktopInfo(
        prepareInfo.localDeviceId,
        prepareInfo.remoteDeviceId,
        negotiateVisitDesktopParamsResponse.monitorId,
        negotiateVisitDesktopParamsResponse.monitorWidth,
        negotiateVisitDesktopParamsResponse.monitorHeight,
        textureId,
        BoxFit.scaleDown,
        FilterQuality.low,
      );

      final newDesktopInfos =
          Map<DesktopId, DesktopInfo>.from(state.desktopInfoLists)
            ..[desktopId] = desktopInfo;

      emit(state.copyWith(desktopInfoLists: newDesktopInfos));

      log("${state.desktopInfoLists}");
    } catch (err) {
      if (textureId != null) {
        await TextureRender.instance.deregisterTexture(textureId);
      }

      return Future.error(err);
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

  void deviceInput(DesktopId desktopId, InputEvent event) async {
    await MirrorXCoreSDK.instance.endpointInput(
      req: InputRequest(
        activeDeviceId: desktopId.localDeviceId,
        passiveDeviceId: desktopId.remoteDeviceId,
        event: event,
      ),
    );
  }

  void updateBoxFit(DesktopId desktopId) {
    final oldDesktopInfo = state.desktopInfoLists[desktopId];

    if (oldDesktopInfo != null) {
      final newDesktopInfo = oldDesktopInfo.copyWith(
          boxFit: oldDesktopInfo.boxFit == BoxFit.none
              ? BoxFit.scaleDown
              : BoxFit.none);

      final newDesktopInfos =
          Map<DesktopId, DesktopInfo>.from(state.desktopInfoLists)
            ..[desktopId] = newDesktopInfo;

      emit(state.copyWith(desktopInfoLists: newDesktopInfos));
    }
  }

  void updateFilterQuality(
      DesktopId desktopId, FilterQuality newFilterQuality) {
    final oldDesktopInfo = state.desktopInfoLists[desktopId];

    if (oldDesktopInfo != null &&
        oldDesktopInfo.filterQuality != newFilterQuality) {
      final newDesktopInfo =
          oldDesktopInfo.copyWith(filterQuality: newFilterQuality);

      final newDesktopInfos =
          Map<DesktopId, DesktopInfo>.from(state.desktopInfoLists)
            ..[desktopId] = newDesktopInfo;

      emit(state.copyWith(desktopInfoLists: newDesktopInfos));
    }
  }

  void onMediaStreamData(int remoteDeviceId, FlutterMediaMessage message) {
    message.when(
        video: (videoFrameBuffer) {
          TextureRender.instance.sendVideoFrameBuffer(videoFrameBuffer);
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

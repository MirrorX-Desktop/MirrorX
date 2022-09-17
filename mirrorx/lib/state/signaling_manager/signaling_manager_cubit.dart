import 'package:equatable/equatable.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mirrorx/env/sdk/mirrorx_core.dart';
import 'package:mirrorx/env/sdk/mirrorx_core_sdk.dart';
import 'package:mirrorx/env/utility/error_notifier.dart';
import 'package:mirrorx/env/utility/rng.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';

part 'signaling_manager_state.dart';

class SignalingManagerCubit extends Cubit<SignalingManagerState> {
  SignalingManagerCubit(BuildContext context, this._configPath)
      : _snackBarNotifier = SnackBarNotifier(context),
        _dialogNotifier = DialogNotifier(context),
        super(const SignalingManagerState()) {
    // initial first connect
    Future.microtask(connect);
  }

  final String _configPath;
  final SnackBarNotifier _snackBarNotifier;
  final DialogNotifier _dialogNotifier;
  Stream<PublishMessage>? _subscribeStream;

  Future connect({String? domain}) async {
    try {
      emit(
          state.copyWith(connectionState: SignalingConnectionState.connecting));

      final connectDomain = domain ??
          await MirrorXCoreSDK.instance.readPrimaryDomain(path: _configPath);

      DomainConfig? domainConfig = connectDomain != null
          ? await MirrorXCoreSDK.instance
              .readDomainConfig(path: _configPath, domain: connectDomain)
          : null;

      String uri = domainConfig?.uri ?? "http://192.168.0.101:28000";

      await MirrorXCoreSDK.instance.signalingDial(req: DialRequest(uri: uri));

      final registerResponse = await MirrorXCoreSDK.instance.signalingRegister(
        req: RegisterRequest(
          deviceId: domainConfig?.deviceId,
          deviceFingerPrint: domainConfig?.deviceFingerPrint,
        ),
      );

      final newDomainConfig = DomainConfig(
        uri: uri,
        deviceId: registerResponse.deviceId,
        deviceFingerPrint: registerResponse.deviceFingerPrint,
        devicePassword: domainConfig?.devicePassword ?? generatePassword(),
      );

      await MirrorXCoreSDK.instance.saveDomainConfig(
        path: _configPath,
        domain: registerResponse.domain,
        value: newDomainConfig,
      );

      final primaryDomain =
          await MirrorXCoreSDK.instance.readPrimaryDomain(path: _configPath);
      if (primaryDomain == null) {
        await MirrorXCoreSDK.instance.savePrimaryDomain(
            path: _configPath, value: registerResponse.domain);
      }

      _subscribeStream = MirrorXCoreSDK.instance.signalingSubscribe(
        req: SubscribeRequest(
          localDeviceId: newDomainConfig.deviceId,
          deviceFingerPrint: newDomainConfig.deviceFingerPrint,
          configPath: _configPath,
        ),
      );

      if (_subscribeStream == null) {
        throw Exception("signaling subscribe returns null stream");
      }

      _subscribeStream?.listen(
        onSubscribeStreamData,
        onError: onSubscribeStreamError,
        onDone: onSubscribeStreamDone,
      );

      emit(state.copyWith(
        connectionState: SignalingConnectionState.connected,
        domain: registerResponse.domain,
        domainConfig: newDomainConfig,
      ));
    } catch (err, stackTrace) {
      emit(state.copyWith(
        connectionState: SignalingConnectionState.disconnected,
      ));
      _snackBarNotifier.notifyError(
        (context) => "Connect to Signaling Server failed",
        error: err,
        stackTrace: stackTrace,
      );
    }
  }

  Future disconnect() async {
    await MirrorXCoreSDK.instance.signalingDisconnect();
    emit(state.copyWith(
      connectionState: SignalingConnectionState.disconnected,
    ));
  }

  Future<VisitResponse> visit(int remoteDeviceId) async {
    final domain = state.domain;
    final domainConfig = state.domainConfig;

    if (domain == null) {
      return Future.error("domain is empty");
    }

    if (domainConfig == null) {
      return Future.error("domain config is empty");
    }

    return await MirrorXCoreSDK.instance.signalingVisit(
      req: VisitRequest(
        domain: domain,
        localDeviceId: domainConfig.deviceId,
        remoteDeviceId: remoteDeviceId,
        resourceType: ResourceType.Desktop,
      ),
    );
  }

  Future<KeyExchangeResponse> keyExchange(
      String password, int remoteDeviceId) async {
    final domain = state.domain;
    final domainConfig = state.domainConfig;

    if (domain == null) {
      return Future.error("domain is empty");
    }

    if (domainConfig == null) {
      return Future.error("domain config is empty");
    }

    try {
      return await MirrorXCoreSDK.instance.signalingKeyExchange(
        req: KeyExchangeRequest(
          domain: domain,
          localDeviceId: domainConfig.deviceId,
          remoteDeviceId: remoteDeviceId,
          password: password,
        ),
      );
    } catch (err, stackTrace) {
      // _errorNotifier.notifyError(error: err, stackTrace: stackTrace);
      return Future.error(err, stackTrace);
    }
  }

  void updateDevicePassword(String? newPassword) async {
    try {
      if (state.domain != null && state.domainConfig != null) {
        final newDomainConfigPassword = newPassword ?? generatePassword();
        final newDomainConfig = DomainConfig(
          uri: state.domainConfig!.uri,
          deviceId: state.domainConfig!.deviceId,
          deviceFingerPrint: state.domainConfig!.deviceFingerPrint,
          devicePassword: newDomainConfigPassword,
        );

        await MirrorXCoreSDK.instance.saveDomainConfig(
            path: _configPath, domain: state.domain!, value: newDomainConfig);

        emit(state.copyWith(domainConfig: newDomainConfig));
      }
    } catch (err, stackTrace) {
      _snackBarNotifier.notifyError(
        (context) => "Update password failed",
        error: err,
        stackTrace: stackTrace,
      );
    }
  }

  void onSubscribeStreamData(PublishMessage message) {
    message.when(
      streamClosed: handlePublishMessageStreamClosed,
      visitRequest: handlePublishMessageVisitRequest,
    );
  }

  void onSubscribeStreamError(Object object, StackTrace stackTrace) {}

  void onSubscribeStreamDone() {}

  void handlePublishMessageStreamClosed() {
    _snackBarNotifier.notifyError((context) => "subscribe stream has closed");
  }

  void handlePublishMessageVisitRequest(
    int activeDeviceId,
    int passiveDeviceId,
    ResourceType resourceType,
  ) async {
    final allow = await _dialogNotifier.popupDialog(
      contentBuilder: (context) {
        return Text(
            "$activeDeviceId want to visit your ${resourceType == ResourceType.Desktop ? "Desktop" : "Files"}");
      },
      actionBuilder: (context, navState) {
        return [
          TextButton(
            onPressed: () {
              navState.pop(true);
            },
            child: Text(AppLocalizations.of(context)!.dialogAllow),
          ),
          TextButton(
            onPressed: () {
              navState.pop(false);
            },
            child: Text(AppLocalizations.of(context)!.dialogDisallow),
          ),
        ];
      },
    );

    try {
      await MirrorXCoreSDK.instance.signalingVisitReply(
        req: VisitReplyRequest(
          domain: state.domain ?? "",
          activeDeviceId: activeDeviceId,
          passiveDeviceId: passiveDeviceId,
          allow: allow,
        ),
      );
    } catch (err, stackTrace) {
      _snackBarNotifier.notifyError(
        (context) =>
            "reply visit request for active device '$activeDeviceId' failed",
        error: err,
        stackTrace: stackTrace,
      );
    }
  }
}

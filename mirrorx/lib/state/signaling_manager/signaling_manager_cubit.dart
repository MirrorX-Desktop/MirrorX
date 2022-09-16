import 'package:equatable/equatable.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mirrorx/env/sdk/mirrorx_core.dart';
import 'package:mirrorx/env/sdk/mirrorx_core_sdk.dart';
import 'package:mirrorx/env/utility/error_notifier.dart';
import 'package:mirrorx/env/utility/rng.dart';

part 'signaling_manager_state.dart';

class SignalingManagerCubit extends Cubit<SignalingManagerState> {
  SignalingManagerCubit(BuildContext context, this._configPath)
      : _errorNotifier = SnackBarNotifier(context),
        super(const SignalingManagerState()) {
    // initial first connect
    Future.microtask(connect);
  }

  final String _configPath;
  final SnackBarNotifier _errorNotifier;
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

      String uri = domainConfig?.uri ?? "http://localhost:28000";

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

      emit(state.copyWith(
        connectionState: SignalingConnectionState.connected,
        domain: registerResponse.domain,
        domainConfig: newDomainConfig,
      ));
    } catch (err, stackTrace) {
      emit(state.copyWith(
        connectionState: SignalingConnectionState.disconnected,
      ));
      _errorNotifier.notifyError(
        "Connect to Signaling Server failed",
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
      _errorNotifier.notifyError(
        "Update password failed",
        error: err,
        stackTrace: stackTrace,
      );
    }
  }
}

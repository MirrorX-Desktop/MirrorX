part of 'mirrorx_core_bloc.dart';

enum MirrorXCoreStateStatus { initial, loading, success, failure }

extension MirrorXCoreStateStatusExtension on MirrorXCoreStateStatus {
  bool get isInitial => this == MirrorXCoreStateStatus.initial;
  bool get isLoading => this == MirrorXCoreStateStatus.loading;
  bool get isSuccess => this == MirrorXCoreStateStatus.success;
  bool get isFailure => this == MirrorXCoreStateStatus.failure;
}

class MirrorXCoreState extends Equatable {
  const MirrorXCoreState({
    this.core,
    this.status = MirrorXCoreStateStatus.initial,
    this.lastError,
    this.deviceId,
    this.password,
  });

  final MirrorXCoreImpl? core;
  final MirrorXCoreStateStatus status;
  final Object? lastError;

  final String? deviceId;
  final String? password;

  MirrorXCoreState copyWith({
    MirrorXCoreImpl? core,
    MirrorXCoreStateStatus? status,
    Object? lastError,
    String? deviceId,
    String? password,
  }) =>
      MirrorXCoreState(
        core: core ?? this.core,
        status: status ?? this.status,
        lastError: lastError ?? this.lastError,
        deviceId: deviceId ?? this.deviceId,
        password: password ?? this.password,
      );

  @override
  List<Object?> get props => [core, status, deviceId, password];
}

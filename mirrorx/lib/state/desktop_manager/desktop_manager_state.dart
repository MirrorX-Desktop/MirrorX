part of 'desktop_manager_cubit.dart';

class DesktopManagerState extends Equatable {
  const DesktopManagerState({
    this.desktopPrepareInfoLists = const [],
    this.desktopInfoLists = const [],
  });

  final List<DesktopPrepareInfo> desktopPrepareInfoLists;
  final List<DesktopInfo> desktopInfoLists;

  DesktopManagerState copyWith({
    List<DesktopPrepareInfo>? desktopPrepareInfoLists,
    List<DesktopInfo>? desktopInfoLists,
  }) =>
      DesktopManagerState(
        desktopPrepareInfoLists:
            desktopPrepareInfoLists ?? this.desktopPrepareInfoLists,
        desktopInfoLists: desktopInfoLists ?? this.desktopInfoLists,
      );

  @override
  List<Object?> get props => [desktopPrepareInfoLists, desktopInfoLists];
}

class DesktopPrepareInfo {
  final int localDeviceId;
  final int remoteDeviceId;
  final String visitCredentials;
  final Uint8List openingKeyBytes;
  final Uint8List openingNonceBytes;
  final Uint8List sealingKeyBytes;
  final Uint8List sealingNonceBytes;
  // final OperatingSystemType osType;
  // final int monitorWidth;
  // final int monitorHeight;
  // final int textureID;
  // final int videoTexturePointer;
  // final int updateFrameCallbackPointer;
  // final StreamSubscription<void> subscription;

  DesktopPrepareInfo(
    this.localDeviceId,
    this.remoteDeviceId,
    this.visitCredentials,
    this.openingKeyBytes,
    this.openingNonceBytes,
    this.sealingKeyBytes,
    this.sealingNonceBytes,
  );
}

class DesktopInfo {
  final int localDeviceId;
  final int remoteDeviceId;
  final String monitorId;
  final int monitorWidth;
  final int monitorHeight;
  final int textureId;

  DesktopInfo(
    this.localDeviceId,
    this.remoteDeviceId,
    this.monitorId,
    this.monitorWidth,
    this.monitorHeight,
    this.textureId,
  );
}

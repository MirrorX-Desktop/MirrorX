part of 'desktop_manager_cubit.dart';

class DesktopManagerState extends Equatable {
  const DesktopManagerState({
    this.desktopPrepareInfoLists = const [],
    this.closedDesktops = const [],
  });

  final List<DesktopPrepareInfo> desktopPrepareInfoLists;
  final List<String> closedDesktops;

  DesktopManagerState copyWith({
    List<DesktopPrepareInfo>? desktopPrepareInfoLists,
    List<String>? closedDesktops,
  }) =>
      DesktopManagerState(
        desktopPrepareInfoLists:
            desktopPrepareInfoLists ?? this.desktopPrepareInfoLists,
        closedDesktops: closedDesktops ?? this.closedDesktops,
      );

  @override
  List<Object?> get props => [desktopPrepareInfoLists, closedDesktops];
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

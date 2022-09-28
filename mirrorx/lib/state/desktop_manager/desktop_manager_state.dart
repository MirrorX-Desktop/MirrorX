part of 'desktop_manager_cubit.dart';

class DesktopId {
  final int localDeviceId;
  final int remoteDeviceId;

  const DesktopId(this.localDeviceId, this.remoteDeviceId);

  @override
  bool operator ==(Object other) {
    return other is DesktopId &&
        localDeviceId == other.localDeviceId &&
        remoteDeviceId == other.remoteDeviceId;
  }

  @override
  int get hashCode => 0;

  @override
  String toString() {
    return "$localDeviceId@$remoteDeviceId";
  }
}

class DesktopManagerState extends Equatable {
  const DesktopManagerState({
    this.desktopPrepareInfoLists = const [],
    this.desktopInfoLists = const <DesktopId, DesktopInfo>{},
  });

  final List<DesktopPrepareInfo> desktopPrepareInfoLists;
  final Map<DesktopId, DesktopInfo> desktopInfoLists;

  DesktopManagerState copyWith({
    List<DesktopPrepareInfo>? desktopPrepareInfoLists,
    Map<DesktopId, DesktopInfo>? desktopInfoLists,
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

class DesktopInfo extends Equatable {
  final int localDeviceId;
  final int remoteDeviceId;
  final String monitorId;
  final int monitorWidth;
  final int monitorHeight;
  final int textureId;
  final BoxFit boxFit;
  final FilterQuality filterQuality;

  const DesktopInfo(
    this.localDeviceId,
    this.remoteDeviceId,
    this.monitorId,
    this.monitorWidth,
    this.monitorHeight,
    this.textureId,
    this.boxFit,
    this.filterQuality,
  );

  DesktopInfo copyWith({
    String? monitorId,
    int? monitorWidth,
    int? monitorHeight,
    BoxFit? boxFit,
    FilterQuality? filterQuality,
  }) =>
      DesktopInfo(
        localDeviceId,
        remoteDeviceId,
        monitorId ?? this.monitorId,
        monitorWidth ?? this.monitorWidth,
        monitorHeight ?? this.monitorHeight,
        textureId,
        boxFit ?? this.boxFit,
        filterQuality ?? this.filterQuality,
      );

  @override
  List<Object?> get props =>
      [monitorId, monitorWidth, monitorHeight, boxFit, filterQuality];
}

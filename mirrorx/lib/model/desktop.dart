class DesktopModel {
  final String remoteDeviceID;
  final int textureID;
  final int videoTexturePointer;
  final int updateFrameCallbackPointer;
  bool alreadyPrepared = false;

  DesktopModel({
    required this.remoteDeviceID,
    required this.textureID,
    required this.videoTexturePointer,
    required this.updateFrameCallbackPointer,
  });
}

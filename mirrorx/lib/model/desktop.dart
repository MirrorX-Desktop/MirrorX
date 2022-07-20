import 'package:mirrorx/env/sdk/mirrorx_core.dart';

class DesktopModel {
  final String remoteDeviceId;
  final int monitorWidth;
  final int monitorHeight;
  final int textureID;
  final int videoTexturePointer;
  final int updateFrameCallbackPointer;

  DesktopModel({
    required this.remoteDeviceId,
    required this.monitorWidth,
    required this.monitorHeight,
    required this.textureID,
    required this.videoTexturePointer,
    required this.updateFrameCallbackPointer,
  });
}

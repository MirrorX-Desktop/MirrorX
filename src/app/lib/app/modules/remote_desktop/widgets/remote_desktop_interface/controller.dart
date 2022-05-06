import 'dart:developer';

import 'package:get/get.dart';
import 'package:mirrorx/app/controllers/mirrorx_core.dart';
import 'package:video_texture/video_texture.dart';

class RemoteDesktopInterfaceController extends GetxController {
  final String remoteID;

  late MirrorXCoreController _sdk;
  int _textureID = 0;

  int get textureID => _textureID;

  RemoteDesktopInterfaceController(this.remoteID);

  @override
  void onInit() async {
    _sdk = Get.find();
    await registerStream(remoteID);
    super.onInit();
  }

  @override
  void onClose() {
    deregisterTexture();
    super.onClose();
  }

  Future<int> registerTexture() async {
    if (_textureID > 0) {
      return _textureID;
    }

    var newTextureID = await VideoTexture.registerTextureID();
    log("register texture: $newTextureID");

    if (newTextureID != null && newTextureID > 0) {
      _textureID = newTextureID;
      return _textureID;
    } else {
      return Future.error("register texture failed");
    }
  }

  Future<void> deregisterTexture() async {
    await VideoTexture.deregisterTextureID(textureID);
  }

  Future<void> registerStream(String remoteID) async {
    _sdk
        .getInstance()
        .desktopRegisterFrameStream(remoteDeviceId: remoteID)
        .listen((event) {
      log("stream: $event");
    });
  }
}

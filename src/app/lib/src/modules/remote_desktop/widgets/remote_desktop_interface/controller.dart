import 'dart:developer';

import 'package:app_plugin/app_plugin.dart';
import 'package:get/get.dart';
import 'package:app/src/controllers/mirrorx_core.dart';

class RemoteDesktopInterfaceController extends GetxController {
  final String remoteID;

  late MirrorXCoreController _sdk;
  int _textureID = 0;

  int get textureID => _textureID;

  RemoteDesktopInterfaceController(this.remoteID);

  @override
  void onInit() async {
    _sdk = Get.find();
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

    var newTextureID = await AppPlugin.videoTextureRegister();
    log("register texture: $newTextureID");
    _textureID = newTextureID;
    await _sdk.getInstance().beginVideo(textureId: _textureID);
    return _textureID;
  }

  Future<void> deregisterTexture() async {
    await AppPlugin.deregisterTextureID(textureID);
  }
}

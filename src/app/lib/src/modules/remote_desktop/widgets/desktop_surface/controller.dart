import 'dart:developer';

import 'package:app_plugin/app_plugin.dart';
import 'package:app_plugin/method_channel_model.dart';
import 'package:get/get.dart';
import 'package:app/src/controllers/mirrorx_core.dart';

class DesktopSurfaceController extends GetxController {
  final String remoteID;
  final MirrorXCoreController _sdk = Get.find();

  RegisterVideoTextureModel? _registerVideoTextureModel;

  DesktopSurfaceController(this.remoteID);

  @override
  void onClose() async {
    if (_registerVideoTextureModel != null) {
      await AppPlugin.deregisterVideoTexture(
          _registerVideoTextureModel!.textureID,
          _registerVideoTextureModel!.videoTexturePointer);
    }

    super.onClose();
  }

  Future<int> registerTexture() async {
    if (_registerVideoTextureModel != null) {
      return _registerVideoTextureModel!.textureID;
    }

    RegisterVideoTextureModel? model;

    try {
      model = await AppPlugin.registerVideoTexture();
      await _sdk.getInstance().beginVideo(
          textureId: model.textureID,
          videoTexturePtr: model.videoTexturePointer,
          updateFrameCallbackPtr: model.updateFrameCallbackPointer);
      _registerVideoTextureModel = model;
      return model.textureID;
    } catch (error) {
      if (model != null) {
        AppPlugin.deregisterVideoTexture(
            model.textureID, model.videoTexturePointer);
      }

      return Future.error(error);
    }
  }
}

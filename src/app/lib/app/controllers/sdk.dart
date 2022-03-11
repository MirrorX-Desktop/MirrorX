import 'dart:developer';

import 'package:get/get.dart';
import 'package:mirrorx_sdk/mirrorx_sdk.dart';

class SDKController extends GetxController {
  MirrorXSDK? _sdk;

  @override
  void onReady() async {
    _sdk = await initSDK();
    super.onReady();
  }

  MirrorXSDK? get sdk => _sdk;
}

import 'package:get/get.dart';
import 'package:mirrorx_sdk/bridge_generated.dart';
import 'package:mirrorx_sdk/mirrorx_sdk.dart';

class SDKController extends GetxController with StateMixin<MirrorXCore> {
  @override
  void onReady() async {
    await initMirrorXSDK();
    super.onReady();
  }

  Future<void> initMirrorXSDK() async {
    change(null, status: RxStatus.loading());

    final sdk = await initSDK();
    sdk == null
        ? change(null, status: RxStatus.error())
        : change(sdk, status: RxStatus.success());
  }

  MirrorXCore getSDKInstance() {
    if (state == null) {
      throw Exception("get sdk instance but it's null");
    }
    return state!;
  }
}

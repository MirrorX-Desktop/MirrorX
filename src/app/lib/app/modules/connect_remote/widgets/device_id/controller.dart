import 'package:get/get.dart';
import 'package:mirrorx/app/controllers/sdk.dart';

class DeviceIDController extends GetxController with StateMixin<String> {
  @override
  void onReady() async {
    await loadDeviceId();
    super.onReady();
  }

  Future<void> loadDeviceId() async {
    try {
      final sdk = Get.find<SDKController>();
      final deviceId = await sdk.getSDKInstance().configReadDeviceId();
      change(deviceId, status: RxStatus.success());
    } catch (e) {
      change(null, status: RxStatus.error(e.toString()));
    }
  }
}

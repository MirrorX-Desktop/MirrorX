import 'package:get/get.dart';
import 'package:mirrorx/app/controllers/sdk.dart';
import 'package:mirrorx/app/model/device_profile.dart';
import 'package:mirrorx_sdk/bridge_generated.dart';

class DeviceProfileController extends GetxController
    with StateMixin<DeviceProfileModel> {
  late SDKController _sdkController;

  String get deviceID => state!.deviceID;
  String get deviceToken => state!.deviceToken;

  @override
  void onInit() {
    _sdkController = Get.find<SDKController>();
    super.onInit();
  }

  @override
  void onReady() async {
    await fetchDeviceProfile();
    super.onReady();
  }

  Future<void> fetchDeviceProfile() async {
    change(null, status: RxStatus.loading());

    try {
      final results = await Future.wait([
        _fetchDeviceProfile(),
        Future.delayed(const Duration(seconds: 2)),
      ]);

      change(results[0],
          status: results[0] == null ? RxStatus.error() : RxStatus.success());
    } catch (err) {
      change(null, status: RxStatus.error());
    }
  }

  Future<DeviceProfileModel?> _fetchDeviceProfile() async {
    var deviceToken =
        await _sdkController.getSDKInstance().readConfig(key: "device_token");

    deviceToken = await _sdkController
        .getSDKInstance()
        .createOrUpdateToken(token: deviceToken);

    final splitted = deviceToken.split('.');
    if (splitted.length != 4) {
      change(null, status: RxStatus.error("invalid token format"));
      return null;
    }

    await _sdkController
        .getSDKInstance()
        .storeConfig(key: "device_token", value: deviceToken);

    return DeviceProfileModel(deviceID: splitted[0], deviceToken: deviceToken);
  }
}

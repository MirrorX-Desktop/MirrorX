import 'package:get/get.dart';
import 'package:mirrorx/app/controllers/sdk.dart';

class DevicePasswordController extends GetxController {
  late bool _visable;
  late bool _editing;
  late String _password;
  late SDKController _sdkController;

  bool get passwordVisable => _visable;
  bool get isEditing => _editing;
  String get password => _password;

  @override
  void onInit() {
    _visable = false;
    _editing = false;
    _password = "";
    _sdkController = Get.find<SDKController>();
    super.onInit();
  }

  @override
  void onReady() async {
    await fetchDevicePassword();
    super.onReady();
  }

  Future<void> fetchDevicePassword() async {
    try {
      final storedPassword = await _sdkController
          .getSDKInstance()
          .readConfig(key: "device_password");

      if (storedPassword == null || storedPassword == "") {
        _password = await _generateAndSaveNewDevicePassword();
      }

      _password = storedPassword!;
    } catch (err) {
      _password = await _generateAndSaveNewDevicePassword();
    }
    update();
  }

  void changeVisable() {
    _visable = !_visable;
    update();
  }

  void editing() {
    if (_editing) {
      // enter edit mode
// todo: commit
    } else {}
    _visable = false;
    _editing = !_editing;
    update();
  }

  Future<String> _generateAndSaveNewDevicePassword() async {
    final password =
        await _sdkController.getSDKInstance().generateDevicePassword();
    await _sdkController
        .getSDKInstance()
        .storeConfig(key: "device_password", value: password);
    return password;
  }
}

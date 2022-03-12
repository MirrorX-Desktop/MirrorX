import 'package:get/get.dart';
import 'package:mirrorx/app/controllers/device_profile.dart';
import 'package:mirrorx/app/controllers/page_view.dart';
import 'package:mirrorx/app/controllers/sdk.dart';

class InitialBindings implements Bindings {
  @override
  void dependencies() {
    Get.lazyPut(() => SDKController());
    Get.lazyPut(() => PageViewController());
    Get.lazyPut(() => DeviceProfileController());
  }
}

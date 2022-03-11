import 'package:get/get.dart';
import 'package:mirrorx/app/controllers/page_view.dart';
import 'package:mirrorx/app/controllers/sdk.dart';

class InitialBindings implements Bindings {
  @override
  void dependencies() {
    Get.put<SDKController>(SDKController());
    Get.put<PageViewController>(PageViewController());
  }
}

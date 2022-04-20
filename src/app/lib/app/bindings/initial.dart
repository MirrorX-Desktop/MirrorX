import 'package:get/get.dart';
import 'package:mirrorx/app/controllers/page_view.dart';
import 'package:mirrorx/app/controllers/mirrorx_core.dart';

class InitialBindings implements Bindings {
  @override
  void dependencies() {
    Get.lazyPut(() => MirrorXCoreController());
    Get.lazyPut(() => PageViewController());
  }
}

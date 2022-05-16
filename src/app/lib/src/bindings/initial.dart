import 'package:get/get.dart';
import 'package:app/src/controllers/page_view.dart';
import 'package:app/src/controllers/mirrorx_core.dart';

class InitialBindings implements Bindings {
  @override
  void dependencies() {
    Get.lazyPut(() => MirrorXCoreController());
    Get.lazyPut(() => PageViewController());
  }
}

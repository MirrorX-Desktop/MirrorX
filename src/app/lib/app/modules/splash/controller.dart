import 'package:get/get.dart';
import 'package:mirrorx/app/controllers/sdk.dart';
import 'package:mirrorx/app/routes/pages.dart';

class SplashController extends GetxController {
  @override
  void onReady() async {
    await Future.delayed(Duration(milliseconds: 2000));

    var _sdkController = Get.find<SDKController>();

    _sdkController.sdk == null
        ? Get.offNamed(Routes.error)
        : Get.offNamed(Routes.main);

    super.onReady();
  }
}

import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:mirrorx/app/controllers/sdk.dart';
import 'package:mirrorx/app/modules/error/page.dart';
import 'package:mirrorx/app/modules/main/page.dart';

class SplashPage extends StatelessWidget {
  const SplashPage({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Get.find<SDKController>().obx(
        (_) => const MainPage(),
        onLoading: const Center(child: CircularProgressIndicator()),
        onError: (_) => const ErrorPage(),
      ),
    );
  }
}

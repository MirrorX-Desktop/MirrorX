import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:app/src/controllers/mirrorx_core.dart';
import 'package:app/src/modules/error/page.dart';
import 'package:app/src/modules/main/page.dart';

class SplashPage extends StatelessWidget {
  const SplashPage({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Get.find<MirrorXCoreController>().obx(
      (_) => const MainPage(),
      onLoading: const Center(child: CircularProgressIndicator()),
      onError: (_) => const ErrorPage(),
    );
  }
}

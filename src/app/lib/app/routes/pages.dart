import 'package:get/get.dart';
import 'package:mirrorx/app/modules/error/page.dart';
import 'package:mirrorx/app/modules/main/page.dart';
import 'package:mirrorx/app/modules/splash/binding.dart';
import 'package:mirrorx/app/modules/splash/page.dart';
part './routes.dart';

abstract class AppPages {
  static final pages = [
    GetPage(
      name: Routes.splash,
      page: () => const SplashPage(),
      binding: SplashBinding(),
    ),
    GetPage(
      name: Routes.main,
      page: () => const MainPage(),
    ),
    GetPage(
      name: Routes.error,
      page: () => const ErrorPage(),
    ),
  ];
}

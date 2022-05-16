import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:app/src/bindings/initial.dart';
import 'package:app/src/core/lang/translation.dart';
import 'package:app/src/modules/splash/page.dart';
import 'dart:ui' as ui;

void main() async {
  WidgetsFlutterBinding.ensureInitialized();

  runApp(GetMaterialApp(
    showPerformanceOverlay: false,
    translations: Translation(),
    locale: ui.window.locale,
    fallbackLocale: const Locale('en'),
    initialBinding: InitialBindings(),
    debugShowCheckedModeBanner: false,
    title: "MirrorX",
    theme: ThemeData(
        backgroundColor: Colors.white,
        scrollbarTheme:
            ScrollbarThemeData(thickness: MaterialStateProperty.all(4)),
        fontFamily: "Microsoft YaHei"),
    defaultTransition: Transition.circularReveal,
    transitionDuration: const Duration(milliseconds: 1500),
    home: const SplashPage(),
  ));
}

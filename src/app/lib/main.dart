import 'package:easy_localization/easy_localization.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:mirrorx/components/navigator/navigator.dart';
import 'package:mirrorx/pages/launch.dart';
import 'package:provider/provider.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await EasyLocalization.ensureInitialized();

  runApp(EasyLocalization(
    supportedLocales: const [Locale('en'), Locale('zh')],
    path: "assets/translations",
    fallbackLocale: const Locale('en'),
    child: const MirrorxApp(),
  ));
}

class MirrorxApp extends StatelessWidget {
  const MirrorxApp({Key? key}) : super(key: key);

  static const String _title = "MirrorX";

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
        // showPerformanceOverlay: true,
        debugShowCheckedModeBanner: false,
        title: _title,
        theme: ThemeData(
          scaffoldBackgroundColor: Colors.white,
        ),
        localizationsDelegates: context.localizationDelegates,
        supportedLocales: context.supportedLocales,
        locale: context.locale,
        home: MultiProvider(providers: [
          // ChangeNotifierProvider(create: (context) => _appDDD),
          ChangeNotifierProvider(create: (context) => AppNavigator()),
        ], child: const LaunchPage())); //_initSDKPreProcess(context));
  }
}

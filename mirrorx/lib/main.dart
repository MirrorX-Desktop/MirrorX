import 'package:mirrorx/pages/loading/loading_page.dart';
import 'package:flutter/material.dart';
import 'dart:ui' as ui;
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:window_manager/window_manager.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await windowManager.ensureInitialized();

  const windowOptions = WindowOptions(
    size: Size(995, 636),
    minimumSize: Size(995, 636),
    backgroundColor: Colors.white,
    skipTaskbar: false,
    center: true,
  );

  windowManager.waitUntilReadyToShow(windowOptions, () async {
    await windowManager.show(inactive: true);
    // await windowManager.focus();
  });

  runApp(const App());
}

class App extends StatelessWidget {
  const App({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      showPerformanceOverlay: false,
      locale: ui.window.locale,
      debugShowCheckedModeBanner: false,
      localizationsDelegates: AppLocalizations.localizationsDelegates,
      supportedLocales: AppLocalizations.supportedLocales,
      title: "MirrorX",
      theme: ThemeData(
        useMaterial3: true,
        scrollbarTheme: ScrollbarTheme.of(context)
            .copyWith(thickness: MaterialStateProperty.all(4)),
        scaffoldBackgroundColor: Colors.white,
      ),
      home: const Scaffold(body: LoadingPage()),
    );
  }
}

import 'package:app/business/mirrorx_core/mirrorx_core_bloc.dart';
import 'package:app/pages/loading/loading_page.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'dart:ui' as ui;
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:window_manager/window_manager.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();

  windowManager.ensureInitialized();

  WindowOptions windowOptions = const WindowOptions(
    minimumSize: Size(995, 636),
    center: true,
    // titleBarStyle: TitleBarStyle.hidden,
  );

  windowManager.waitUntilReadyToShow(windowOptions, () async {
    await windowManager.show();
    await windowManager.focus();
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
        home: MultiBlocProvider(
          providers: [
            BlocProvider<MirrorXCoreBloc>(
                create: (context) => MirrorXCoreBloc()..add(MirrorXCoreInit()))
          ],
          child: const Scaffold(
            body: LoadingPage(),
          ),
        ));
  }
}

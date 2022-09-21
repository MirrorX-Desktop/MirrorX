import 'dart:developer';

import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter/material.dart';
import 'dart:ui' as ui;
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:mirrorx/pages/main/main_page.dart';
import 'package:mirrorx/state/desktop_manager/desktop_manager_cubit.dart';
import 'package:mirrorx/state/page_manager/page_manager_cubit.dart';
import 'package:mirrorx/state/signaling_manager/signaling_manager_cubit.dart';
import 'package:path_provider/path_provider.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  final supportDirectory = await getApplicationSupportDirectory();
  final configPath = "${supportDirectory.path}/mirrorx.db";
  log("config path: $configPath");
  runApp(App(configPath: configPath));
}

class App extends StatelessWidget {
  const App({required this.configPath, Key? key}) : super(key: key);

  final String configPath;

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
      home: Scaffold(
        body: MultiBlocProvider(
          providers: [
            BlocProvider(
              create: (context) => SignalingManagerCubit(context, configPath),
              lazy: false,
            ),
            BlocProvider(
              create: (context) => PageManagerCubit(),
              lazy: false,
            ),
            BlocProvider(
              create: (context) => DesktopManagerCubit(),
              lazy: false,
            ),
          ],
          child: const MainPage(),
        ),
      ),
    );
  }

  // Future<ConfigManagerCubit> initConfigManager() async {
  //   final primaryDomain =
  //       await MirrorXCoreSDK.instance.readPrimaryDomain(path: configPath);

  //   DomainConfig? primaryDomainConfig;
  //   if (primaryDomain != null) {
  //     primaryDomainConfig = await MirrorXCoreSDK.instance
  //         .readDomainConfig(path: configPath, domain: primaryDomain);
  //   }

  //   return ConfigManagerCubit(configPath, primaryDomain, primaryDomainConfig);
  // }
}

import 'package:easy_localization/easy_localization.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:mirrorx/components/navigator/navigator.dart';
import 'package:mirrorx/pages/launch.dart';
import 'package:mirrorx_sdk/mirrorx_sdk.dart';
import 'package:provider/provider.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await EasyLocalization.ensureInitialized();

  runApp(EasyLocalization(
    supportedLocales: const [Locale('en'), Locale('zh')],
    path: "assets/translations",
    fallbackLocale: const Locale('en'),
    child: const MirrorXApp(),
  ));
}

class MirrorXApp extends StatefulWidget {
  const MirrorXApp({Key? key}) : super(key: key);

  @override
  _MirrorXAppState createState() => _MirrorXAppState();
}

class _MirrorXAppState extends State<MirrorXApp> {
  // make sure sdk init success
  final _initSDKFuture = MirrorXSDK.getInstance();

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
          ChangeNotifierProvider(create: (context) => AppNavigator()),
        ], child: _mainDisplayContent())); //_initSDKPreProcess(context));
  }

  Widget _mainDisplayContent() {
    return FutureBuilder(
      future: _initSDKFuture,
      builder: (context, snapshot) {
        if (snapshot.connectionState != ConnectionState.done) {
          return const CircularProgressIndicator();
        }

        if (snapshot.hasError) {
          return _showSDKInitializeFailedDialog();
        }

        return const LaunchPage();
      },
    );
  }

  Widget _showSDKInitializeFailedDialog() {
    return AlertDialog(
      title: const Text(_title),
      content: Text(tr("init_sdk_failed_dialog_message")),
      actions: [
        TextButton(
            onPressed: _sdkInitializeFailedDisalogButtonClickOK,
            child: Text(tr("init_sdk_failed_dialog_ok")))
      ],
    );
  }

  void _sdkInitializeFailedDisalogButtonClickOK() {
    SystemChannels.platform.invokeMethod('SystemNavigator.pop');
  }
}

// class MirrorxApp extends StatelessWidget {
//   const MirrorxApp({Key? key}) : super(key: key);

//   static const String _title = "MirrorX";

//   @override
//   Widget build(BuildContext context) {
//     await MirrorXSDK.getInstance();
//     return MaterialApp(
//         // showPerformanceOverlay: true,
//         debugShowCheckedModeBanner: false,
//         title: _title,
//         theme: ThemeData(
//           scaffoldBackgroundColor: Colors.white,
//         ),
//         localizationsDelegates: context.localizationDelegates,
//         supportedLocales: context.supportedLocales,
//         locale: context.locale,
//         home: MultiProvider(providers: [
//           ChangeNotifierProvider(create: (context) => AppNavigator()),
//         ], child: const LaunchPage())); //_initSDKPreProcess(context));
//   }
// }

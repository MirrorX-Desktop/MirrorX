import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:mirrorx/app/bindings/initial.dart';
import 'package:mirrorx/app/core/lang/translation.dart';
import 'package:mirrorx/app/routes/pages.dart';
import 'dart:ui' as ui;

void main() async {
  WidgetsFlutterBinding.ensureInitialized();

  runApp(
    GetMaterialApp(
      showPerformanceOverlay: false,
      translations: Translation(),
      locale: ui.window.locale,
      fallbackLocale: const Locale('en'),
      initialBinding: InitialBindings(),
      debugShowCheckedModeBanner: false,
      title: "MirrorX",
      theme: ThemeData(
        backgroundColor: Colors.white,
      ),
      getPages: AppPages.pages,
      defaultTransition: Transition.circularReveal,
      transitionDuration: const Duration(milliseconds: 1500),
    ),
  );
}

// class MirrorXApp extends StatefulWidget {
//   const MirrorXApp({Key? key}) : super(key: key);

//   @override
//   _MirrorXAppState createState() => _MirrorXAppState();
// }

// class _MirrorXAppState extends State<MirrorXApp> {
//   // make sure sdk init success
//   final _initSDKFuture = MirrorXSDK.getInstance();

//   @override
//   Widget build(BuildContext context) {
//     return GetMaterialApp(
//       // showPerformanceOverlay: true,
//       debugShowCheckedModeBanner: false,
//       title: "MirrorX",
//       theme: ThemeData(
//         scaffoldBackgroundColor: Colors.white,
//       ),
//       localizationsDelegates: context.localizationDelegates,
//       supportedLocales: context.supportedLocales,
//       locale: context.locale,
//       getPages: AppPages.pages,
//     ); //_initSDKPreProcess(context));
//   }

//   Widget _mainDisplayContent() {
//     return FutureBuilder(
//       future: _initSDKFuture,
//       builder: (context, snapshot) {
//         if (snapshot.connectionState != ConnectionState.done) {
//           return const CircularProgressIndicator();
//         }

//         if (snapshot.hasError) {
//           return _showSDKInitializeFailedDialog();
//         }

//         return const LaunchPage();
//       },
//     );
//   }

//   Widget _showSDKInitializeFailedDialog() {
//     return AlertDialog(
//       title: const Text(_title),
//       content: Text(tr("init_sdk_failed_dialog_message")),
//       actions: [
//         TextButton(
//             onPressed: _sdkInitializeFailedDisalogButtonClickOK,
//             child: Text(tr("init_sdk_failed_dialog_ok")))
//       ],
//     );
//   }

//   void _sdkInitializeFailedDisalogButtonClickOK() {
//     SystemChannels.platform.invokeMethod('SystemNavigator.pop');
//   }
// }

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

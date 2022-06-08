// import 'package:mirrorx/src/controllers/page_view.dart';
// import 'package:flutter/material.dart';
// import 'package:get/get.dart';
// import 'package:mirrorx/src/controllers/mirrorx_core.dart';
// import 'package:mirrorx/src/modules/error/page.dart';
// import 'package:mirrorx/src/modules/main/page.dart';

// class SplashPage extends StatelessWidget {
//   const SplashPage({Key? key}) : super(key: key);

//   @override
//   Widget build(BuildContext context) {
//     return FutureBuilder(
//         future: init(),
//         builder: ((context, snapshot) {
//           if (snapshot.connectionState != ConnectionState.done) {
//             return const Center(child: CircularProgressIndicator());
//           } else {
//             if (snapshot.hasError) {
//               return ErrorPage(snapshot.error);
//             } else {
//               return const MainPage();
//             }
//           }
//         }));
//   }

//   Future<void> init() async {
//     Get.put(MirrorXCoreController());
//     Get.put(PageViewController());
//   }
// }

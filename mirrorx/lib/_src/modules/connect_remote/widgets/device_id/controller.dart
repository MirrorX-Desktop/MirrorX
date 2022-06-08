// import 'package:get/get.dart';
// import 'package:mirrorx/src/controllers/mirrorx_core.dart';

// class DeviceIDController extends GetxController with StateMixin<String> {
//   @override
//   void onReady() async {
//     await loadDeviceId();
//     super.onReady();
//   }

//   Future<void> loadDeviceId() async {
//     try {
//       final sdk = Get.find<MirrorXCoreController>();
//       final deviceId = await sdk.getInstance().configReadDeviceId();
//       change(deviceId, status: RxStatus.success());
//     } catch (e) {
//       change(null, status: RxStatus.failure(e.toString()));
//     }
//   }
// }

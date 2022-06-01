// import 'package:flutter/material.dart';
// import 'package:flutter/services.dart';
// import 'package:get/get.dart';
// import 'package:app/src/core/values/colors.dart';
// import 'package:app/src/modules/connect_remote/widgets/device_password/controller.dart';

// class DevicePassword extends GetView<DevicePasswordController> {
//   const DevicePassword({Key? key}) : super(key: key);

//   @override
//   Widget build(BuildContext context) {
//     final controller = Get.put(DevicePasswordController());

//     return SizedBox(
//       height: 48,
//       child: GetBuilder<DevicePasswordController>(
//         init: controller,
//         builder: (controller) => Row(
//           children: [
//             Expanded(
//               child: _buildEditingOrReadonly(),
//             ),
//             SizedBox(
//               height: 32,
//               width: 32,
//               child: controller.isEditing
//                   ? IconButton(
//                       tooltip:
//                           "connect_to_remote.device_password_edit_cancel".tr,
//                       onPressed: controller.cancelEditing,
//                       splashRadius: 14,
//                       splashColor: Colors.transparent,
//                       padding: EdgeInsets.zero,
//                       hoverColor: const Color.fromARGB(240, 220, 220, 220),
//                       iconSize: 16,
//                       icon: const Icon(Icons.close),
//                     )
//                   : IconButton(
//                       tooltip: controller.passwordVisable
//                           ? "connect_to_remote.device_password_hide".tr
//                           : "connect_to_remote.device_password_show".tr,
//                       onPressed: controller.changeVisable,
//                       splashRadius: 14,
//                       splashColor: Colors.transparent,
//                       padding: EdgeInsets.zero,
//                       hoverColor: const Color.fromARGB(240, 220, 220, 220),
//                       iconSize: 16,
//                       icon: Icon(
//                         controller.passwordVisable
//                             ? Icons.visibility_off
//                             : Icons.visibility,
//                       ),
//                     ),
//             ),
//             SizedBox(
//               height: 32,
//               width: 32,
//               child: IconButton(
//                 tooltip: controller.isEditing
//                     ? "connect_to_remote.device_password_edit_confirm".tr
//                     : "connect_to_remote.device_password_edit".tr,
//                 onPressed: controller.editOrCommitPassword,
//                 splashRadius: 14,
//                 splashColor: Colors.transparent,
//                 padding: EdgeInsets.zero,
//                 hoverColor: const Color.fromARGB(240, 220, 220, 220),
//                 iconSize: 16,
//                 icon: Icon(controller.isEditing ? Icons.check : Icons.edit),
//               ),
//             ),
//             SizedBox(
//               height: 32,
//               width: 32,
//               child: IconButton(
//                 tooltip: "connect_to_remote.device_password_random_generate".tr,
//                 onPressed: controller.generateNewRandomPassword,
//                 splashRadius: 14,
//                 splashColor: Colors.transparent,
//                 padding: EdgeInsets.zero,
//                 hoverColor: const Color.fromARGB(240, 220, 220, 220),
//                 iconSize: 16,
//                 icon: const Icon(Icons.lock_reset),
//               ),
//             ),
//           ],
//         ),
//       ),
//     );
//   }

//   Widget _buildEditingOrReadonly() {
//     if (!controller.isEditing) {
//       if (controller.passwordVisable) {
//         return Center(
//           child: SelectableText(
//             controller.password,
//             style: const TextStyle(fontSize: 18),
//           ),
//         );
//       } else {
//         return const Center(
//           child: Text(
//             "＊＊＊＊＊＊",
//             style: TextStyle(fontSize: 16),
//           ),
//         );
//       }
//     } else {
//       return TextField(
//         controller: controller.textController,
//         textAlignVertical: TextAlignVertical.center,
//         maxLength: 16,
//         maxLines: 1,
//         cursorColor: ColorValues.primaryColor,
//         decoration: InputDecoration(
//           counterText: "",
//           enabledBorder: UnderlineInputBorder(
//             borderSide: BorderSide(
//               color: controller.textController.text.length >= 8 &&
//                       controller.textController.text.length <= 16
//                   ? ColorValues.primaryColor
//                   : Colors.red,
//               width: 2,
//             ),
//           ),
//           focusedBorder: UnderlineInputBorder(
//             borderSide: BorderSide(
//               color: controller.textController.text.length >= 8 &&
//                       controller.textController.text.length <= 16
//                   ? ColorValues.primaryColor
//                   : Colors.red,
//               width: 2,
//             ),
//           ),
//         ),
//         autofocus: true,
//         inputFormatters: [
//           FilteringTextInputFormatter.deny(" "),
//           FilteringTextInputFormatter.deny("\n"),
//           FilteringTextInputFormatter.deny("\t"),
//           FilteringTextInputFormatter.deny("\r"),
//         ],
//         scrollPhysics: const NeverScrollableScrollPhysics(),
//         textAlign: TextAlign.center,
//         style: const TextStyle(fontSize: 18),
//         onChanged: (_) {
//           controller.update();
//         },
//       );
//     }
//   }
// }

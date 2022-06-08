// import 'dart:developer';

// import 'package:flutter/material.dart';
// import 'package:flutter/services.dart';
// import 'package:get/get.dart';

// // without 0, I, L, O
// const _allowInputKey = [
//   LogicalKeyboardKey.digit1,
//   LogicalKeyboardKey.digit2,
//   LogicalKeyboardKey.digit3,
//   LogicalKeyboardKey.digit4,
//   LogicalKeyboardKey.digit5,
//   LogicalKeyboardKey.digit6,
//   LogicalKeyboardKey.digit7,
//   LogicalKeyboardKey.digit8,
//   LogicalKeyboardKey.digit9,
//   LogicalKeyboardKey.numpad1,
//   LogicalKeyboardKey.numpad2,
//   LogicalKeyboardKey.numpad3,
//   LogicalKeyboardKey.numpad4,
//   LogicalKeyboardKey.numpad5,
//   LogicalKeyboardKey.numpad6,
//   LogicalKeyboardKey.numpad7,
//   LogicalKeyboardKey.numpad8,
//   LogicalKeyboardKey.numpad9,
//   LogicalKeyboardKey.keyA,
//   LogicalKeyboardKey.keyB,
//   LogicalKeyboardKey.keyC,
//   LogicalKeyboardKey.keyD,
//   LogicalKeyboardKey.keyE,
//   LogicalKeyboardKey.keyF,
//   LogicalKeyboardKey.keyG,
//   LogicalKeyboardKey.keyH,
//   LogicalKeyboardKey.keyJ,
//   LogicalKeyboardKey.keyK,
//   LogicalKeyboardKey.keyL,
//   LogicalKeyboardKey.keyM,
//   LogicalKeyboardKey.keyN,
//   LogicalKeyboardKey.keyP,
//   LogicalKeyboardKey.keyQ,
//   LogicalKeyboardKey.keyR,
//   LogicalKeyboardKey.keyS,
//   LogicalKeyboardKey.keyT,
//   LogicalKeyboardKey.keyU,
//   LogicalKeyboardKey.keyV,
//   LogicalKeyboardKey.keyW,
//   LogicalKeyboardKey.keyX,
//   LogicalKeyboardKey.keyY,
//   LogicalKeyboardKey.keyZ,
// ];

// class CharacterInputController extends GetxController {
//   late FocusScopeNode _focusScopeNode;
//   late List<TextEditingController> _textEditingControllers;

//   FocusScopeNode get focusScopeNode => _focusScopeNode;
//   List<TextEditingController> get textEditingControllers =>
//       _textEditingControllers;
//   String? get inputText => _textEditingControllers.map((e) => e.text).join();

//   @override
//   void onInit() {
//     _textEditingControllers = <TextEditingController>[];

//     for (int i = 0; i < 8; i++) {
//       final controller = TextEditingController();
//       _textEditingControllers.add(controller);
//     }

//     _focusScopeNode = _createFocusScopeNode();
//     super.onInit();
//   }

//   FocusScopeNode _createFocusScopeNode() {
//     return FocusScopeNode(onKeyEvent: (node, event) {
//       if (event.logicalKey == LogicalKeyboardKey.delete ||
//           event.logicalKey == LogicalKeyboardKey.backspace) {
//         for (int i = 0; i < 8; i++) {
//           if (_focusScopeNode.focusedChild ==
//               _focusScopeNode.children.elementAt(i)) {
//             final controller = _textEditingControllers[i];
//             if (controller.text.isEmpty &&
//                 _focusScopeNode.focusedChild !=
//                     _focusScopeNode.children.first) {
//               _focusScopeNode.previousFocus();
//               update();
//               return KeyEventResult.handled;
//             }
//             break;
//           }
//         }
//       }

//       // for input
//       if (_allowInputKey.any((element) => element == event.logicalKey)) {
//         for (int i = 0; i < 8; i++) {
//           if (_focusScopeNode.focusedChild ==
//               _focusScopeNode.children.elementAt(i)) {
//             final controller = _textEditingControllers[i];
//             if (_focusScopeNode.focusedChild != _focusScopeNode.children.last) {
//               if (controller.text.isNotEmpty) {
//                 _focusScopeNode.nextFocus();
//                 update();
//                 return KeyEventResult.handled;
//               }
//             }
//             break;
//           }
//         }
//       }

//       /// arrow-right key : move cursor to next
//       if (event is KeyUpEvent &&
//           event.logicalKey == LogicalKeyboardKey.arrowRight) {
//         if (_focusScopeNode.focusedChild != _focusScopeNode.children.last) {
//           _focusScopeNode.nextFocus();
//           update();
//           return KeyEventResult.handled;
//         }
//       }

//       /// arrow-right key : move cursor to previous
//       if (event is KeyUpEvent &&
//           event.logicalKey == LogicalKeyboardKey.arrowLeft) {
//         if (_focusScopeNode.focusedChild != _focusScopeNode.children.first) {
//           _focusScopeNode.previousFocus();
//           update();
//           return KeyEventResult.handled;
//         }
//       }

//       update();
//       return KeyEventResult.ignored;
//     });
//   }

//   @override
//   void onClose() {
//     _focusScopeNode.dispose();
//     for (var element in _textEditingControllers) {
//       element.dispose();
//     }
//     super.onClose();
//   }
// }

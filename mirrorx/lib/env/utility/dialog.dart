import 'package:flutter/material.dart';
import 'package:mirrorx/state/navigator_key.dart';

Future<T?> popupDialog<T>({
  required Widget Function(BuildContext) contentBuilder,
  required List<Widget>? Function(NavigatorState) actionBuilder,
}) async {
  return showGeneralDialog<T?>(
    context: navigatorKey.currentContext!,
    pageBuilder: (context, animationValue1, animationValue2) {
      return AlertDialog(
        title: const Text("MirrorX", textAlign: TextAlign.center),
        content: contentBuilder(navigatorKey.currentContext!),
        actions: actionBuilder(navigatorKey.currentState!),
      );
    },
    barrierDismissible: false,
    transitionBuilder: (context, animationValue1, animationValue2, child) {
      return Transform.scale(
        scale: animationValue1.value,
        child: Opacity(
          opacity: animationValue1.value,
          child: child,
        ),
      );
    },
    transitionDuration: kThemeAnimationDuration * 2,
  );
}

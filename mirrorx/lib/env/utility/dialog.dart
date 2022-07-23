import 'package:flutter/material.dart';

Future<T?> popupDialog<T>(
  BuildContext context, {
  required Widget Function(BuildContext) contentBuilder,
  required List<Widget>? Function(NavigatorState) actionBuilder,
}) async {
  return showGeneralDialog<T?>(
    context: context,
    pageBuilder: (context, animationValue1, animationValue2) {
      return AlertDialog(
        title: const Text("MirrorX", textAlign: TextAlign.center),
        content: contentBuilder(context),
        actions: actionBuilder(Navigator.of(context)),
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

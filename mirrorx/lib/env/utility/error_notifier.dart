import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';

class SnackBarNotifier {
  final BuildContext context;

  SnackBarNotifier(this.context);

  void notifyError(
    String Function(BuildContext context) messageBuilder, {
    Object? error,
    StackTrace? stackTrace,
  }) {
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        behavior: SnackBarBehavior.floating,
        content: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(messageBuilder(context)),
            Visibility(visible: error != null, child: Text(error.toString())),
            Visibility(
              visible: stackTrace != null,
              child: Text(
                stackTrace.toString(), // only print first line
              ),
            )
          ],
        ),
        duration: const Duration(days: 1),
        dismissDirection: DismissDirection.none,
        action: SnackBarAction(
          label: AppLocalizations.of(context)!.dialogOK,
          onPressed: () {
            ScaffoldMessenger.of(context).hideCurrentSnackBar();
          },
        ),
      ),
    );
  }
}

class DialogNotifier {
  final BuildContext context;

  DialogNotifier(this.context);

  Future<T?> popupDialog<T>({
    required Widget Function(BuildContext) contentBuilder,
    required List<Widget>? Function(BuildContext, NavigatorState) actionBuilder,
  }) async {
    return showGeneralDialog<T?>(
      context: context,
      pageBuilder: (context, animationValue1, animationValue2) {
        return AlertDialog(
          title: const Text("MirrorX", textAlign: TextAlign.center),
          content: contentBuilder(context),
          actions: actionBuilder(context, Navigator.of(context)),
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
}

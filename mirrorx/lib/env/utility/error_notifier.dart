import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';

class ErrorNotifier {
  final BuildContext context;

  ErrorNotifier(this.context);

  void notifyError({Object? error, StackTrace? stackTrace}) {
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        behavior: SnackBarBehavior.floating,
        content: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
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

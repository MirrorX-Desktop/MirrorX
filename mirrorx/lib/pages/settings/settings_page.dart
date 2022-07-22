import 'package:flutter/material.dart';
import 'package:mirrorx/pages/settings/widgets/language_selector.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';

class SettingsPage extends StatelessWidget {
  const SettingsPage({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(left: 16, right: 16, top: 16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Video Settings
          Text(
            AppLocalizations.of(context)!.settingsPageGroupVideoTitle,
            style: const TextStyle(fontSize: 36),
          ),
          const Divider(),
          Padding(
            padding: const EdgeInsets.only(left: 16),
            child: Column(
              children: const [],
            ),
          ),

          // Audio Settings
          Text(
            AppLocalizations.of(context)!.settingsPageGroupAudioTitle,
            style: const TextStyle(fontSize: 36),
          ),
          const Divider(),
          Padding(
            padding: const EdgeInsets.only(left: 16),
            child: Column(
              children: const [],
            ),
          ),

          // Input Settings
          Text(
            AppLocalizations.of(context)!.settingsPageGroupInputTitle,
            style: const TextStyle(fontSize: 36),
          ),
          const Divider(),
          Padding(
            padding: const EdgeInsets.only(left: 16),
            child: Column(
              children: const [],
            ),
          ),

          // Other Settings,
          Text(
            AppLocalizations.of(context)!.settingsPageGroupOtherTitle,
            style: const TextStyle(fontSize: 36),
          ),
          const Divider(),
          Padding(
            padding: const EdgeInsets.only(left: 16),
            child: Column(
              children: const [LanguageSelector(), Divider()],
            ),
          ),
        ],
      ),
    );
  }
}

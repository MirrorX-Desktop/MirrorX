import 'package:easy_localization/easy_localization.dart';
import 'package:flutter/material.dart';
import 'package:mirrorx/pages/page.dart';

class SettingsPage extends AppPage {
  const SettingsPage({Key? key, required String tag})
      : super(key: key, tag: tag);

  @override
  _SettingsPageState createState() => _SettingsPageState();
}

class _SettingsPageState extends State<SettingsPage> {
  @override
  Widget build(BuildContext context) {
    return Column(
      children: const [
        LanguageSettings(),
      ],
    );
  }
}

class LanguageSettings extends StatelessWidget {
  const LanguageSettings({Key? key}) : super(key: key);

  static final _languageMenuItems = [
    const DropdownMenuItem(child: Text("中文"), value: "zh"),
    const DropdownMenuItem(child: Text("English"), value: "en"),
  ];

  @override
  Widget build(BuildContext context) {
    return DropdownButton(
      value: context.locale.toString(),
      items: _languageMenuItems,
      onChanged: (String? value) {
        if (value != null) {
          context.setLocale(Locale(value));
        }
      },
    );
  }
}

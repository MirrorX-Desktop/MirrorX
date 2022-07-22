import 'package:flutter/material.dart';
import 'package:mirrorx/pages/settings/widgets/language_selector.dart';

class SettingsPage extends StatelessWidget {
  const SettingsPage({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Column(
      children: const [LanguageSelector()],
    );
  }
}

import 'package:app/business/page_manager/page.dart';
import 'package:flutter/material.dart';

class SettingsPage extends NavigationPage {
  const SettingsPage({Key? key})
      : super(key: key, title: "Settings", titleIcon: Icons.settings);

  @override
  Widget build(BuildContext context) {
    return const Center(child: Text("Settings is comming soon!"));
  }

  @override
  int getIndex() => 4;
}

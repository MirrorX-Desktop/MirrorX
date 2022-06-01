import 'package:app/business/page_manager/page.dart';
import 'package:flutter/material.dart';

class LanPage extends NavigationPage {
  const LanPage({Key? key})
      : super(key: key, title: "LAN", titleIcon: Icons.lan);

  @override
  Widget build(BuildContext context) {
    return const Center(child: Text("LAN Discovery is comming soon!"));
  }

  @override
  int getIndex() => 1;
}

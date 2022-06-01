import 'package:app/business/page_manager/page.dart';
import 'package:flutter/material.dart';

class HistoryPage extends NavigationPage {
  const HistoryPage({Key? key})
      : super(key: key, title: "History", titleIcon: Icons.history);

  @override
  Widget build(BuildContext context) {
    return const Center(child: Text("History is comming soon!"));
  }

  @override
  int getIndex() => 3;
}

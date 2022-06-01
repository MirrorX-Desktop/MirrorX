import 'package:app/business/page_manager/page.dart';
import 'package:flutter/material.dart';

class FilesPage extends NavigationPage {
  const FilesPage({Key? key})
      : super(key: key, title: "Files", titleIcon: Icons.drive_file_move_rtl);

  @override
  Widget build(BuildContext context) {
    return const Center(child: Text("Files Transfer is comming soon!"));
  }

  @override
  int getIndex() => 2;
}

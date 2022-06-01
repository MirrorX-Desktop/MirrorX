import 'package:flutter/material.dart';

abstract class NavigationPage extends StatelessWidget {
  final IconData titleIcon;
  final String title;

  const NavigationPage({Key? key, required this.titleIcon, required this.title})
      : super(key: key);

  int getIndex();
}

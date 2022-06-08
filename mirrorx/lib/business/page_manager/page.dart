import 'package:flutter/material.dart';

abstract class NavigationPage extends StatelessWidget {
  const NavigationPage({Key? key, required this.uniqueTag, required this.icon})
      : super(key: key);

  final String uniqueTag;

  final IconData icon;
}

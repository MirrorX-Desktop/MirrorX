import 'package:flutter/material.dart';

abstract class AppPage extends StatefulWidget {
  const AppPage({Key? key, required this.tag}) : super(key: key);

  final String tag;
}

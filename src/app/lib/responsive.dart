import 'package:flutter/widgets.dart';

class Responsive extends StatelessWidget {
  final Widget mobile;
  final Widget desktop;

  const Responsive({Key? key, required this.mobile, required this.desktop})
      : super(key: key);

  static bool isMobile(BuildContext context) =>
      MediaQuery.of(context).size.width <= 600;

  @override
  Widget build(BuildContext context) {
    return isMobile(context) ? mobile : desktop;
  }
}

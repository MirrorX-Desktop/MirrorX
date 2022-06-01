import 'package:app/pages/loading/loading_page.dart';
import 'package:flutter/material.dart';
import 'dart:ui' as ui;

void main() async {
  WidgetsFlutterBinding.ensureInitialized();

  runApp(const App());
}

class App extends StatelessWidget {
  const App({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      showPerformanceOverlay: false,
      locale: ui.window.locale,
      debugShowCheckedModeBanner: false,
      title: "MirrorX",
      theme: ThemeData(
        useMaterial3: true,
        scrollbarTheme: ScrollbarTheme.of(context)
            .copyWith(thickness: MaterialStateProperty.all(4)),
      ),
      home: Material(
        child: Container(
          color: Colors.white,
          child: const LoadingPage(),
        ),
      ),
    );
  }
}

import 'dart:core';
import 'dart:developer';

import 'package:flutter/material.dart';
import 'dart:async';

import 'package:flutter/services.dart';
import 'package:app_plugin/app_plugin.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({Key? key}) : super(key: key);

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  int _registerTextureID = 0;

  @override
  void initState() {
    super.initState();
    do_test();
    videoTextureRegister();
  }

  Future<void> videoTextureRegister() async {
    int textureID;

    try {
      textureID = await AppPlugin.videoTextureRegister();
    } catch (error) {
      textureID = -1;
    }

    if (!mounted) return;

    setState(() {
      _registerTextureID = textureID;
    });
  }

  Future<void> do_test() async {
    final res = await AppPlugin.testForSwift();
    log("resutl: $res");
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: const Text('Plugin example app'),
        ),
        body: Center(
          child: Text('texture id: $_registerTextureID\n'),
        ),
      ),
    );
  }
}

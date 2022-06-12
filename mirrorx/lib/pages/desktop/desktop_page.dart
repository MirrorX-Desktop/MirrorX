import 'dart:developer';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mirrorx/env/sdk/mirrorx_core_sdk.dart';
import 'package:texture_render/model.dart';
import 'package:texture_render/texture_render_platform_interface.dart';

class DesktopPage extends StatelessWidget {
  const DesktopPage({Key? key, required this.resp}) : super(key: key);

  final RegisterTextureResponse resp;

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Row(
          children: [
            TextButton(onPressed: () {}, child: Text("AAA")),
            TextButton(onPressed: () {}, child: Text("BBB"))
          ],
        ),
        Expanded(
          child: RepaintBoundary(
            child: Container(
              color: Colors.black,
              child: Center(
                child: AspectRatio(
                  aspectRatio: 16.0 / 9.0,
                  child: Texture(
                    textureId: resp.textureID,
                    freeze: true,
                    filterQuality: FilterQuality.none,
                  ),
                ),
              ),
            ),
          ),
        )
      ],
    );
  }
}

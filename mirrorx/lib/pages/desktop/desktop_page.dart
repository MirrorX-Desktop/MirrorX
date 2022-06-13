import 'dart:developer';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mirrorx/env/langs/tr.dart';
import 'package:mirrorx/env/sdk/mirrorx_core_sdk.dart';
import 'package:mirrorx/model/desktop.dart';
import 'package:texture_render/model.dart';
import 'package:texture_render/texture_render_platform_interface.dart';

class DesktopPage extends StatelessWidget {
  const DesktopPage({Key? key, required this.model}) : super(key: key);

  final DesktopModel model;

  @override
  Widget build(BuildContext context) {
    return model.alreadyPrepared
        ? _buildDesktopSurface()
        : FutureBuilder(
            future: prepare(),
            builder: (context, snapshot) {
              switch (snapshot.connectionState) {
                case ConnectionState.none:
                case ConnectionState.waiting:
                case ConnectionState.active:
                  return Center(
                    child: SizedBox(
                      width: 200,
                      height: 100,
                      child: Column(
                        children: [
                          const CircularProgressIndicator(),
                          Padding(
                            padding: const EdgeInsets.only(top: 16),
                            child: Text(Tr.of(context).desktopPagePreparing),
                          )
                        ],
                      ),
                    ),
                  );
                case ConnectionState.done:
                  model.alreadyPrepared = true;
                  return _buildDesktopSurface();
              }
            },
          );
  }

  Future<void> prepare() async {
    await MirrorXCoreSDK.instance.desktopStartMediaTransmission(
        remoteDeviceId: model.remoteDeviceID,
        textureId: model.textureID,
        videoTexturePtr: model.videoTexturePointer,
        updateFrameCallbackPtr: model.updateFrameCallbackPointer);
  }

  Widget _buildDesktopSurface() {
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
                    textureId: model.textureID,
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

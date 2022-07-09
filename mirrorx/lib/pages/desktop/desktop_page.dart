import 'dart:developer';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mirrorx/env/langs/tr.dart';
import 'package:mirrorx/env/sdk/mirrorx_core.dart';
import 'package:mirrorx/env/sdk/mirrorx_core_sdk.dart';
import 'package:mirrorx/model/desktop.dart';
import 'package:mirrorx/pages/desktop/widgets/desktop_render_box/desktop_render_box.dart';
import 'package:texture_render/model.dart';
import 'package:texture_render/texture_render_platform_interface.dart';
import 'package:flutter/foundation.dart';

class DesktopPage extends StatefulWidget {
  const DesktopPage({Key? key, required this.model}) : super(key: key);

  final DesktopModel model;

  @override
  _DesktopPageState createState() => _DesktopPageState();
}

class _DesktopPageState extends State<DesktopPage> {
  @override
  Widget build(BuildContext context) {
    return widget.model.alreadyPrepared
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
                  if (snapshot.hasError) {
                    return Center(
                      child: Text(snapshot.error.toString()),
                    );
                  }

                  widget.model.alreadyPrepared = true;
                  return _buildDesktopSurface();
              }
            },
          );
  }

  Future<void> prepare() async {
    final resp = await MirrorXCoreSDK.instance
        .endpointGetDisplayInfo(remoteDeviceId: widget.model.remoteDeviceID);

    final displayID = await _popupSelectMonitorDialog(resp.displays);

    if (displayID == null) {
      return Future.error("display Id is null");
    }

    var fps = 30;

    for (var display in resp.displays) {
      if (display.id == displayID) {
        fps = display.refreshRate;
        break;
      }
    }

    await MirrorXCoreSDK.instance.endpointStartMediaTransmission(
      remoteDeviceId: widget.model.remoteDeviceID,
      expectFps: fps,
      expectDisplayId: displayID,
      textureId: widget.model.textureID,
      videoTexturePtr: widget.model.videoTexturePointer,
      updateFrameCallbackPtr: widget.model.updateFrameCallbackPointer,
    );
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
          child: DesktopRenderBox(
            model: widget.model,
          ),
        )
      ],
    );
  }

  Future<String?> _popupSelectMonitorDialog(List<DisplayInfo> displays) {
    return showGeneralDialog<String?>(
      context: context,
      pageBuilder: (context, animationValue1, animationValue2) {
        return AlertDialog(
          title: const Text("MirrorX", textAlign: TextAlign.center),
          content: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              Text("选择显示器"),
              _buildMonitors(displays),
            ],
          ),
          actions: <Widget>[
            TextButton(
              child: Text(Tr.of(context).dialogCancel),
              onPressed: () {
                Navigator.of(context).pop(null);
              },
            ),
          ],
        );
      },
      barrierDismissible: false,
      transitionBuilder: (context, animationValue1, animationValue2, child) {
        return Transform.scale(
          scale: animationValue1.value,
          child: Opacity(
            opacity: animationValue1.value,
            child: child,
          ),
        );
      },
      transitionDuration: kThemeAnimationDuration * 2,
    );
  }

  Widget _buildMonitors(List<DisplayInfo> displays) {
    return Row(
      children: [
        for (var display in displays)
          IconButton(
            icon: Image.memory(display.screenShot),
            onPressed: () {
              Navigator.of(context).pop(display.id);
            },
          ),
      ],
    );
  }
}

import 'dart:developer';

import 'package:card_swiper/card_swiper.dart';
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
  StartMediaTransmissionResponse? _startMediaTransmissionResponse;
  BoxFit _fit = BoxFit.none;

  @override
  Widget build(BuildContext context) {
    return widget.model.alreadyPrepared
        ? _buildDesktopSurface(
            _startMediaTransmissionResponse!.screenWidth,
            _startMediaTransmissionResponse!.screenHeight,
          )
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

                  _startMediaTransmissionResponse =
                      snapshot.data as StartMediaTransmissionResponse;

                  widget.model.alreadyPrepared = true;

                  return _buildDesktopSurface(
                    _startMediaTransmissionResponse!.screenWidth,
                    _startMediaTransmissionResponse!.screenHeight,
                  );
              }
            },
          );
  }

  Future<StartMediaTransmissionResponse> prepare() async {
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

    return await MirrorXCoreSDK.instance.endpointStartMediaTransmission(
      remoteDeviceId: widget.model.remoteDeviceID,
      expectFps: fps,
      expectDisplayId: displayID,
      textureId: widget.model.textureID,
      videoTexturePtr: widget.model.videoTexturePointer,
      updateFrameCallbackPtr: widget.model.updateFrameCallbackPointer,
    );
  }

  Widget _buildDesktopSurface(int width, int height) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        Row(
          children: [
            // Text(widget.model.remoteDeviceID),
            // VerticalDivider(),
            Tooltip(
              message: "Raw Resolution",
              child: Container(
                width: 36,
                height: 36,
                padding: const EdgeInsets.all(3.0),
                child: TextButton(
                  onPressed: _handleBoxFitClick,
                  style: ButtonStyle(
                    shape: MaterialStateProperty.all(
                      RoundedRectangleBorder(
                          borderRadius: BorderRadius.circular(4.0)),
                    ),
                    padding: MaterialStateProperty.all(EdgeInsets.zero),
                    foregroundColor: MaterialStateProperty.all(Colors.black),
                  ),
                  child: _fit == BoxFit.none
                      ? const Icon(Icons.aspect_ratio)
                      : const Icon(Icons.fit_screen),
                ),
              ),
            ),
          ],
        ),
        Expanded(
          child: Container(
            color: Colors.black,
            child: DesktopRenderBox(
              model: widget.model,
              width: width,
              height: height,
              fit: _fit,
            ),
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
              ScreenShotSwiper(displays: displays),
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

  void _handleBoxFitClick() {
    setState(() {
      if (_fit == BoxFit.none) {
        _fit = BoxFit.scaleDown;
      } else {
        _fit = BoxFit.none;
      }
    });
  }
}

class ScreenShotSwiper extends StatefulWidget {
  const ScreenShotSwiper({Key? key, required this.displays}) : super(key: key);

  final List<DisplayInfo> displays;

  @override
  _ScreenShotSwiperState createState() => _ScreenShotSwiperState();
}

class _ScreenShotSwiperState extends State<ScreenShotSwiper> {
  int _selectedIndex = 0;

  @override
  Widget build(BuildContext context) {
    var monitorName = widget.displays[_selectedIndex].name;
    if (monitorName.isEmpty) {
      monitorName = "内建显示器";
    }
    return Column(
      children: [
        SizedBox(
          width: 500,
          height: 280,
          child: Swiper(
            itemCount: widget.displays.length,
            pagination: const SwiperPagination(
              builder: DotSwiperPaginationBuilder(
                activeColor: Colors.yellow,
                color: Colors.white,
              ),
            ),
            control: const SwiperControl(
                iconPrevious: Icons.chevron_left_rounded,
                iconNext: Icons.chevron_right_rounded,
                color: Colors.yellow,
                size: 60,
                padding: EdgeInsets.zero),
            indicatorLayout: PageIndicatorLayout.SCALE,
            onIndexChanged: (index) {
              setState(() {
                _selectedIndex = index;
              });
            },
            itemBuilder: (BuildContext context, int index) {
              final display = widget.displays[index];
              return Container(
                padding: const EdgeInsets.symmetric(horizontal: 60),
                child: Center(
                  child: MouseRegion(
                    cursor: SystemMouseCursors.click,
                    child: GestureDetector(
                      child: AspectRatio(
                        aspectRatio: display.width / display.height,
                        child: Container(
                          padding: const EdgeInsets.symmetric(vertical: 8),
                          decoration: BoxDecoration(
                            borderRadius: BorderRadius.circular(4),
                            boxShadow: [
                              BoxShadow(
                                color: Colors.black.withOpacity(0.1),
                                blurRadius: 3,
                                spreadRadius: 1.2,
                              ),
                            ],
                          ),
                          child: ClipRRect(
                            borderRadius: BorderRadius.circular(4),
                            child: Image.memory(
                              display.screenShot,
                            ),
                          ),
                        ),
                      ),
                      onTap: () {
                        Navigator.pop(context, display.id);
                      },
                    ),
                  ),
                ),
              );
            },
          ),
        ),
        Text(monitorName),
        Text(
            "${widget.displays[_selectedIndex].width}x${widget.displays[_selectedIndex].height}@${widget.displays[_selectedIndex].refreshRate}"),
      ],
    );
  }
}

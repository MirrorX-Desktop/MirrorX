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
  BoxFit _fit = BoxFit.none;

  @override
  Widget build(BuildContext context) {
    return _buildDesktopSurface();
  }

  Widget _buildDesktopSurface() {
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
              fit: _fit,
            ),
          ),
        )
      ],
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

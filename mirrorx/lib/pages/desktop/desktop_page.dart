import 'dart:developer';

import 'package:card_swiper/card_swiper.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:mirrorx/env/sdk/mirrorx_core.dart';
import 'package:mirrorx/env/sdk/mirrorx_core_sdk.dart';
import 'package:mirrorx/env/utility/dialog.dart';
import 'package:mirrorx/model/desktop.dart';
import 'package:mirrorx/pages/desktop/widgets/desktop_render_box/desktop_render_box.dart';
import 'package:mirrorx/state/desktop_manager/desktop_manager_cubit.dart';
import 'package:mirrorx/state/navigator_key.dart';
import 'package:mirrorx/state/page_manager/page_manager_cubit.dart';
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
    return BlocListener<DesktopManagerCubit, DesktopManagerState>(
      listenWhen: ((previous, current) {
        return !previous.closedDesktops.contains(widget.model.remoteDeviceId) &&
            current.closedDesktops.contains(widget.model.remoteDeviceId);
      }),
      listener: (context, state) {
        context
            .read<DesktopManagerCubit>()
            .removeDesktop(widget.model.remoteDeviceId);

        context.read<PageManagerCubit>().switchPage("Connect");

        popupDialog(
          contentBuilder: (_) => Text(
              AppLocalizations.of(context)!.dialogContentConnectionDisconnected,
              textAlign: TextAlign.center),
          actionBuilder: (navigatorState) => [
            TextButton(
              onPressed: navigatorState.pop,
              child: Text(AppLocalizations.of(context)!.dialogOK),
            ),
          ],
        );
      },
      child: _buildDesktopSurface(),
    );
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
              message: _fit == BoxFit.none
                  ? AppLocalizations.of(context)!
                      .desktopPageToolbarButtonTooltipScale
                  : AppLocalizations.of(context)!
                      .desktopPageToolbarButtonTooltipNoneScale,
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

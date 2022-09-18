import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:mirrorx/env/utility/dialog.dart';
import 'package:mirrorx/env/utility/error_notifier.dart';
import 'package:mirrorx/pages/desktop/widgets/desktop_render_box/desktop_render_box.dart';
import 'package:mirrorx/state/desktop_manager/desktop_manager_cubit.dart';
import 'package:mirrorx/state/page_manager/page_manager_cubit.dart';

class DesktopPage extends StatefulWidget {
  const DesktopPage(
    this.localDeviceId,
    this.remoteDeviceId, {
    Key? key,
  }) : super(key: key);

  final int localDeviceId;
  final int remoteDeviceId;

  @override
  _DesktopPageState createState() => _DesktopPageState();
}

class _DesktopPageState extends State<DesktopPage> {
  BoxFit _fit = BoxFit.none;
  late DialogNotifier _dialogNotifier;

  @override
  void initState() {
    super.initState();
    _dialogNotifier = DialogNotifier(context);
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<DesktopManagerCubit, DesktopManagerState>(
      builder: (context, state) {
        final cubit = context.read<DesktopManagerCubit>();

        if (state.desktopPrepareInfoLists.any((element) =>
            element.localDeviceId == widget.localDeviceId &&
            element.remoteDeviceId == widget.remoteDeviceId)) {
          Future.microtask(() async {
            try {
              await cubit.connect(widget.remoteDeviceId);
            } catch (err) {
              await _dialogNotifier.popupDialog(
                  contentBuilder: (_) => Text("Connect failed $err"),
                  actionBuilder: (context, navState) {
                    return [
                      TextButton(
                          onPressed: navState.pop,
                          child: Text(AppLocalizations.of(context)!.dialogOK)),
                    ];
                  });

              // todo: remote current page
            }
          });

          return const CircularProgressIndicator();
        }

        return Container();
      },
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
        // Expanded(
        //   child: Container(
        //     color: Colors.black,
        //     child: DesktopRenderBox(
        //       model: widget.model,
        //       fit: _fit,
        //     ),
        //   ),
        // )
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

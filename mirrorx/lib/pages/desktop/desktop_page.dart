import 'dart:developer';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:mirrorx/env/utility/error_notifier.dart';
import 'package:mirrorx/pages/desktop/widgets/desktop_render_box/desktop_render_box.dart';
import 'package:mirrorx/pages/desktop/widgets/desktop_toolbar.dart';
import 'package:mirrorx/state/desktop_manager/desktop_manager_cubit.dart';

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
  late DialogNotifier _dialogNotifier;
  late DesktopId _desktopId;

  @override
  void initState() {
    super.initState();
    _dialogNotifier = DialogNotifier(context);
    log("${widget.localDeviceId},${widget.remoteDeviceId}");
    _desktopId = DesktopId(widget.localDeviceId, widget.remoteDeviceId);
  }

  @override
  Widget build(BuildContext context) {
    final cubit = context.read<DesktopManagerCubit>();

    return FutureBuilder(
      future: _prepareConnection(cubit),
      builder: (context, snapshot) {
        if (snapshot.connectionState != ConnectionState.done) {
          return const Center(
            child: SizedBox(
              width: 60,
              height: 60,
              child: CircularProgressIndicator(),
            ),
          );
        }

        if (snapshot.hasError) {
          return Center(
            child: Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                const Text(
                  "Connect failed",
                  style: TextStyle(fontSize: 36.0, fontWeight: FontWeight.bold),
                ),
                Text(
                  "${snapshot.error}",
                  style: const TextStyle(fontSize: 20.0),
                ),
                TextButton(
                    onPressed: () {
                      // todo: remote this page
                    },
                    child: Text(AppLocalizations.of(context)!.dialogOK))
              ],
            ),
          );
        }

        return _buildDesktopSurface();
      },
    );
  }

  Widget _buildDesktopSurface() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        DesktopToolbar(_desktopId),
        Expanded(
          child: Container(
            color: Colors.black,
            child: DesktopRenderBox(_desktopId),
          ),
        )
      ],
    );
  }

  Future _prepareConnection(DesktopManagerCubit cubit) async {
    if (cubit.state.desktopPrepareInfoLists.any((element) =>
        element.localDeviceId == widget.localDeviceId &&
        element.remoteDeviceId == widget.remoteDeviceId)) {
      final prepareInfo = cubit.removePrepareInfo(widget.remoteDeviceId);
      if (prepareInfo != null) {
        await cubit.connectAndNegotiate(prepareInfo);
        return;
      }
    }

    if (cubit.state.desktopInfoLists.containsKey(_desktopId)) {
      return;
    }

    return Future.error(
        "DesktopInfo for remote device '${widget.remoteDeviceId}' not exist");
  }
}

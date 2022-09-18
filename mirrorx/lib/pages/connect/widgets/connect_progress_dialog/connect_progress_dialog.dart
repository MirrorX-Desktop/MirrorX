import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mirrorx/env/sdk/mirrorx_core.dart';
import 'package:mirrorx/env/sdk/mirrorx_core_sdk.dart';
import 'package:mirrorx/state/desktop_manager/desktop_manager_cubit.dart';
import 'package:mirrorx/state/page_manager/page_manager_cubit.dart';
import 'package:texture_render/texture_render.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';

import 'screenshot_swiper.dart';

enum ConnectStep {
  inputPassword,
  keyExchange,
  prepareMedia,
}

class ConnectProgressStateDialog extends StatefulWidget {
  const ConnectProgressStateDialog({
    Key? key,
    required this.remoteDeviceId,
  }) : super(key: key);

  final String remoteDeviceId;

  @override
  _ConnectProgressStateDialogState createState() =>
      _ConnectProgressStateDialogState();
}

class _ConnectProgressStateDialogState
    extends State<ConnectProgressStateDialog> {
  late TextEditingController _textController;
  late ConnectStep _connectStep;
  // DisplayInfo? _selectedDisplayInfo;

  @override
  void initState() {
    super.initState();
    _textController = TextEditingController();
    _connectStep = ConnectStep.inputPassword;
  }

  @override
  void dispose() {
    _textController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    // switch (_connectStep) {
    //   case ConnectStep.inputPassword:
    //     return _buildInputPasswordPanel();
    //   case ConnectStep.keyExchange:
    //   // return _buildKeyExchangePanel();
    //   case ConnectStep.prepareMedia:
    //     // return _buildPrepareMediaPanel();
    // }
    return Container();
  }

  Widget _buildInputPasswordPanel() {
    return _buildAlertDialog(
      Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Text(AppLocalizations.of(context)!.dialogContentInputDevicePassword),
          TextField(
            textAlign: TextAlign.center,
            textAlignVertical: TextAlignVertical.center,
            controller: _textController,
            maxLines: 1,
            maxLength: 24,
            keyboardType: TextInputType.text,
          ),
        ],
      ),
      actions: [
        TextButton(
          child: Text(AppLocalizations.of(context)!.dialogOK),
          onPressed: () {
            setState(() {
              _connectStep = ConnectStep.keyExchange;
            });
          },
        ),
        TextButton(
          child: Text(AppLocalizations.of(context)!.dialogCancel),
          onPressed: () {
            Navigator.of(context).pop(null);
          },
        ),
      ],
    );
  }

  // Widget _buildKeyExchangePanel() {
  //   return FutureBuilder(
  //     future: MirrorXCoreSDK.instance.signalingConnectionKeyExchange(
  //       remoteDeviceId: widget.remoteDeviceId,
  //       password: _textController.text,
  //     ),
  //     builder: (context, snapshot) {
  //       if (snapshot.connectionState != ConnectionState.done) {
  //         return _buildProgressAndTip(AppLocalizations.of(context)!
  //             .connectPageConnectProgressTipKeyExchange);
  //       }

  //       if (snapshot.hasError) {
  //         return _buildAlertDialog(
  //           Column(
  //             mainAxisAlignment: MainAxisAlignment.center,
  //             children: [
  //               Text(AppLocalizations.of(context)!
  //                   .connectPageConnectProgressTipKeyExchangeFailed),
  //               Text(snapshot.error.toString())
  //             ],
  //           ),
  //           actions: [
  //             TextButton(
  //               onPressed: () {
  //                 Navigator.of(context).pop();
  //               },
  //               child: Text(AppLocalizations.of(context)!.dialogOK),
  //             )
  //           ],
  //         );
  //       }

  //       return _buildSelectMonitorsPanel();
  //     },
  //   );
  // }

  // Widget _buildSelectMonitorsPanel() {
  //   return FutureBuilder(
  //     future: MirrorXCoreSDK.instance
  //         .endpointGetDisplayInfo(remoteDeviceId: widget.remoteDeviceId),
  //     builder: (context, snapshot) {
  //       if (snapshot.connectionState != ConnectionState.done) {
  //         return _buildProgressAndTip(AppLocalizations.of(context)!
  //             .connectPageConnectProgressTipListMonitors);
  //       }

  //       if (snapshot.hasError) {
  //         return _buildAlertDialog(
  //           Column(
  //             mainAxisAlignment: MainAxisAlignment.center,
  //             children: [
  //               Text(AppLocalizations.of(context)!
  //                   .connectPageConnectProgressTipListMonitorsFailed),
  //               Text(snapshot.error.toString())
  //             ],
  //           ),
  //           actions: [
  //             TextButton(
  //               onPressed: () {
  //                 Navigator.of(context).pop();
  //               },
  //               child: Text(AppLocalizations.of(context)!.dialogOK),
  //             )
  //           ],
  //         );
  //       }

  //       final resp = snapshot.data as GetDisplayInfoResponse;

  //       return _buildMonitorsPanel(resp);
  //     },
  //   );
  // }

  // Widget _buildMonitorsPanel(GetDisplayInfoResponse resp) {
  //   return _buildAlertDialog(
  //     Column(
  //       mainAxisSize: MainAxisSize.min,
  //       children: [
  //         Text(AppLocalizations.of(context)!.dialogContentSelectMonitor),
  //         ScreenShotSwiper(
  //           displays: resp.displays,
  //           selectCallback: (displayInfo) {
  //             setState(() {
  //               _selectedDisplayInfo = displayInfo;
  //               _connectStep = ConnectStep.prepareMedia;
  //             });
  //           },
  //         ),
  //       ],
  //     ),
  //     actions: [
  //       TextButton(
  //         onPressed: () {
  //           Navigator.of(context).pop();
  //         },
  //         child: Text(AppLocalizations.of(context)!.dialogCancel),
  //       )
  //     ],
  //   );
  // }

  // Widget _buildPrepareMediaPanel() {
  //   return FutureBuilder(
  //     future: prepareMediaTransmission(context.read<DesktopManagerCubit>()),
  //     builder: (context, snapshot) {
  //       if (snapshot.connectionState != ConnectionState.done) {
  //         return _buildProgressAndTip(AppLocalizations.of(context)!
  //             .connectPageConnectProgressTipPrepareMedia);
  //       }

  //       if (snapshot.hasError) {
  //         return _buildAlertDialog(
  //           Column(
  //             mainAxisAlignment: MainAxisAlignment.center,
  //             children: [
  //               Text(AppLocalizations.of(context)!
  //                   .connectPageConnectProgressTipPrepareMediaFailed),
  //               Text(snapshot.error.toString())
  //             ],
  //           ),
  //           actions: [
  //             TextButton(
  //               onPressed: () {
  //                 Navigator.of(context).pop();
  //               },
  //               child: Text(AppLocalizations.of(context)!.dialogOK),
  //             )
  //           ],
  //         );
  //       }

  //       context
  //           .read<DesktopManagerCubit>()
  //           .addDesktop(snapshot.data as DesktopModel);

  //       context.read<PageManagerCubit>().switchPage(widget.remoteDeviceId);

  //       Navigator.of(context).pop();

  //       return const SizedBox.shrink();
  //     },
  //   );
  // }

  // Future<DesktopModel> prepareMediaTransmission(
  //     DesktopManagerCubit cubit) async {
  //   final registerTextureResponse =
  //       await TextureRender.instance.registerTexture();

  //   final startMediaTransmissionResponse =
  //       await MirrorXCoreSDK.instance.endpointStartMediaTransmission(
  //     remoteDeviceId: widget.remoteDeviceId,
  //     expectFps: _selectedDisplayInfo!.refreshRate,
  //     expectDisplayId: _selectedDisplayInfo!.id,
  //     textureId: registerTextureResponse.textureID,
  //     videoTexturePtr: registerTextureResponse.videoTexturePointer,
  //     updateFrameCallbackPtr:
  //         registerTextureResponse.updateFrameCallbackPointer,
  //   );

  //   final closeNotifyStream = MirrorXCoreSDK.instance
  //       .endpointCloseNotify(remoteDeviceId: widget.remoteDeviceId);

  //   final subscription = closeNotifyStream.listen(
  //     (event) {
  //       cubit.markDeskopClosed(widget.remoteDeviceId);
  //     },
  //     onDone: () {
  //       cubit.markDeskopClosed(widget.remoteDeviceId);
  //     },
  //   );

  //   return DesktopModel(
  //     remoteDeviceId: widget.remoteDeviceId,
  //     osType: startMediaTransmissionResponse.osType,
  //     monitorWidth: startMediaTransmissionResponse.screenWidth,
  //     monitorHeight: startMediaTransmissionResponse.screenHeight,
  //     textureID: registerTextureResponse.textureID,
  //     videoTexturePointer: registerTextureResponse.videoTexturePointer,
  //     updateFrameCallbackPointer:
  //         registerTextureResponse.updateFrameCallbackPointer,
  //     subscription: subscription,
  //   );
  // }

  Widget _buildAlertDialog(Widget content, {List<Widget>? actions}) {
    return AlertDialog(
      title: const Text("MirrorX", textAlign: TextAlign.center),
      content: content,
      actions: actions,
    );
  }

  Widget _buildProgressAndTip(String tip) {
    return _buildAlertDialog(SizedBox(
      width: 80,
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          const CircularProgressIndicator(),
          Padding(
            padding: const EdgeInsets.symmetric(vertical: 8),
            child: Text(
              tip,
              textAlign: TextAlign.center,
            ),
          )
        ],
      ),
    ));
  }
}

import 'package:flutter/material.dart';
import 'package:flutter/src/foundation/key.dart';
import 'package:flutter/src/widgets/framework.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mirrorx/env/langs/tr.dart';
import 'package:mirrorx/env/sdk/mirrorx_core.dart';
import 'package:mirrorx/env/sdk/mirrorx_core_sdk.dart';
import 'package:mirrorx/model/desktop.dart';
import 'package:mirrorx/state/desktop_manager/desktop_manager_cubit.dart';
import 'package:mirrorx/state/page_manager/page_manager_cubit.dart';
import 'package:texture_render/texture_render.dart';

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
  bool _visiable = true;
  DisplayInfo? _selectedDisplayInfo;

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
    switch (_connectStep) {
      case ConnectStep.inputPassword:
        return _buildInputPasswordPanel();
      case ConnectStep.keyExchange:
        return _buildKeyExchangePanel();
      case ConnectStep.prepareMedia:
        return _buildPrepareMediaPanel();
    }
  }

  Widget _buildInputPasswordPanel() {
    return _buildAlertDialog(
      Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Text(tr.dialogContentInputDevicePassword),
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
          child: Text(tr.dialogOK),
          onPressed: () {
            setState(() {
              _connectStep = ConnectStep.keyExchange;
            });
          },
        ),
        TextButton(
          child: Text(tr.dialogCancel),
          onPressed: () {
            Navigator.of(context).pop(null);
          },
        ),
      ],
    );
  }

  Widget _buildKeyExchangePanel() {
    return FutureBuilder(
      future: MirrorXCoreSDK.instance.signalingConnectionKeyExchange(
        remoteDeviceId: widget.remoteDeviceId,
        password: _textController.text,
      ),
      builder: (context, snapshot) {
        if (snapshot.connectionState != ConnectionState.done) {
          return _buildAlertDialog(const CircularProgressIndicator());
        }

        if (snapshot.hasError) {
          return _buildAlertDialog(
            Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Text("auth password has an error"),
                Text(snapshot.error.toString())
              ],
            ),
            actions: [
              TextButton(
                onPressed: () {
                  Navigator.of(context).pop();
                },
                child: Text(tr.dialogOK),
              )
            ],
          );
        }

        return _buildSelectMonitorsPanel();
      },
    );
  }

  Widget _buildSelectMonitorsPanel() {
    return FutureBuilder(
      future: MirrorXCoreSDK.instance
          .endpointGetDisplayInfo(remoteDeviceId: widget.remoteDeviceId),
      builder: (context, snapshot) {
        if (snapshot.connectionState != ConnectionState.done) {
          return _buildAlertDialog(const CircularProgressIndicator());
        }

        if (snapshot.hasError) {
          return _buildAlertDialog(
            Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Text("auth password has an error"),
                Text(snapshot.error.toString())
              ],
            ),
            actions: [
              TextButton(
                onPressed: () {
                  Navigator.of(context).pop();
                },
                child: Text(tr.dialogOK),
              )
            ],
          );
        }

        final resp = snapshot.data as GetDisplayInfoResponse;

        return _buildMonitorsPanel(resp);
      },
    );
  }

  Widget _buildMonitorsPanel(GetDisplayInfoResponse resp) {
    return _buildAlertDialog(
      Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Text("选择显示器"),
          ScreenShotSwiper(
            displays: resp.displays,
            selectCallback: (displayInfo) {
              setState(() {
                _selectedDisplayInfo = displayInfo;
                _connectStep = ConnectStep.prepareMedia;
              });
            },
          ),
        ],
      ),
      actions: [
        TextButton(
          onPressed: () {
            Navigator.of(context).pop();
          },
          child: Text(tr.dialogOK),
        )
      ],
    );
  }

  Widget _buildPrepareMediaPanel() {
    return FutureBuilder(
      future: prepareMediaTransmission(),
      builder: (context, snapshot) {
        if (snapshot.connectionState != ConnectionState.done) {
          return _buildAlertDialog(const CircularProgressIndicator());
        }

        if (snapshot.hasError) {
          return _buildAlertDialog(
            Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Text("prepare media has an error"),
                Text(snapshot.error.toString())
              ],
            ),
            actions: [
              TextButton(
                onPressed: () {
                  Navigator.of(context).pop();
                },
                child: Text(tr.dialogOK),
              )
            ],
          );
        }

        context
            .read<DesktopManagerCubit>()
            .addDesktop(snapshot.data as DesktopModel);

        context.read<PageManagerCubit>().switchPage(widget.remoteDeviceId);

        Navigator.of(context).pop();

        return const SizedBox.shrink();
      },
    );
  }

  Future<DesktopModel> prepareMediaTransmission() async {
    final registerTextureResponse =
        await TextureRender.instance.registerTexture();

    final startMediaTransmissionResponse =
        await MirrorXCoreSDK.instance.endpointStartMediaTransmission(
      remoteDeviceId: widget.remoteDeviceId,
      expectFps: _selectedDisplayInfo!.refreshRate,
      expectDisplayId: _selectedDisplayInfo!.id,
      textureId: registerTextureResponse.textureID,
      videoTexturePtr: registerTextureResponse.videoTexturePointer,
      updateFrameCallbackPtr:
          registerTextureResponse.updateFrameCallbackPointer,
    );

    return DesktopModel(
      remoteDeviceId: widget.remoteDeviceId,
      monitorWidth: startMediaTransmissionResponse.screenWidth,
      monitorHeight: startMediaTransmissionResponse.screenHeight,
      textureID: registerTextureResponse.textureID,
      videoTexturePointer: registerTextureResponse.videoTexturePointer,
      updateFrameCallbackPointer:
          registerTextureResponse.updateFrameCallbackPointer,
    );
  }

  Widget _buildAlertDialog(Widget content, {List<Widget>? actions}) {
    return AlertDialog(
      title: const Text("MirrorX", textAlign: TextAlign.center),
      content: content,
      actions: actions,
    );
  }
}

import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:mirrorx/app/core/values/colors.dart';
import 'package:mirrorx/app/modules/connect_remote/controller.dart';
import 'package:mirrorx/app/modules/connect_remote/widgets/connect_to/widget.dart';
import 'package:mirrorx/app/modules/connect_remote/widgets/device_id/widget.dart';
import 'package:mirrorx/app/modules/connect_remote/widgets/device_password/widget.dart';

class ConnectRemotePage extends GetView<ConnectRemoteController> {
  const ConnectRemotePage({Key? key, required String staticTag})
      : _staticTag = staticTag,
        super(key: key);

  final String _staticTag;

  @override
  String? get tag => _staticTag;

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        Expanded(
          flex: 1,
          child: Column(
            // crossAxisAlignment: CrossAxisAlignment.end,
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Padding(
                padding: const EdgeInsets.symmetric(vertical: 16.0),
                child: SizedBox(
                  width: 320,
                  child: _createBorderedField(
                      "connect_to_remote.connect_remote_title".tr,
                      const ConnectTo()),
                ),
              ),
              Padding(
                padding: const EdgeInsets.symmetric(vertical: 16.0),
                child: SizedBox(
                  width: 320,
                  child: _createBorderedField(
                      "connect_to_remote.device_id_title".tr, const DeviceID()),
                ),
              ),
              Padding(
                padding: const EdgeInsets.symmetric(vertical: 16.0),
                child: SizedBox(
                  width: 320,
                  child: _createBorderedField(
                      "connect_to_remote.device_password_title".tr,
                      const DevicePassword()),
                ),
              )
            ],
          ),
        ),
        const VerticalDivider(
          indent: 90,
          endIndent: 90,
          width: 30,
        ),
        Expanded(
          flex: 1,
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Text(
                "There's no recently connected devices",
                style: TextStyle(color: Colors.grey.withOpacity(0.5)),
              )
            ],
          ),
        ),
      ],
    );
  }

  Widget _createBorderedField(String title, Widget child) {
    return Stack(
      clipBehavior: Clip.none,
      children: [
        DecoratedBox(
          decoration: BoxDecoration(
              border: Border.all(), borderRadius: BorderRadius.circular(8)),
          child: Padding(
            padding: const EdgeInsets.fromLTRB(12, 20, 12, 16),
            child: child,
          ),
        ),
        Positioned(
          left: 8,
          top: -12,
          child: DecoratedBox(
            decoration: const BoxDecoration(color: ColorValues.primaryColor),
            child: Text(
              " $title ",
              style: const TextStyle(
                fontWeight: FontWeight.w500,
                fontSize: 20,
              ),
            ),
          ),
        ),
      ],
    );
  }
}

import 'package:mirrorx/pages/connect/widgets/remote_connect_field/remote_connect_field.dart';
import 'package:flutter/material.dart';

import 'widgets/device_id_field/device_id_field.dart';
import 'widgets/device_password_field/device_password_field.dart';

class ConnectPage extends StatelessWidget {
  const ConnectPage({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        DecoratedBox(
          decoration: const BoxDecoration(
              border:
                  Border(right: BorderSide(color: Colors.black, width: 1.0))),
          child: Padding(
            padding: const EdgeInsets.fromLTRB(16, 26, 16, 0),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: const [
                Padding(
                  padding: EdgeInsets.only(bottom: 10),
                  child: DeviceIdField(),
                ),
                Padding(
                  padding: EdgeInsets.only(top: 10),
                  child: DevicePasswordField(),
                ),
              ],
            ),
          ),
        ),
        Expanded(
          child: Padding(
            padding: const EdgeInsets.fromLTRB(16, 26, 16, 0),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: const [RemoteConnectField()],
            ),
          ),
        ),
      ],
    );
  }
}

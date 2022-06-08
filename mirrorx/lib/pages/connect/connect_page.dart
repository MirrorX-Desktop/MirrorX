import 'package:mirrorx/business/mirrorx_core/mirrorx_core_bloc.dart';
import 'package:mirrorx/business/page_manager/page.dart';
import 'package:mirrorx/env/langs/tr.dart';
import 'package:mirrorx/pages/connect/widgets/device_id_field/device_id_field.dart';
import 'package:mirrorx/pages/connect/widgets/password_field/password_field.dart';
import 'package:mirrorx/pages/connect/widgets/remote_connect_field/remote_connect_field.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

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
                DeviceIdField(),
                Divider(
                  color: Colors.transparent,
                  height: 12,
                ),
                PasswordField(),
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

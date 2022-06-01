import 'package:app/business/page_manager/page.dart';
import 'package:app/pages/connect/widgets/connect_id_field/connect_id_field.dart';
import 'package:app/pages/connect/widgets/password_field/password_field.dart';
import 'package:app/pages/connect/widgets/remote_connect_field/remote_connect_field.dart';
import 'package:flutter/material.dart';

class ConnectPage extends NavigationPage {
  const ConnectPage({Key? key})
      : super(key: key, title: 'Connect', titleIcon: Icons.screen_share);

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
                ConnectIdField(),
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
          child: Container(
            padding: const EdgeInsets.fromLTRB(16, 26, 0, 0),
            child: const RemoteConnectField(),
          ),
        )
      ],
    );
  }

  @override
  int getIndex() => 0;
}

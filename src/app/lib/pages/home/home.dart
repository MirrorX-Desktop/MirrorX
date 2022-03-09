import 'package:easy_localization/easy_localization.dart';
import 'package:flutter/material.dart';
import 'package:mirrorx/constants.dart';
import 'package:mirrorx/pages/home/components/connectRemoteField.dart';
import 'package:mirrorx/pages/home/components/localAccessIdField.dart';
import 'package:mirrorx/pages/home/components/localAccessPasswordField.dart';
import 'package:mirrorx/pages/page.dart';

class HomePage extends AppPage {
  const HomePage({Key? key, required String tag}) : super(key: key, tag: tag);

  @override
  _HomeState createState() => _HomeState();
}

class _HomeState extends State<HomePage> {
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
                      tr("connect_to_remote.connect_remote_title"),
                      const ConnectRemoteField()),
                ),
              ),
              Padding(
                padding: const EdgeInsets.symmetric(vertical: 16.0),
                child: SizedBox(
                  width: 320,
                  child: _createBorderedField(
                      tr("connect_to_remote.local_access_id_title"),
                      const LocalAccessIdField()),
                ),
              ),
              Padding(
                padding: const EdgeInsets.symmetric(vertical: 16.0),
                child: SizedBox(
                  width: 320,
                  child: _createBorderedField(
                      tr("connect_to_remote.local_access_password_title"),
                      const LocalAccessPasswordField()),
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
            decoration: const BoxDecoration(color: primaryColor),
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

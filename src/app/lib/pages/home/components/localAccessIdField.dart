import 'dart:developer' as dev;

import 'package:easy_localization/easy_localization.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:mirrorx_sdk/mirrorx_sdk.dart';

class LocalAccessIdField extends StatefulWidget {
  const LocalAccessIdField({Key? key}) : super(key: key);

  @override
  _LocalAccessIdFieldState createState() => _LocalAccessIdFieldState();
}

class _LocalAccessIdFieldState extends State<LocalAccessIdField> {
  late final Future<String> _loadLocalAccessIdFuture =
      MirrorXSDK.requestDeviceToken();

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 40,
      child: Row(
        children: [
          Expanded(
            child: FutureBuilder(
                future: _loadLocalAccessIdFuture,
                builder: (context, snapshot) {
                  if (snapshot.connectionState != ConnectionState.done) {
                    return const CupertinoActivityIndicator();
                  }

                  if (snapshot.hasError) {
                    return const Text("error",
                        style: TextStyle(color: Colors.red));
                  }

                  if (!snapshot.hasData) {
                    return const Text("no data",
                        style: TextStyle(color: Colors.red));
                  }

                  final accessId = snapshot.data! as String;
                  dev.log(accessId);
                  final splitted = accessId.split('.');
                  if (splitted.length != 4) {
                    return const Text("splitted error",
                        style: TextStyle(color: Colors.red));
                  }

                  return NumericPanel(numericStr: splitted[0].padLeft(8, '0'));
                }),
          ),
          IconButton(
              onPressed: () {
                ScaffoldMessenger.of(context).showSnackBar(
                  SnackBar(
                    content: Text(
                        tr("connect_to_remote.local_access_id_copy_tooltip")),
                  ),
                );
              },
              tooltip: tr("connect_to_remote.local_access_id_copy"),
              splashRadius: 14,
              splashColor: Colors.transparent,
              hoverColor: const Color.fromARGB(240, 220, 220, 220),
              icon: const Icon(
                Icons.content_copy,
                size: 16,
              )),
        ],
      ),
    );
  }
}

class NumericPanel extends StatelessWidget {
  final String numericStr;
  const NumericPanel({Key? key, required this.numericStr}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisAlignment: MainAxisAlignment.center,
      children: numericStr.characters
          .map((e) => _singleNumericPanel(context, e))
          .toList(),
    );
  }

  Widget _singleNumericPanel(BuildContext context, String char) {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 2.0),
      child: SizedBox(
        width: 24,
        height: 34,
        child: DecoratedBox(
          decoration: BoxDecoration(
              color: const Color.fromARGB(255, 235, 235, 235),
              borderRadius: const BorderRadius.all(Radius.circular(3)),
              boxShadow: [
                BoxShadow(
                  color: Colors.grey.withOpacity(0.5),
                  spreadRadius: 0,
                  blurRadius: 0,
                  offset: const Offset(0, 3), // changes position of shadow
                ),
              ]),
          child: Center(
              child: Text(
            char,
            style: const TextStyle(fontSize: 18),
          )),
        ),
      ),
    );
  }
}

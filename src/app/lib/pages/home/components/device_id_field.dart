import 'dart:async';
import 'dart:developer' as dev;

import 'package:easy_localization/easy_localization.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:mirrorx_sdk/mirrorx_sdk.dart';

class DeviceIDField extends StatefulWidget {
  const DeviceIDField({Key? key}) : super(key: key);

  @override
  _DeviceIDFieldState createState() => _DeviceIDFieldState();
}

class _DeviceIDFieldState extends State<DeviceIDField> {
  late Future<String> _getDeviceIDFuture = _getDeviceID();

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 40,
      child: Row(
        children: [
          Expanded(
            child: FutureBuilder(
                future: _getDeviceIDFuture,
                builder: (context, snapshot) {
                  if (snapshot.connectionState != ConnectionState.done) {
                    return const CupertinoActivityIndicator();
                  }

                  if (snapshot.hasError) {
                    return IconButton(
                        onPressed: _retryGetDeviceID,
                        tooltip: tr("device_id_field.load_failed_tooltip"),
                        splashRadius: 14,
                        splashColor: Colors.transparent,
                        hoverColor: const Color.fromARGB(240, 220, 220, 220),
                        icon: const Icon(
                          Icons.warning,
                          size: 16,
                          color: Colors.red,
                        ));
                  }

                  final deviceID = snapshot.data! as String;
                  dev.log("get device id: $deviceID");

                  return NumericPanel(numericStr: deviceID.padLeft(8, '0'));
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

  Future<String> _getDeviceID() async {
    try {
      final sdk = await MirrorXSDK.getInstance();
      final deviceID = await sdk.config.readConfig("device_id");
      if (deviceID != null) {
        return Future.value(deviceID);
      }

      final deviceToken = await sdk.requestDeviceToken();
      final splitted = deviceToken.split('.');
      if (splitted.length != 4) {
        return Future.error(Exception("invalid device token"));
      }

      final newDeviceID = splitted[0];
      await sdk.config.storeConfig("device_id", newDeviceID);

      return Future.value(newDeviceID);
    } catch (err) {
      return Future.error(err);
    }
  }

  void _retryGetDeviceID() {
    setState(() {
      _getDeviceIDFuture = _getDeviceID();
    });
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

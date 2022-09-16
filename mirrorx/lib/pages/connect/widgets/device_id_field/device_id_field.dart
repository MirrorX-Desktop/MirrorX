import 'dart:developer';

import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:font_awesome_flutter/font_awesome_flutter.dart';
import 'package:mirrorx/state/signaling_manager/signaling_manager_cubit.dart';

class DeviceIdField extends StatelessWidget {
  const DeviceIdField({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      height: 110,
      width: 360,
      decoration: const BoxDecoration(
        border: Border(left: BorderSide(color: Colors.yellow, width: 4)),
      ),
      child: Padding(
        padding: const EdgeInsets.only(left: 12.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Text(
                  AppLocalizations.of(context)!.connectPageDeviceIDTitle,
                  style: const TextStyle(fontSize: 27),
                ),
                SizedBox(
                  width: 50,
                  child:
                      BlocBuilder<SignalingManagerCubit, SignalingManagerState>(
                    builder: (context, state) => IconButton(
                      onPressed: () {
                        Clipboard.setData(ClipboardData(
                          text: state.domainConfig?.deviceId.toString() ?? "",
                        )).then(
                          (_) => ScaffoldMessenger.of(context)
                              .showSnackBar(SnackBar(
                            content: Text(AppLocalizations.of(context)!
                                .connectPageDeviceIDButtonCopySnackbarContent),
                            behavior: SnackBarBehavior.floating,
                          )),
                        );
                      },
                      icon: const FaIcon(
                        FontAwesomeIcons.copy,
                        size: 24,
                      ),
                      tooltip: AppLocalizations.of(context)!
                          .connectPageDeviceIDButtonCopyTooltip,
                    ),
                  ),
                ),
              ],
            ),
            Expanded(
              child: BlocBuilder<SignalingManagerCubit, SignalingManagerState>(
                builder: (context, state) {
                  final deviceId = state.domainConfig?.deviceId.toString();
                  if (deviceId != null) {
                    return Text(
                      "${deviceId.substring(0, 2)}-${deviceId.substring(2, 6)}-${deviceId.substring(6, 10)}",
                      style: const TextStyle(fontSize: 45),
                    );
                  }

                  return const FaIcon(FontAwesomeIcons.xmark, size: 24);
                },
              ),
            )
          ],
        ),
      ),
    );
  }
}

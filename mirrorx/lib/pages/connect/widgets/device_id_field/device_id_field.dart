import 'dart:developer';

import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mirrorx/state/profile/profile_state_cubit.dart';

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
                BlocBuilder<ProfileStateCubit, ProfileState>(
                  builder: (context, state) => IconButton(
                    onPressed: () {
                      Clipboard.setData(ClipboardData(text: state.deviceID))
                          .then(
                        (_) =>
                            ScaffoldMessenger.of(context).showSnackBar(SnackBar(
                          content: Text(AppLocalizations.of(context)!
                              .connectPageDeviceIDButtonCopySnackbarContent),
                          behavior: SnackBarBehavior.floating,
                        )),
                      );
                    },
                    icon: const Icon(Icons.copy),
                    splashRadius: 20,
                    hoverColor: Colors.yellow,
                    tooltip: AppLocalizations.of(context)!
                        .connectPageDeviceIDButtonCopyTooltip,
                  ),
                ),
              ],
            ),
            Expanded(
              child: FutureBuilder(
                future: context.read<ProfileStateCubit>().getDeviceID(),
                builder: (context, snapshot) {
                  switch (snapshot.connectionState) {
                    case ConnectionState.none:
                    case ConnectionState.waiting:
                    case ConnectionState.active:
                      return const Center(child: CircularProgressIndicator());
                    case ConnectionState.done:
                      if (snapshot.hasError) {
                        log("Error: ${snapshot.error}");
                        return const Center(
                            child: Icon(Icons.report, color: Colors.red));
                      } else {
                        final id = snapshot.data.toString().padLeft(10, '0');

                        return Text(
                          "${id.substring(0, 2)}-${id.substring(2, 6)}-${id.substring(6, 10)}",
                          style: const TextStyle(fontSize: 45),
                        );
                      }
                  }
                },
              ),
            )
          ],
        ),
      ),
    );
  }
}

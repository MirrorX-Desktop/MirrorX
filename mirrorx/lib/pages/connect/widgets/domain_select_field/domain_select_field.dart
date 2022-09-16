import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:font_awesome_flutter/font_awesome_flutter.dart';
import 'package:mirrorx/state/signaling_manager/signaling_manager_cubit.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';

class DomainSelectField extends StatelessWidget {
  const DomainSelectField({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<SignalingManagerCubit, SignalingManagerState>(
      builder: (context, state) {
        Color borderColor = Colors.blue;
        if (state.connectionState == SignalingConnectionState.connected) {
          borderColor = Colors.green;
        } else if (state.connectionState ==
            SignalingConnectionState.disconnected) {
          borderColor = Colors.red;
        }

        return Container(
          width: 360,
          height: 110,
          decoration: BoxDecoration(
            border: Border(left: BorderSide(color: borderColor, width: 4)),
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
                      AppLocalizations.of(context)!.connectPageDomainTitle,
                      style: const TextStyle(fontSize: 27),
                    ),
                    SizedBox(
                      width: 50,
                      child: IconButton(
                        onPressed: () {},
                        icon: const FaIcon(
                          FontAwesomeIcons.objectGroup,
                          size: 24,
                        ),
                      ),
                    ),
                  ],
                ),
                Expanded(
                  child: Text(
                    state.domain ?? "",
                    style: const TextStyle(fontSize: 45),
                  ),
                ),
              ],
            ),
          ),
        );
      },
    );
  }
}

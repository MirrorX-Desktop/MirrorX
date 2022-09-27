import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:mirrorx/state/desktop_manager/desktop_manager_cubit.dart';

class DesktopToolbar extends StatelessWidget {
  const DesktopToolbar(this.desktopId, {Key? key}) : super(key: key);

  final DesktopId desktopId;

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<DesktopManagerCubit, DesktopManagerState>(
      builder: (context, state) {
        final desktopInfo = state.desktopInfoLists[desktopId];
        return Row(
          children: [
            Text(desktopId.remoteDeviceId.toString()),
            const VerticalDivider(),
            Tooltip(
              message: desktopInfo?.boxFit == BoxFit.none
                  ? AppLocalizations.of(context)!
                      .desktopPageToolbarButtonTooltipScale
                  : AppLocalizations.of(context)!
                      .desktopPageToolbarButtonTooltipNoneScale,
              child: Container(
                width: 36,
                height: 36,
                padding: const EdgeInsets.all(3.0),
                child: TextButton(
                  onPressed: () {
                    context.read<DesktopManagerCubit>().switchBoxFit(desktopId);
                  },
                  style: ButtonStyle(
                    shape: MaterialStateProperty.all(
                      RoundedRectangleBorder(
                          borderRadius: BorderRadius.circular(4.0)),
                    ),
                    padding: MaterialStateProperty.all(EdgeInsets.zero),
                    foregroundColor: MaterialStateProperty.all(Colors.black),
                  ),
                  child: desktopInfo?.boxFit == BoxFit.none
                      ? const Icon(Icons.aspect_ratio)
                      : const Icon(Icons.fit_screen),
                ),
              ),
            ),
          ],
        );
      },
    );
  }
}

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:mirrorx/state/desktop_manager/desktop_manager_cubit.dart';

class DesktopToolbar extends StatelessWidget {
  const DesktopToolbar(this.desktopId, {Key? key}) : super(key: key);

  final DesktopId desktopId;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 40,
      child: BlocBuilder<DesktopManagerCubit, DesktopManagerState>(
        builder: (context, state) {
          final desktopInfo = state.desktopInfoLists[desktopId];
          return Row(
            children: [
              Text(desktopId.remoteDeviceId.toString()),
              const VerticalDivider(
                thickness: 1.5,
                indent: 8,
                endIndent: 8,
              ),
              Padding(
                padding:
                    const EdgeInsets.symmetric(vertical: 6.0, horizontal: 8.0),
                child: Tooltip(
                  message: desktopInfo?.boxFit == BoxFit.none
                      ? AppLocalizations.of(context)!
                          .desktopPageToolbarButtonTooltipScale
                      : AppLocalizations.of(context)!
                          .desktopPageToolbarButtonTooltipNoneScale,
                  child: TextButton(
                    onPressed: () {
                      context
                          .read<DesktopManagerCubit>()
                          .updateBoxFit(desktopId);
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
                        ? const Icon(Icons.fit_screen)
                        : const Icon(Icons.aspect_ratio),
                  ),
                ),
              ),
              Padding(
                padding:
                    const EdgeInsets.symmetric(vertical: 6.0, horizontal: 8.0),
                child: Tooltip(
                  message: "Scale Quality",
                  child: DropdownButtonHideUnderline(
                    child: DropdownButton<FilterQuality>(
                      elevation: 1,
                      // alignment: AlignmentDirectional.center,
                      // dropdownColor: Colors.white,
                      borderRadius: const BorderRadius.all(Radius.circular(5)),
                      value: desktopInfo?.filterQuality,
                      items: const [
                        DropdownMenuItem(
                          value: FilterQuality.none,
                          child: Padding(
                            padding: EdgeInsets.symmetric(horizontal: 6),
                            child: Text("较差"),
                          ),
                        ),
                        DropdownMenuItem(
                          value: FilterQuality.low,
                          child: Padding(
                            padding: EdgeInsets.symmetric(horizontal: 6),
                            child: Text("一般"),
                          ),
                        ),
                        DropdownMenuItem(
                          value: FilterQuality.medium,
                          child: Padding(
                            padding: EdgeInsets.symmetric(horizontal: 6),
                            child: Text("较好"),
                          ),
                        ),
                        DropdownMenuItem(
                          value: FilterQuality.high,
                          child: Padding(
                            padding: EdgeInsets.symmetric(horizontal: 6),
                            child: Text("极质"),
                          ),
                        )
                      ],
                      onChanged: (FilterQuality? value) {
                        if (value != null) {
                          context
                              .read<DesktopManagerCubit>()
                              .updateFilterQuality(desktopId, value);
                        }
                      },
                    ),
                  ),
                ),
              )
            ],
          );
        },
      ),
    );
  }
}

import 'package:font_awesome_flutter/font_awesome_flutter.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mirrorx/state/desktop_manager/desktop_manager_cubit.dart';
import 'package:mirrorx/state/page_manager/page_manager_cubit.dart';

import 'navigation_menu_item.dart';

class NavigationMenu extends StatelessWidget {
  const NavigationMenu({
    Key? key,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<PageManagerCubit, PageManagerState>(
      builder: (context, state) => Column(
        children: [
          Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              NavigationMenuItem(
                pageTag: "Connect",
                iconBuilder: (color) => Icon(Icons.screen_share, color: color),
                title: AppLocalizations.of(context)!.connectPageTitle,
                system: true,
              ),
              NavigationMenuItem(
                pageTag: "Intranet",
                iconBuilder: (color) => Icon(Icons.lan, color: color),
                title: AppLocalizations.of(context)!.intranetPageTitle,
                system: true,
              ),
              NavigationMenuItem(
                pageTag: "Files",
                iconBuilder: (color) => Icon(Icons.folder_copy, color: color),
                title: AppLocalizations.of(context)!.filesPageTitle,
                system: true,
              ),
              NavigationMenuItem(
                pageTag: "History",
                iconBuilder: (color) => Icon(Icons.history, color: color),
                title: AppLocalizations.of(context)!.historyPageTitle,
                system: true,
              ),
              NavigationMenuItem(
                pageTag: "Settings",
                iconBuilder: (color) => Icon(Icons.tune, color: color),
                title: AppLocalizations.of(context)!.settingsPageTitle,
                system: true,
              ),
            ],
          ),
          Visibility(
            visible: state.desktopIds.isNotEmpty,
            child: Container(
              width: 36,
              margin: const EdgeInsets.symmetric(vertical: 6),
              decoration: BoxDecoration(
                border: Border.all(color: Colors.black, width: 0.5),
                borderRadius: BorderRadius.circular(4),
              ),
            ),
          ),
          Expanded(
            child: SizedBox(
              width: 72,
              child: ListView(
                primary: true,
                physics: const BouncingScrollPhysics(),
                children: state.desktopIds.map((desktopId) {
                  final splitIds = desktopId.split("@");
                  return Padding(
                      padding: const EdgeInsets.symmetric(vertical: 2.0),
                      child: NavigationMenuItem(
                        remoteDeviceId: int.parse(splitIds[1]),
                        pageTag: desktopId,
                        iconBuilder: (color) =>
                            FaIcon(FontAwesomeIcons.windows),
                        // FaIcon(_getOSIcon(model.osType), color: color),
                        title: splitIds[1],
                        system: false,
                        // state.closedDesktops.contains(model.remoteDeviceId),
                      ));
                }).toList(),
              ),
            ),
          )
        ],
      ),
    );
  }

  // IconData _getOSIcon(OperatingSystemType osType) {
  //   if (osType is OperatingSystemType_Windows) {
  //     return FontAwesomeIcons.windows;
  //   } else if (osType is OperatingSystemType_macOS) {
  //     return FontAwesomeIcons.apple;
  //   } else if (osType is OperatingSystemType_iOS) {
  //     return FontAwesomeIcons.apple;
  //   } else if (osType is OperatingSystemType_Android) {
  //     return FontAwesomeIcons.android;
  //   } else if (osType is OperatingSystemType_Linux) {
  //     switch (osType.field0) {
  //       case LinuxType.CentOS:
  //         return FontAwesomeIcons.centos;
  //       case LinuxType.Fedora:
  //         return FontAwesomeIcons.fedora;
  //       case LinuxType.Redhat:
  //         return FontAwesomeIcons.redhat;
  //       case LinuxType.openSUSE:
  //         return FontAwesomeIcons.suse;
  //       case LinuxType.Ubuntu:
  //         return FontAwesomeIcons.ubuntu;
  //       case LinuxType.Other:
  //       default:
  //         return FontAwesomeIcons.linux;
  //     }
  //   } else {
  //     return FontAwesomeIcons.display;
  //   }
  // }
}

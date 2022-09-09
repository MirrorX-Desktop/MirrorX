import 'package:font_awesome_flutter/font_awesome_flutter.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mirrorx/env/sdk/mirrorx_core.dart';
import 'package:mirrorx/env/sdk/mirrorx_core.dart';
import 'package:mirrorx/state/desktop_manager/desktop_manager_cubit.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';

import 'navigation_menu_item.dart';

class NavigationMenu extends StatelessWidget {
  const NavigationMenu({
    Key? key,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<DesktopManagerCubit, DesktopManagerState>(
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
            visible: state.desktopModels.isNotEmpty,
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
                children: state.desktopModels
                    .map(
                      (model) => Padding(
                        padding: const EdgeInsets.symmetric(vertical: 2.0),
                        child: NavigationMenuItem(
                          pageTag: model.remoteDeviceId,
                          iconBuilder: (color) =>
                              FaIcon(_getOSIcon(model.osType), color: color),
                          title: model.remoteDeviceId,
                          system: false,
                          desktopClosed: state.closedDesktops
                              .contains(model.remoteDeviceId),
                          desktopModel: model,
                        ),
                      ),
                    )
                    .toList(),
                // children: [
                //   Padding(
                //     padding: const EdgeInsets.symmetric(vertical: 2.0),
                //     child: NavigationMenuItem(
                //       pageTag: "KDKDD",
                //       iconBuilder: (color) =>
                //           FaIcon(FontAwesomeIcons.redhat, color: color),
                //       title: "DDSDDSDFS",
                //       system: false,
                //       desktopClosed: false,
                //     ),
                //   ),
                // ],
              ),
            ),
          )
        ],
      ),
    );
  }

  IconData _getOSIcon(OperatingSystemType osType) {
    if (osType is OperatingSystemType_Windows) {
      return FontAwesomeIcons.windows;
    } else if (osType is OperatingSystemType_macOS) {
      return FontAwesomeIcons.apple;
    } else if (osType is OperatingSystemType_iOS) {
      return FontAwesomeIcons.apple;
    } else if (osType is OperatingSystemType_Android) {
      return FontAwesomeIcons.android;
    } else if (osType is OperatingSystemType_Linux) {
      switch (osType.field0) {
        case LinuxType.CentOS:
          return FontAwesomeIcons.centos;
        case LinuxType.Fedora:
          return FontAwesomeIcons.fedora;
        case LinuxType.Redhat:
          return FontAwesomeIcons.redhat;
        case LinuxType.openSUSE:
          return FontAwesomeIcons.suse;
        case LinuxType.Ubuntu:
          return FontAwesomeIcons.ubuntu;
        case LinuxType.Other:
        default:
          return FontAwesomeIcons.linux;
      }
    } else {
      return FontAwesomeIcons.display;
    }
  }
}

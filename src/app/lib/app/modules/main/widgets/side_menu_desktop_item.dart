import 'package:flutter/material.dart';
import 'package:font_awesome_flutter/font_awesome_flutter.dart';
import 'package:get/get.dart';
import 'package:just_the_tooltip/just_the_tooltip.dart';
import 'package:mirrorx/app/controllers/page_view.dart';

import 'controllers/side_menu_item.dart';

class SideMenuDesktopItem extends StatelessWidget {
  const SideMenuDesktopItem({
    Key? key,
    required this.pageTag,
    required this.title,
    required this.osName,
    required this.osVersion,
  }) : super(key: key);

  final String pageTag;
  final String title;
  final String osName;
  final String osVersion;

  // @override
  // Widget build(BuildContext context) {
  //   Get.lazyPut(() => SideMenuItemController(pageTag), tag: pageTag);

  //   return GetBuilder<SideMenuItemController>(
  //     tag: pageTag,
  //     builder: (controller) {
  //       return JustTheTooltip(
  //           controller: controller.tooltipController,
  //           preferredDirection: AxisDirection.right,
  //           content: Padding(
  //               padding: const EdgeInsets.all(8),
  //               child: Text(
  //                 title,
  //                 style: const TextStyle(
  //                     fontSize: 16, fontWeight: FontWeight.bold),
  //               )),
  //           tailBaseWidth: 6,
  //           tailLength: 4,
  //           offset: 12,
  //           margin: EdgeInsets.zero,
  //           borderRadius: const BorderRadius.all(Radius.circular(8)),
  //           child: Padding(
  //               padding: const EdgeInsets.symmetric(vertical: 4),
  //               child: AnimatedBuilder(
  //                   animation: controller.animationController,
  //                   builder: (context, child) {
  //                     return DecoratedBox(
  //                       decoration: BoxDecoration(
  //                           color: controller.backgroundColorAnimation.value,
  //                           borderRadius:
  //                               const BorderRadius.all(Radius.circular(10)),
  //                           boxShadow: [
  //                             BoxShadow(
  //                                 offset: const Offset(0, 1.5),
  //                                 color: Colors.grey.withAlpha(100),
  //                                 blurRadius: 3.5)
  //                           ]),
  //                       child: MouseRegion(
  //                         cursor: SystemMouseCursors.click,
  //                         onEnter: (_) => controller.hoverEnter(),
  //                         onExit: (_) => controller.hoverLeave(),
  //                         child: GestureDetector(
  //                           onTap: () => controller.pageViewController
  //                               .jumpToPage(pageTag),
  //                           behavior: HitTestBehavior.opaque,
  //                           child: _buildInnerButton(controller),
  //                         ),
  //                       ),
  //                     );
  //                   })));
  //     },
  //   );
  // }

  // Widget _buildInnerButton(SideMenuItemController sideMenuItemController) {
  //   return Stack(
  //     children: [
  //       Table(
  //         defaultVerticalAlignment: TableCellVerticalAlignment.middle,
  //         children: [
  //           TableRow(
  //             children: [
  //               Padding(
  //                 padding: const EdgeInsets.all(8.0),
  //                 child: Text(
  //                   "ID: $pageTag",
  //                   style: TextStyle(
  //                       color: sideMenuItemController.titleColorAnimation.value,
  //                       fontSize: 14,
  //                       fontWeight: FontWeight.w500),
  //                 ),
  //               ),
  //             ],
  //           ),
  //           TableRow(
  //             children: [
  //               Padding(
  //                 padding: const EdgeInsets.fromLTRB(8, 0, 8, 8),
  //                 child: Text(
  //                   "hostName",
  //                   style: TextStyle(
  //                       color: sideMenuItemController.titleColorAnimation.value,
  //                       fontSize: 14,
  //                       fontWeight: FontWeight.w500),
  //                 ),
  //               ),
  //             ],
  //           ),
  //         ],
  //       ),
  //       Positioned(
  //         right: 0,
  //         bottom: 0,
  //         child: ClipRRect(
  //           borderRadius:
  //               const BorderRadius.only(bottomRight: Radius.circular(10)),
  //           child: Align(
  //             alignment: Alignment.topLeft,
  //             widthFactor: 0.8,
  //             heightFactor: 0.8,
  //             child: FaIcon(
  //               _buildSystemIcon(),
  //               color: sideMenuItemController.titleColorAnimation.value,
  //               size: 45,
  //             ),
  //           ),
  //         ),
  //       ),
  //     ],
  //   );
  // }

  @override
  Widget build(BuildContext context) {
    Get.lazyPut(() => SideMenuItemController(pageTag), tag: pageTag);

    return GetBuilder<SideMenuItemController>(
        tag: pageTag,
        builder: (controller) {
          return JustTheTooltip(
              controller: controller.tooltipController,
              preferredDirection: AxisDirection.right,
              content: Padding(
                padding: const EdgeInsets.all(8),
                child: Column(
                  mainAxisSize: MainAxisSize.min,
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Padding(
                      padding: const EdgeInsets.symmetric(vertical: 2.0),
                      child: Text("Device ID: KLSIBO57"),
                    ),
                    Padding(
                      padding: const EdgeInsets.symmetric(vertical: 2.0),
                      child: Text("Remote OS: Windows 10 Pro"),
                    ),
                    Padding(
                      padding: const EdgeInsets.symmetric(vertical: 2.0),
                      child: Text("OS Version: Build 15236"),
                    )
                  ],
                ),
              ),
              tailBaseWidth: 6,
              tailLength: 4,
              offset: 8,
              margin: EdgeInsets.zero,
              elevation: 2,
              borderRadius: const BorderRadius.all(Radius.circular(6)),
              child: Padding(
                  padding: const EdgeInsets.symmetric(vertical: 4),
                  child: AnimatedBuilder(
                    animation: controller.animationController,
                    builder: (context, child) {
                      return DecoratedBox(
                          decoration: BoxDecoration(
                            color: controller.backgroundColorAnimation.value,
                            borderRadius:
                                const BorderRadius.all(Radius.circular(7)),
                          ),
                          child: MouseRegion(
                              cursor: SystemMouseCursors.click,
                              onEnter: (_) => controller.hoverEnter(),
                              onExit: (_) => controller.hoverLeave(),
                              child: GestureDetector(
                                  onTap: () => controller.pageViewController
                                      .jumpToPage(pageTag),
                                  behavior: HitTestBehavior.opaque,
                                  child: Center(
                                    child: SizedBox(
                                      width: 42,
                                      height: 42,
                                      child: Icon(_buildSystemIcon(),
                                          size: 26,
                                          color: controller
                                              .titleColorAnimation.value),
                                    ),
                                  ))));
                    },
                  )));
        });
  }

  IconData _buildSystemIcon() {
    switch (osName) {
      case "windows":
        return FontAwesomeIcons.windows;
      case "macos":
        return FontAwesomeIcons.apple;
      case "android":
        return FontAwesomeIcons.android;
      case "ios":
        return FontAwesomeIcons.mobileAlt;
      case "linux":
        final version = osVersion.toLowerCase();

        if (version.contains("ubuntu")) {
          return FontAwesomeIcons.ubuntu;
        } else if (version.contains("centos")) {
          return FontAwesomeIcons.centos;
        } else if (version.contains("fedora")) {
          return FontAwesomeIcons.fedora;
        } else if (version.contains("redhat")) {
          return FontAwesomeIcons.redhat;
        } else if (version.contains("suse")) {
          return FontAwesomeIcons.suse;
        } else {
          return FontAwesomeIcons.linux;
        }
      default:
        return FontAwesomeIcons.desktop;
    }
  }
}

import 'package:flutter/material.dart';
import 'package:font_awesome_flutter/font_awesome_flutter.dart';
import 'package:get/get.dart';
import 'package:mirrorx/app/controllers/page_view.dart';

import 'controllers/side_menu_item.dart';

class SideMenuDesktopItem extends StatelessWidget {
  const SideMenuDesktopItem(
      {Key? key,
      required this.icon,
      required this.title,
      required this.pageTag})
      : super(key: key);

  final IconData icon;
  final String title;
  final String pageTag;

  @override
  Widget build(BuildContext context) {
    Get.lazyPut(() => SideMenuItemController(pageTag), tag: pageTag);

    return Padding(
        padding: const EdgeInsets.symmetric(vertical: 4),
        child: GetBuilder<SideMenuItemController>(
            tag: pageTag,
            builder: (controller) {
              return AnimatedBuilder(
                  animation: controller.animationController,
                  builder: (context, child) {
                    return ClipRRect(
                      borderRadius: const BorderRadius.all(Radius.circular(10)),
                      child: DecoratedBox(
                        decoration: BoxDecoration(
                          color: controller.backgroundColorAnimation.value,
                          border: Border.all(
                              width: 1,
                              color: (controller
                                              .pageViewController.selectedTag ==
                                          pageTag
                                      ? controller
                                          .backgroundColorAnimation.value
                                      : controller.titleColorAnimation.value) ??
                                  Colors.transparent),
                          borderRadius: const BorderRadius.all(
                            Radius.circular(10),
                          ),
                        ),
                        child: MouseRegion(
                          cursor: SystemMouseCursors.click,
                          onEnter: (_) => controller.hoverEnter(),
                          onExit: (_) => controller.hoverLeave(),
                          child: GestureDetector(
                            onTap: () => controller.pageViewController
                                .jumpToPage(pageTag),
                            behavior: HitTestBehavior.opaque,
                            child: _buildInnerButton(controller),
                          ),
                        ),
                      ),
                    );
                  });
            }));
  }

  Widget _buildInnerButton(SideMenuItemController sideMenuItemController) {
    return Stack(
      children: [
        Table(
          defaultVerticalAlignment: TableCellVerticalAlignment.middle,
          children: [
            TableRow(
              children: [
                Padding(
                  padding: const EdgeInsets.all(8.0),
                  child: Text(
                    "ID: $pageTag",
                    style: TextStyle(
                        color: sideMenuItemController.titleColorAnimation.value,
                        fontSize: 14,
                        fontWeight: FontWeight.w500),
                  ),
                ),
              ],
            ),
            TableRow(
              children: [
                Padding(
                  padding: const EdgeInsets.fromLTRB(8, 0, 8, 8),
                  child: Text(
                    "hostName",
                    style: TextStyle(
                        color: sideMenuItemController.titleColorAnimation.value,
                        fontSize: 14,
                        fontWeight: FontWeight.w500),
                  ),
                ),
              ],
            ),
          ],
        ),
        Positioned(
          right: -6,
          bottom: -12,
          child: FaIcon(
            FontAwesomeIcons.windows,
            color: sideMenuItemController.titleColorAnimation.value,
            size: 45,
          ),
        ),
      ],
    );
  }
}

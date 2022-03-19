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
                    return DecoratedBox(
                      decoration: BoxDecoration(
                          color: controller.backgroundColorAnimation.value,
                          borderRadius:
                              const BorderRadius.all(Radius.circular(10)),
                          boxShadow: [
                            BoxShadow(
                                offset: const Offset(0, 1.5),
                                color: Colors.grey.withAlpha(100),
                                blurRadius: 3.5)
                          ]),
                      child: MouseRegion(
                        cursor: SystemMouseCursors.click,
                        onEnter: (_) => controller.hoverEnter(),
                        onExit: (_) => controller.hoverLeave(),
                        child: GestureDetector(
                          onTap: () =>
                              controller.pageViewController.jumpToPage(pageTag),
                          behavior: HitTestBehavior.opaque,
                          child: _buildInnerButton(controller),
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
          right: 0,
          bottom: 0,
          child: ClipRRect(
            borderRadius:
                const BorderRadius.only(bottomRight: Radius.circular(10)),
            child: Align(
              alignment: Alignment.topLeft,
              widthFactor: 0.8,
              heightFactor: 0.8,
              child: FaIcon(
                FontAwesomeIcons.windows,
                color: sideMenuItemController.titleColorAnimation.value,
                size: 45,
              ),
            ),
          ),
        ),
      ],
    );
  }
}

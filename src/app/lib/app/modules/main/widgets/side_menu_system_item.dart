import 'package:flutter/material.dart';
import 'package:get/get.dart';

import 'controllers/side_menu_item.dart';

class SideMenuSystemItem extends StatelessWidget {
  const SideMenuSystemItem(
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
                  borderRadius: const BorderRadius.all(Radius.circular(10)),
                  boxShadow: [
                    BoxShadow(
                        offset: const Offset(0, 1.5),
                        color: Colors.grey.withOpacity(0.3),
                        blurRadius: 3)
                  ],
                ),
                child: MouseRegion(
                  cursor: SystemMouseCursors.click,
                  onEnter: (_) => controller.hoverEnter(),
                  onExit: (_) => controller.hoverLeave(),
                  child: GestureDetector(
                    onTap: () =>
                        controller.pageViewController.jumpToPage(pageTag),
                    behavior: HitTestBehavior.opaque,
                    child: Padding(
                      padding: const EdgeInsets.all(8.0),
                      child: Row(
                        children: [
                          Padding(
                            padding: const EdgeInsets.only(right: 8.0),
                            child: Icon(icon,
                                size: 24,
                                color: controller.titleColorAnimation.value),
                          ),
                          Text(
                            title,
                            style: TextStyle(
                                color: controller.titleColorAnimation.value,
                                fontSize: 14,
                                fontWeight: FontWeight.w500),
                          ),
                        ],
                      ),
                    ),
                  ),
                ),
              );
            },
          );
        },
      ),
    );
  }
}

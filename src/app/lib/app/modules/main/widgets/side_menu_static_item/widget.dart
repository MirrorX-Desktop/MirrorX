import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:mirrorx/app/controllers/page_view.dart';

import 'controller.dart';

class SideMenuStaticItem extends StatelessWidget {
  const SideMenuStaticItem(
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
    Get.put(SideMenuStaticItemController(pageTag), tag: pageTag);

    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 4),
      child: GetBuilder<SideMenuStaticItemController>(
        tag: pageTag,
        builder: (controller) {
          return AnimatedBuilder(
            animation: controller.animationController,
            builder: (context, child) {
              return DecoratedBox(
                decoration: BoxDecoration(
                  color: controller.backgroundColorAnimation.value,
                  borderRadius: const BorderRadius.all(Radius.circular(10)),
                ),
                child: MouseRegion(
                  cursor: SystemMouseCursors.click,
                  onEnter: (_) => controller.hoverEnter(),
                  onExit: (_) => controller.hoverLeave(),
                  child: GestureDetector(
                    onTap: () =>
                        Get.find<PageViewController>().jumpToPage(pageTag),
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

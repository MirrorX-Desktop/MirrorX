import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:just_the_tooltip/just_the_tooltip.dart';

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

    return GetBuilder<SideMenuItemController>(
        tag: pageTag,
        builder: (controller) {
          return JustTheTooltip(
              controller: controller.tooltipController,
              preferredDirection: AxisDirection.right,
              content: Padding(
                  padding: const EdgeInsets.all(8),
                  child: Text(
                    title,
                    style: const TextStyle(
                        fontSize: 16, fontWeight: FontWeight.bold),
                  )),
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
                                  child: SizedBox(
                                    width: 42,
                                    height: 42,
                                    child: Center(
                                      child: Icon(icon,
                                          size: 26,
                                          color: controller
                                              .titleColorAnimation.value),
                                    ),
                                  ))));
                    },
                  )));
        });
  }
}

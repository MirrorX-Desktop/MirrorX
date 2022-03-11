import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:mirrorx/app/controllers/page_view.dart';

import 'controller.dart';

class SideMenuStaticItem extends GetView<SideMenuStaticItemController> {
  const SideMenuStaticItem(
      {Key? key,
      required this.icon,
      required this.title,
      required this.jumpPageTag})
      : super(key: key);

  final IconData icon;
  final String title;
  final String jumpPageTag;

  @override
  Widget build(BuildContext context) {
    final SideMenuStaticItemController _sideMenuStaticItemController =
        Get.put(SideMenuStaticItemController(jumpPageTag), tag: jumpPageTag);

    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 4),
      child: GetBuilder<SideMenuStaticItemController>(
        init: _sideMenuStaticItemController,
        tag: jumpPageTag,
        builder: (staticItemController) {
          return AnimatedBuilder(
            animation: staticItemController.animationController,
            builder: (context, child) {
              return DecoratedBox(
                decoration: BoxDecoration(
                  color: staticItemController.backgroundColorAnimation.value,
                  borderRadius: const BorderRadius.all(Radius.circular(10)),
                ),
                child: MouseRegion(
                  cursor: SystemMouseCursors.click,
                  onEnter: (_) => staticItemController.hoverEnter(),
                  onExit: (_) => staticItemController.hoverLeave(),
                  child: GestureDetector(
                    onTap: () =>
                        Get.find<PageViewController>().jumpToPage(jumpPageTag),
                    behavior: HitTestBehavior.opaque,
                    child: Padding(
                      padding: const EdgeInsets.all(8.0),
                      child: Row(
                        children: [
                          Padding(
                            padding: const EdgeInsets.only(right: 8.0),
                            child: Icon(icon,
                                size: 24,
                                color: staticItemController
                                    .titleColorAnimation.value),
                          ),
                          Text(
                            title,
                            style: TextStyle(
                                color: staticItemController
                                    .titleColorAnimation.value,
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

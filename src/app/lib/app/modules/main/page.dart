import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:mirrorx/app/controllers/page_view.dart';
import 'package:mirrorx/app/modules/main/widgets/side_menu.dart';

class MainPage extends StatelessWidget {
  const MainPage({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Material(
      color: Colors.white,
      child: Row(
        children: [
          const SizedBox(
            width: 160,
            height: double.infinity,
            child: SideMenu(),
          ),
          const VerticalDivider(
            width: 1,
          ),
          Expanded(
            child: GetBuilder<PageViewController>(
              builder: (controller) => PageView.builder(
                controller: controller.pageController,
                physics: const NeverScrollableScrollPhysics(),
                itemCount: controller.totalPageViewCount,
                itemBuilder: (context, index) => controller.getPage(index),
              ),
            ),
          ),
        ],
      ),
    );
  }
}

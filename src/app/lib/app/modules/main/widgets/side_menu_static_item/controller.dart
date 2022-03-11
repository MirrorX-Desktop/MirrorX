import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:mirrorx/app/controllers/page_view.dart';
import 'package:mirrorx/app/core/values/colors.dart';

class SideMenuStaticItemController extends GetxController
    with GetSingleTickerProviderStateMixin {
  SideMenuStaticItemController(String tg) : myTag = tg;

  final String myTag;

  late AnimationController animationController;
  late Animation<Color?> titleColorAnimation;
  late Animation<Color?> backgroundColorAnimation;

  Color? _currentTextColor;
  Color? _currentBackgroundColor;

  late bool _selected;

  late PageViewController _pageViewController;

  @override
  void onInit() {
    animationController = AnimationController(
        duration: const Duration(milliseconds: 160), vsync: this);

    titleColorAnimation = ColorTween().animate(animationController);
    backgroundColorAnimation = ColorTween().animate(animationController);

    _selected = false;

    _pageViewController = Get.find<PageViewController>();
    _pageViewController.addListener(() {
      print("tag: $myTag, ${myTag == _pageViewController.selectedTag}");
      myTag == _pageViewController.selectedTag ? selected() : unselected();
    });

    super.onInit();
  }

  void hoverEnter() {
    if (!_selected) {
      _updateTextColorAnimation(ColorValues.primaryColor, Colors.white);
    }
  }

  void hoverLeave() {
    if (!_selected) {
      _updateTextColorAnimation(Colors.black, Colors.white);
    }
  }

  void selected() {
    _selected = true;
    _updateTextColorAnimation(Colors.white, ColorValues.primaryColor);
  }

  void unselected() {
    _selected = false;
    _updateTextColorAnimation(Colors.black, Colors.white);
  }

  void _updateTextColorAnimation(
      Color titleForwardColor, Color backgroundForwardColor) {
    animationController.reset();

    titleColorAnimation =
        ColorTween(begin: _currentTextColor, end: titleForwardColor)
            .animate(animationController);
    _currentTextColor = titleForwardColor;

    backgroundColorAnimation =
        ColorTween(begin: _currentBackgroundColor, end: backgroundForwardColor)
            .animate(animationController);
    _currentBackgroundColor = backgroundForwardColor;

    animationController.forward();
  }

  @override
  void dispose() {
    animationController.dispose();
    super.dispose();
  }
}

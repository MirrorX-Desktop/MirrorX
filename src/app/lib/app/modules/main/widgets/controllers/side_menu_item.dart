import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:just_the_tooltip/just_the_tooltip.dart';
import 'package:mirrorx/app/controllers/page_view.dart';
import 'package:mirrorx/app/core/values/colors.dart';

class SideMenuItemController extends GetxController
    with GetSingleTickerProviderStateMixin {
  SideMenuItemController(String tag) : _tag = tag;

  final String _tag;

  late AnimationController animationController;
  late Animation<Color?> titleColorAnimation;
  late Animation<Color?> backgroundColorAnimation;

  late Color _currentTextColor;
  late Color _currentBackgroundColor;

  late bool _selected;

  late PageViewController _pageViewController;

  late JustTheController _tooltipController;

  PageViewController get pageViewController => _pageViewController;
  JustTheController get tooltipController => _tooltipController;

  @override
  void onInit() {
    animationController = AnimationController(
        duration: const Duration(milliseconds: 160), vsync: this);

    titleColorAnimation = ColorTween().animate(animationController);
    backgroundColorAnimation = ColorTween().animate(animationController);

    _currentTextColor = Colors.black;
    _currentBackgroundColor = Colors.white;

    _pageViewController = Get.find<PageViewController>();
    _tooltipController = JustTheController();

    _selected = false;

    super.onInit();
  }

  @override
  void onReady() {
    subscribePageViewControllerUpdate();
    _pageViewController.addListener(subscribePageViewControllerUpdate);
    super.onReady();
  }

  @override
  void onClose() {
    _pageViewController.removeListener(subscribePageViewControllerUpdate);
    animationController.dispose();
    super.onClose();
  }

  void hoverEnter() {
    _tooltipController.showTooltip();
    if (!_selected) {
      _updateTextColorAnimation(ColorValues.primaryColor, Colors.white);
    }
  }

  void subscribePageViewControllerUpdate() {
    _tag == _pageViewController.selectedTag ? selected() : unselected();
  }

  void hoverLeave() {
    _tooltipController.hideTooltip();
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
}

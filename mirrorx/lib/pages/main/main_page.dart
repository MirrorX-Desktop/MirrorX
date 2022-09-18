import 'dart:developer';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mirrorx/pages/connect/connect_page.dart';
import 'package:mirrorx/pages/desktop/desktop_page.dart';
import 'package:mirrorx/pages/files/files_page.dart';
import 'package:mirrorx/pages/history/history_page.dart';
import 'package:mirrorx/pages/intranet/intranet_page.dart';
import 'package:mirrorx/pages/settings/settings_page.dart';
import 'package:mirrorx/state/desktop_manager/desktop_manager_cubit.dart';
import 'package:mirrorx/state/page_manager/page_manager_cubit.dart';

import 'widgets/navigation_menu/navigation_menu.dart';

class MainPage extends StatelessWidget {
  const MainPage({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return const _LayoutView();
  }
}

class _LayoutView extends StatelessWidget {
  const _LayoutView({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    WidgetsBinding.instance.addPostFrameCallback((timeStamp) {
      context.read<PageManagerCubit>().switchPage("Connect");
    });

    return Row(
      children: [
        Container(
          decoration: const BoxDecoration(
            border: Border(right: BorderSide(color: Colors.black, width: 1.0)),
          ),
          padding: EdgeInsets.fromLTRB(0, Platform.isMacOS ? 26 : 6, 0, 0),
          child: const NavigationMenu(),
        ),
        const Expanded(child: _LayoutPageBuilder()),
      ],
    );
  }
}

class _LayoutPageBuilder extends StatefulWidget {
  const _LayoutPageBuilder({Key? key}) : super(key: key);

  @override
  _LayoutPageBuilderState createState() => _LayoutPageBuilderState();
}

class _LayoutPageBuilderState extends State<_LayoutPageBuilder>
    with SingleTickerProviderStateMixin {
  late AnimationController _animationController;

  @override
  void initState() {
    super.initState();

    _animationController =
        AnimationController(duration: kThemeAnimationDuration, vsync: this);
  }

  @override
  Widget build(BuildContext context) {
    return BlocConsumer<PageManagerCubit, PageManagerState>(
      builder: (context, state) => FadeTransition(
        opacity: _animationController.view,
        child: _buildPage(state.currentPageTag),
      ),
      listener: (context, state) {
        _animationController.reset();
        _animationController.forward();
      },
    );
  }

  Widget? _buildPage(String pageTag) {
    switch (pageTag) {
      case "Connect":
        return const ConnectPage();
      case "Intranet":
        return const IntranetPage();
      case "Files":
        return const FilesPage();
      case "History":
        return const HistoryPage();
      case "Settings":
        return const SettingsPage();
      default:
        for (final id in context.read<PageManagerCubit>().state.desktopIds) {
          if (id == pageTag) {
            final splitIds = id.split("@");
            return DesktopPage(int.parse(splitIds[0]), int.parse(splitIds[1]));
          }
        }
        log("Unknown page tag: $pageTag");
        return const ConnectPage();
    }
  }

  @override
  void dispose() {
    _animationController.dispose();
    super.dispose();
  }
}

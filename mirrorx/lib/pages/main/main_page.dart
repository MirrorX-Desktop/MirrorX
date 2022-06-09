import 'dart:developer';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mirrorx/global_state/global_state_cubit.dart';
import 'package:mirrorx/pages/main/cubit/main_page_manager_cubit.dart';

import 'widgets/navigation_menu/navigation_menu.dart';

class MainPage extends StatelessWidget {
  const MainPage({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MultiBlocProvider(
      providers: [
        BlocProvider(create: (context) => GlobalStateCubit()),
        BlocProvider(create: (context) => MainPageManagerCubit()),
      ],
      child: const _LayoutView(),
    );
  }
}

class _LayoutView extends StatelessWidget {
  const _LayoutView({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    WidgetsBinding.instance.addPostFrameCallback((timeStamp) {
      context.read<MainPageManagerCubit>().switchPage("Connect");
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
    return BlocConsumer<MainPageManagerCubit, MainPageManagerState>(
      builder: (context, state) => FadeTransition(
        opacity: _animationController.view,
        child: state.currentPage,
      ),
      listener: (context, state) {
        _animationController.reset();
        _animationController.forward();
      },
    );
  }

  @override
  void dispose() {
    _animationController.dispose();
    super.dispose();
  }
}

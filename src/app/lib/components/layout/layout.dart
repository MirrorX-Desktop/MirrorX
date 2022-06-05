import 'dart:io';

import 'package:app/business/page_manager/page_manager_bloc.dart';
import 'package:app/components/navigation_menu/navigation_menu.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

class Layout extends StatelessWidget {
  const Layout({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return BlocProvider(
      create: (context) => PageManagerBloc()..add(PageManagerInit()),
      child: const _LayoutView(),
    );
  }
}

class _LayoutView extends StatelessWidget {
  const _LayoutView({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
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
    return BlocBuilder<PageManagerBloc, PageManagerState>(
      buildWhen: ((previous, current) {
        if (previous.currentPage != current.currentPage) {
          _animationController.reset();
          _animationController.forward();
          return true;
        }

        return false;
      }),
      builder: (context, state) {
        return FadeTransition(
          opacity: _animationController.view,
          child: BlocProvider.of<PageManagerBloc>(context).state.currentPage,
        );
      },
    );
  }

  @override
  void dispose() {
    _animationController.dispose();
    super.dispose();
  }
}

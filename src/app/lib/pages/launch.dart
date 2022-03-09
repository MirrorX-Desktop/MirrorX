import 'package:flutter/material.dart';
import 'package:mirrorx/components/navigator/navigator.dart';
import 'package:mirrorx/components/sideMenu/sideMenu.dart';
import 'package:provider/provider.dart';

class LaunchPage extends StatefulWidget {
  const LaunchPage({Key? key}) : super(key: key);

  @override
  State<StatefulWidget> createState() => _LaunchPageState();
}

class _LaunchPageState extends State<LaunchPage> {
  late AppNavigator _appNavigator;
  int _totalPageCount = 0;

  @override
  void initState() {
    _appNavigator = Provider.of<AppNavigator>(context, listen: false);
    _totalPageCount = _appNavigator.totalPageCount();

    _appNavigator.addListener(() {
      setState(() {
        _totalPageCount = _appNavigator.totalPageCount();
      });
    });
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // if (!Responsive.isMobile(context))
          const SideMenu(),
          Expanded(
            child: PageView.builder(
              controller: _appNavigator.pageController,
              physics: const NeverScrollableScrollPhysics(),
              itemCount: _totalPageCount,
              itemBuilder: (context, index) {
                return _appNavigator.buildPage(index);
              },
            ),
          ),
        ],
      ),
    );
  }

  @override
  void dispose() {
    super.dispose();
  }
}

import 'package:card_swiper/card_swiper.dart';
import 'package:flutter/material.dart';
import 'package:mirrorx/env/sdk/mirrorx_core.dart';

class ScreenShotSwiper extends StatefulWidget {
  const ScreenShotSwiper({
    Key? key,
    required this.displays,
    required this.selectCallback,
  }) : super(key: key);

  final List<DisplayInfo> displays;
  final Function(DisplayInfo displayInfo) selectCallback;

  @override
  _ScreenShotSwiperState createState() => _ScreenShotSwiperState();
}

class _ScreenShotSwiperState extends State<ScreenShotSwiper> {
  int _selectedIndex = 0;

  @override
  Widget build(BuildContext context) {
    var monitorName = widget.displays[_selectedIndex].name;
    if (monitorName.isEmpty) {
      monitorName = "内建显示器";
    }
    return Column(
      children: [
        SizedBox(
          width: 500,
          height: 280,
          child: Swiper(
            itemCount: widget.displays.length,
            pagination: const SwiperPagination(
              builder: DotSwiperPaginationBuilder(
                activeColor: Colors.yellow,
                color: Colors.white,
              ),
            ),
            control: const SwiperControl(
                iconPrevious: Icons.chevron_left_rounded,
                iconNext: Icons.chevron_right_rounded,
                color: Colors.yellow,
                size: 60,
                padding: EdgeInsets.zero),
            indicatorLayout: PageIndicatorLayout.SCALE,
            onIndexChanged: (index) {
              setState(() {
                _selectedIndex = index;
              });
            },
            itemBuilder: (BuildContext context, int index) {
              final display = widget.displays[index];
              return Container(
                padding: const EdgeInsets.symmetric(horizontal: 60),
                child: Center(
                  child: MouseRegion(
                    cursor: SystemMouseCursors.click,
                    child: GestureDetector(
                      child: AspectRatio(
                        aspectRatio: display.width / display.height,
                        child: Container(
                          padding: const EdgeInsets.symmetric(vertical: 8),
                          decoration: BoxDecoration(
                            borderRadius: BorderRadius.circular(4),
                            boxShadow: [
                              BoxShadow(
                                color: Colors.black.withOpacity(0.1),
                                blurRadius: 1,
                                spreadRadius: 1.2,
                              ),
                            ],
                          ),
                          child: ClipRRect(
                            borderRadius: BorderRadius.circular(4),
                            child: Image.memory(
                              display.screenShot,
                            ),
                          ),
                        ),
                      ),
                      onTap: () {
                        widget.selectCallback(display);
                      },
                    ),
                  ),
                ),
              );
            },
          ),
        ),
        Text(monitorName),
        Text(
            "${widget.displays[_selectedIndex].width}x${widget.displays[_selectedIndex].height}@${widget.displays[_selectedIndex].refreshRate}"),
      ],
    );
  }
}

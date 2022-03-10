import 'package:flutter/material.dart';
import 'package:mirrorx/pages/page.dart';

class RemoteDesktopPage extends AppPage {
  const RemoteDesktopPage({Key? key, required String tag})
      : super(key: key, tag: tag);

  @override
  _RemoteDesktopPageState createState() => _RemoteDesktopPageState();
}

class _RemoteDesktopPageState extends State<RemoteDesktopPage> {
  @override
  Widget build(BuildContext context) {
    return Container(
      child: Text("${widget.tag}"),
    );
  }
}

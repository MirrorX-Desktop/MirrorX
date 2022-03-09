import 'package:flutter/widgets.dart';
import 'package:mirrorx/pages/page.dart';

class LanPage extends AppPage {
  const LanPage({Key? key, required String tag}) : super(key: key, tag: tag);

  @override
  _LanPageState createState() => _LanPageState();
}

class _LanPageState extends State<LanPage> {
  @override
  Widget build(BuildContext context) {
    return Container(
      child: Text("Lan Discovery"),
    );
  }
}

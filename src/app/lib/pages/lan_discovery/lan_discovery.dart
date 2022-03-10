import 'package:flutter/widgets.dart';
import 'package:mirrorx/pages/page.dart';

class LanDiscoveryPage extends AppPage {
  const LanDiscoveryPage({Key? key, required String tag})
      : super(key: key, tag: tag);

  @override
  _LanPageState createState() => _LanPageState();
}

class _LanPageState extends State<LanDiscoveryPage> {
  @override
  Widget build(BuildContext context) {
    return Container(
      child: Text("Lan Discovery"),
    );
  }
}

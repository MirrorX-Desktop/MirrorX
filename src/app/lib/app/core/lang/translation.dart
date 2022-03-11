import 'package:get/route_manager.dart';
import 'package:mirrorx/app/core/lang/en.dart';
import 'package:mirrorx/app/core/lang/zh_hans.dart';

class Translation extends Translations {
  @override
  Map<String, Map<String, String>> get keys => {
        "zh_Hans": zhHans,
        "en": en,
      };
}

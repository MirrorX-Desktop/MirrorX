import 'package:get/route_manager.dart';
import 'package:app/src/core/lang/en.dart';
import 'package:app/src/core/lang/zh_hans.dart';

class Translation extends Translations {
  @override
  Map<String, Map<String, String>> get keys => {
        "zh_Hans": zhHans,
        "en": en,
      };
}

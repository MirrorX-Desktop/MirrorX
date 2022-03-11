import 'package:get/get.dart';

class MainController extends GetxController {
  MainController();

  final _obj = ''.obs;
  set obj(value) => this._obj.value = value;
  get obj => this._obj.value;
}

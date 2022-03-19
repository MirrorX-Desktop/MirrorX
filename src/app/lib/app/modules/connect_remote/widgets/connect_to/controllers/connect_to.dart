import 'package:get/get.dart';
import 'package:mirrorx/app/modules/connect_remote/widgets/connect_to/controllers/digit_input.dart';

class ConnectToController extends GetxController with StateMixin<bool> {
  late DigitInputController _digitInputController;

  @override
  void onInit() {
    _digitInputController = Get.put(DigitInputController());
    super.onInit();
  }

  void connectTo() {
    change(null, status: RxStatus.loading());
    final deviceID = _digitInputController.inputText;
  }
}

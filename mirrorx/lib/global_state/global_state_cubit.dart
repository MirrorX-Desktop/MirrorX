import 'package:equatable/equatable.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mirrorx/env/sdk/mirrorx_core_sdk.dart';

part 'global_state_state.dart';

class GlobalStateCubit extends Cubit<GlobalState> {
  GlobalStateCubit() : super(const GlobalState());

  Future<String> fetchDeviceID() async {
    if (state.deviceID != null) {
      return state.deviceID!;
    }

    try {
      final deviceID = await MirrorXCoreSDK.instance.configReadDeviceId();

      if (deviceID == null) {
        return Future.error("device id is null");
      }

      emit(state.copyWith(deviceID: deviceID));

      return deviceID;
    } catch (error) {
      return Future.error(error);
    }
  }

  Future<String> fetchDevicePassword() async {
    if (state.devicePassword != null) {
      return state.devicePassword!;
    }

    try {
      final devicePassword =
          await MirrorXCoreSDK.instance.configReadDevicePassword();

      if (devicePassword == null) {
        return Future.error("device password is null");
      }

      emit(state.copyWith(devicePassword: devicePassword));

      return devicePassword;
    } catch (error) {
      return Future.error(error);
    }
  }

  Future<void> updateDevicePassword(String? newPassword) async {
    try {
      newPassword ??=
          await MirrorXCoreSDK.instance.utilityGenerateDevicePassword();

      await MirrorXCoreSDK.instance
          .configSaveDevicePassword(devicePassword: newPassword);

      emit(state.copyWith(devicePassword: newPassword));
    } catch (error) {
      return Future.error(error);
    }
  }
}

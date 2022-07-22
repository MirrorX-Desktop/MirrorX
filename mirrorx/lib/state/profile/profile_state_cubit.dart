import 'dart:developer';

import 'package:equatable/equatable.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mirrorx/env/sdk/mirrorx_core_sdk.dart';
import 'package:mirrorx/env/utility/rng.dart';

part 'profile_state.dart';

class ProfileStateCubit extends Cubit<ProfileState> {
  ProfileStateCubit() : super(const ProfileState());

  Future<String> getDeviceID() async {
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

  Future<String> getDevicePassword() async {
    if (state.devicePassword != null) {
      return state.devicePassword!;
    }

    try {
      var devicePassword =
          await MirrorXCoreSDK.instance.configReadDevicePassword();

      if (devicePassword == null) {
        devicePassword = generatePassword();
        await MirrorXCoreSDK.instance
            .configSaveDevicePassword(devicePassword: devicePassword);
      }

      emit(state.copyWith(devicePassword: devicePassword));

      return devicePassword;
    } catch (error) {
      return Future.error(error);
    }
  }

  Future<void> updateDevicePassword(String? newPassword) async {
    try {
      newPassword ??= generatePassword();

      await MirrorXCoreSDK.instance
          .configSaveDevicePassword(devicePassword: newPassword);

      emit(state.copyWith(devicePassword: newPassword));
    } catch (error) {
      return Future.error(error);
    }
  }

  void changeLocale(Locale? locale) {
    log("change locale ${locale}");
    emit(state.copyWith(locale: locale));
  }
}

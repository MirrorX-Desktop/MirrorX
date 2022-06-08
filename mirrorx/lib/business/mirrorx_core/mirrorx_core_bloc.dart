import 'dart:developer';
import 'dart:ffi';
import 'dart:io';

import 'package:bloc/bloc.dart';
import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';
import 'package:mirrorx/mirrorx_core.dart';
import 'package:path_provider/path_provider.dart';

part 'mirrorx_core_event.dart';
part 'mirrorx_core_state.dart';

class MirrorXCoreBloc extends Bloc<MirrorXCoreEvent, MirrorXCoreState> {
  MirrorXCoreBloc() : super(const MirrorXCoreState()) {
    on<MirrorXCoreInit>(_init);
    on<MirrorXCoreConfigReadDeviceId>(_configReadDeviceId);
    on<MirrorXCoreConfigReadDevicePassword>(_configReadDevicePassword);
    on<MirrorXCoreConfigSaveDevicePassword>(_configSaveDevicePassword);
    on<MirrorXCoreGenerateDevicePassword>(_generateDevicePassword);
  }

  Future<void> _init(
      MirrorXCoreInit event, Emitter<MirrorXCoreState> emit) async {
    emit(state.copyWith(status: MirrorXCoreStateStatus.loading));

    try {
      final core = MirrorXCoreImpl(_dylib);

      final applicationSupportDir = await getApplicationSupportDirectory();
      log("application support dir: $applicationSupportDir");

      await core.init(
          osName: Platform.operatingSystem,
          osVersion: Platform.operatingSystemVersion,
          configDir: applicationSupportDir.path);

      emit(state.copyWith(core: core, status: MirrorXCoreStateStatus.success));
    } catch (error) {
      emit(state.copyWith(
          status: MirrorXCoreStateStatus.failure, lastError: error));
    }
  }

  Future<void> _configReadDeviceId(MirrorXCoreConfigReadDeviceId event,
      Emitter<MirrorXCoreState> emit) async {
    try {
      final deviceId = await state.core!.configReadDeviceId();
      log("device id: $deviceId");
      emit(state.copyWith(deviceId: deviceId));
    } catch (error) {
      emit(state.copyWith(lastError: error));
    }
  }

  Future<void> _configReadDevicePassword(
      MirrorXCoreConfigReadDevicePassword event,
      Emitter<MirrorXCoreState> emit) async {
    try {
      var password = await state.core!.configReadDevicePassword();
      if (password == null) {
        password = await state.core!.utilityGenerateDevicePassword();
        await state.core!.configSaveDevicePassword(devicePassword: password);
      }

      emit(state.copyWith(password: password));
    } catch (error) {
      emit(state.copyWith(lastError: error));
    }
  }

  Future<void> _configSaveDevicePassword(
      MirrorXCoreConfigSaveDevicePassword event,
      Emitter<MirrorXCoreState> emit) async {
    try {
      await state.core!
          .configSaveDevicePassword(devicePassword: event.devicePassword);
      emit(state.copyWith(password: event.devicePassword));
    } catch (error) {
      emit(state.copyWith(lastError: error));
    }
  }

  Future<void> _generateDevicePassword(MirrorXCoreGenerateDevicePassword event,
      Emitter<MirrorXCoreState> emit) async {
    try {
      final password = await state.core!.utilityGenerateDevicePassword();
      await state.core!.configSaveDevicePassword(devicePassword: password);
      emit(state.copyWith(password: password));
    } catch (error) {
      emit(state.copyWith(lastError: error));
    }
  }
}

const String _libName = 'mirrorx_core';

final DynamicLibrary _dylib = () {
  if (Platform.isIOS) {
    return DynamicLibrary.process();
  }

  if (Platform.isMacOS) {
    return DynamicLibrary.executable();
  }
  if (Platform.isAndroid || Platform.isLinux) {
    return DynamicLibrary.open('lib$_libName.so');
  }
  if (Platform.isWindows) {
    return DynamicLibrary.open('$_libName.dll');
  }
  throw UnsupportedError('Unknown platform: ${Platform.operatingSystem}');
}();

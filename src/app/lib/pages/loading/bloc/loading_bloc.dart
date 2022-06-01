import 'dart:developer';
import 'dart:ffi';
import 'dart:io';

import 'package:app_plugin/app_plugin.dart';
import 'package:app_plugin/bridge_generated.dart';
import 'package:bloc/bloc.dart';
import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';
import 'package:path_provider/path_provider.dart';

part 'loading_event.dart';
part 'loading_state.dart';

class LoadingBloc extends Bloc<LoadingEvent, LoadingState> {
  LoadingBloc() : super(const LoadingState()) {
    on<LoadingEventLoad>(_load);
  }

  Future<void> _load(LoadingEventLoad event, Emitter<LoadingState> emit) async {
    emit(state.copyWith(status: LoadingStateStatus.loading));

    try {
      final MirrorXCore core = MirrorXCoreImpl(_openLibrary());

      final applicationSupportDir = await getApplicationSupportDirectory();
      log("application support dir: $applicationSupportDir");

      await core.init(
          osName: Platform.operatingSystem,
          osVersion: Platform.operatingSystemVersion,
          configDir: applicationSupportDir.path);

      // await Future.delayed(Duration(seconds: 3));

      emit(state.copyWith(status: LoadingStateStatus.success));
    } catch (error) {
      emit(state.copyWith(
          status: LoadingStateStatus.failure, loadingError: error));
    }
  }
}

DynamicLibrary _openLibrary() {
  if (Platform.isMacOS || Platform.isIOS) {
    return DynamicLibrary.executable();
  } else if (Platform.isWindows) {
    return DynamicLibrary.open("mirrorx_core.dll");
  } else {
    throw UnsupportedError("Not supported platform yet");
  }
}

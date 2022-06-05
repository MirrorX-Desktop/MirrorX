part of 'mirrorx_core_bloc.dart';

@immutable
abstract class MirrorXCoreEvent {}

class MirrorXCoreInit extends MirrorXCoreEvent {}

class MirrorXCoreConfigReadDeviceId extends MirrorXCoreEvent {}

class MirrorXCoreConfigReadDevicePassword extends MirrorXCoreEvent {}

class MirrorXCoreConfigSaveDevicePassword extends MirrorXCoreEvent {
  final String devicePassword;

  MirrorXCoreConfigSaveDevicePassword({required this.devicePassword});
}

class MirrorXCoreGenerateDevicePassword extends MirrorXCoreEvent {}

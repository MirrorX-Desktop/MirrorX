part of 'global_state_cubit.dart';

class GlobalState extends Equatable {
  const GlobalState({this.deviceID, this.devicePassword});

  final String? deviceID;
  final String? devicePassword;

  GlobalState copyWith({String? deviceID, String? devicePassword}) =>
      GlobalState(
        deviceID: deviceID ?? this.deviceID,
        devicePassword: devicePassword ?? this.devicePassword,
      );

  @override
  List<Object?> get props => [deviceID, devicePassword];
}

part of 'profile_state_cubit.dart';

class ProfileState extends Equatable {
  const ProfileState({
    this.deviceID,
    this.devicePassword,
  });

  final String? deviceID;
  final String? devicePassword;

  ProfileState copyWith({
    String? deviceID,
    String? devicePassword,
  }) =>
      ProfileState(
        deviceID: deviceID ?? this.deviceID,
        devicePassword: devicePassword ?? this.devicePassword,
      );

  @override
  List<Object?> get props => [deviceID, devicePassword];
}

part of 'profile_state_cubit.dart';

class ProfileState extends Equatable {
  const ProfileState({
    this.deviceID,
    this.devicePassword,
    this.locale,
  });

  final String? deviceID;
  final String? devicePassword;
  final Locale? locale;

  ProfileState copyWith({
    String? deviceID,
    String? devicePassword,
    Locale? locale,
  }) =>
      ProfileState(
        deviceID: deviceID ?? this.deviceID,
        devicePassword: devicePassword ?? this.devicePassword,
        locale: locale,
      );

  @override
  List<Object?> get props => [deviceID, devicePassword, locale];
}

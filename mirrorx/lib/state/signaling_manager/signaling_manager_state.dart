part of 'signaling_manager_cubit.dart';

enum SignalingConnectionState {
  connecting,
  connected,
  disconnected,
}

class SignalingManagerState extends Equatable {
  const SignalingManagerState({
    this.connectionState,
    this.domain,
    this.domainConfig,
  });

  final SignalingConnectionState? connectionState;
  final String? domain;
  final DomainConfig? domainConfig;

  SignalingManagerState copyWith({
    SignalingConnectionState? connectionState,
    String? domain,
    DomainConfig? domainConfig,
  }) =>
      SignalingManagerState(
        connectionState: connectionState ?? this.connectionState,
        domain: domain ?? this.domain,
        domainConfig: domainConfig ?? this.domainConfig,
      );

  @override
  List<Object?> get props => [connectionState, domain, domainConfig];
}

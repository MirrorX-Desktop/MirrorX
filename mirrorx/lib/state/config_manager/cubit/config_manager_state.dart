part of 'config_manager_cubit.dart';

class ConfigManagerState extends Equatable {
  const ConfigManagerState({this.configPath, this.domain, this.domainConfig});

  final String? configPath;
  final String? domain;
  final DomainConfig? domainConfig;

  ConfigManagerState copyWith({
    String? configPath,
    String? domain,
    DomainConfig? domainConfig,
  }) =>
      ConfigManagerState(
        configPath: configPath ?? this.configPath,
        domain: domain ?? this.domain,
        domainConfig: domainConfig ?? this.domainConfig,
      );

  @override
  List<Object?> get props => [configPath, domain, domainConfig];
}

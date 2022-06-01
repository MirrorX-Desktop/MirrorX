part of 'loading_bloc.dart';

enum LoadingStateStatus { initial, loading, success, failure }

extension LoadingStateStatusExtension on LoadingStateStatus {
  bool get isInitial => this == LoadingStateStatus.initial;
  bool get isLoading => this == LoadingStateStatus.loading;
  bool get isSuccess => this == LoadingStateStatus.success;
  bool get isFailure => this == LoadingStateStatus.failure;
}

class LoadingState extends Equatable {
  const LoadingState(
      {this.status = LoadingStateStatus.initial, this.loadingError});

  final LoadingStateStatus status;
  final Object? loadingError;

  LoadingState copyWith({LoadingStateStatus? status, Object? loadingError}) =>
      LoadingState(
          status: status ?? this.status,
          loadingError: loadingError ?? this.loadingError);

  @override
  List<Object> get props => [status];
}

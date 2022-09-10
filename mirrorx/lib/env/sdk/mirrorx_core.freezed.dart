// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target

part of 'mirrorx_core.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$PublishMessage {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() streamClosed,
    required TResult Function(String activeDeviceId, String passiveDeviceId,
            ResourceType resourceType)
        visitRequest,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? streamClosed,
    TResult Function(String activeDeviceId, String passiveDeviceId,
            ResourceType resourceType)?
        visitRequest,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? streamClosed,
    TResult Function(String activeDeviceId, String passiveDeviceId,
            ResourceType resourceType)?
        visitRequest,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(PublishMessage_StreamClosed value) streamClosed,
    required TResult Function(PublishMessage_VisitRequest value) visitRequest,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(PublishMessage_StreamClosed value)? streamClosed,
    TResult Function(PublishMessage_VisitRequest value)? visitRequest,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(PublishMessage_StreamClosed value)? streamClosed,
    TResult Function(PublishMessage_VisitRequest value)? visitRequest,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $PublishMessageCopyWith<$Res> {
  factory $PublishMessageCopyWith(
          PublishMessage value, $Res Function(PublishMessage) then) =
      _$PublishMessageCopyWithImpl<$Res>;
}

/// @nodoc
class _$PublishMessageCopyWithImpl<$Res>
    implements $PublishMessageCopyWith<$Res> {
  _$PublishMessageCopyWithImpl(this._value, this._then);

  final PublishMessage _value;
  // ignore: unused_field
  final $Res Function(PublishMessage) _then;
}

/// @nodoc
abstract class _$$PublishMessage_StreamClosedCopyWith<$Res> {
  factory _$$PublishMessage_StreamClosedCopyWith(
          _$PublishMessage_StreamClosed value,
          $Res Function(_$PublishMessage_StreamClosed) then) =
      __$$PublishMessage_StreamClosedCopyWithImpl<$Res>;
}

/// @nodoc
class __$$PublishMessage_StreamClosedCopyWithImpl<$Res>
    extends _$PublishMessageCopyWithImpl<$Res>
    implements _$$PublishMessage_StreamClosedCopyWith<$Res> {
  __$$PublishMessage_StreamClosedCopyWithImpl(
      _$PublishMessage_StreamClosed _value,
      $Res Function(_$PublishMessage_StreamClosed) _then)
      : super(_value, (v) => _then(v as _$PublishMessage_StreamClosed));

  @override
  _$PublishMessage_StreamClosed get _value =>
      super._value as _$PublishMessage_StreamClosed;
}

/// @nodoc

class _$PublishMessage_StreamClosed implements PublishMessage_StreamClosed {
  const _$PublishMessage_StreamClosed();

  @override
  String toString() {
    return 'PublishMessage.streamClosed()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PublishMessage_StreamClosed);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() streamClosed,
    required TResult Function(String activeDeviceId, String passiveDeviceId,
            ResourceType resourceType)
        visitRequest,
  }) {
    return streamClosed();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? streamClosed,
    TResult Function(String activeDeviceId, String passiveDeviceId,
            ResourceType resourceType)?
        visitRequest,
  }) {
    return streamClosed?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? streamClosed,
    TResult Function(String activeDeviceId, String passiveDeviceId,
            ResourceType resourceType)?
        visitRequest,
    required TResult orElse(),
  }) {
    if (streamClosed != null) {
      return streamClosed();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(PublishMessage_StreamClosed value) streamClosed,
    required TResult Function(PublishMessage_VisitRequest value) visitRequest,
  }) {
    return streamClosed(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(PublishMessage_StreamClosed value)? streamClosed,
    TResult Function(PublishMessage_VisitRequest value)? visitRequest,
  }) {
    return streamClosed?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(PublishMessage_StreamClosed value)? streamClosed,
    TResult Function(PublishMessage_VisitRequest value)? visitRequest,
    required TResult orElse(),
  }) {
    if (streamClosed != null) {
      return streamClosed(this);
    }
    return orElse();
  }
}

abstract class PublishMessage_StreamClosed implements PublishMessage {
  const factory PublishMessage_StreamClosed() = _$PublishMessage_StreamClosed;
}

/// @nodoc
abstract class _$$PublishMessage_VisitRequestCopyWith<$Res> {
  factory _$$PublishMessage_VisitRequestCopyWith(
          _$PublishMessage_VisitRequest value,
          $Res Function(_$PublishMessage_VisitRequest) then) =
      __$$PublishMessage_VisitRequestCopyWithImpl<$Res>;
  $Res call(
      {String activeDeviceId,
      String passiveDeviceId,
      ResourceType resourceType});
}

/// @nodoc
class __$$PublishMessage_VisitRequestCopyWithImpl<$Res>
    extends _$PublishMessageCopyWithImpl<$Res>
    implements _$$PublishMessage_VisitRequestCopyWith<$Res> {
  __$$PublishMessage_VisitRequestCopyWithImpl(
      _$PublishMessage_VisitRequest _value,
      $Res Function(_$PublishMessage_VisitRequest) _then)
      : super(_value, (v) => _then(v as _$PublishMessage_VisitRequest));

  @override
  _$PublishMessage_VisitRequest get _value =>
      super._value as _$PublishMessage_VisitRequest;

  @override
  $Res call({
    Object? activeDeviceId = freezed,
    Object? passiveDeviceId = freezed,
    Object? resourceType = freezed,
  }) {
    return _then(_$PublishMessage_VisitRequest(
      activeDeviceId: activeDeviceId == freezed
          ? _value.activeDeviceId
          : activeDeviceId // ignore: cast_nullable_to_non_nullable
              as String,
      passiveDeviceId: passiveDeviceId == freezed
          ? _value.passiveDeviceId
          : passiveDeviceId // ignore: cast_nullable_to_non_nullable
              as String,
      resourceType: resourceType == freezed
          ? _value.resourceType
          : resourceType // ignore: cast_nullable_to_non_nullable
              as ResourceType,
    ));
  }
}

/// @nodoc

class _$PublishMessage_VisitRequest implements PublishMessage_VisitRequest {
  const _$PublishMessage_VisitRequest(
      {required this.activeDeviceId,
      required this.passiveDeviceId,
      required this.resourceType});

  @override
  final String activeDeviceId;
  @override
  final String passiveDeviceId;
  @override
  final ResourceType resourceType;

  @override
  String toString() {
    return 'PublishMessage.visitRequest(activeDeviceId: $activeDeviceId, passiveDeviceId: $passiveDeviceId, resourceType: $resourceType)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PublishMessage_VisitRequest &&
            const DeepCollectionEquality()
                .equals(other.activeDeviceId, activeDeviceId) &&
            const DeepCollectionEquality()
                .equals(other.passiveDeviceId, passiveDeviceId) &&
            const DeepCollectionEquality()
                .equals(other.resourceType, resourceType));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType,
      const DeepCollectionEquality().hash(activeDeviceId),
      const DeepCollectionEquality().hash(passiveDeviceId),
      const DeepCollectionEquality().hash(resourceType));

  @JsonKey(ignore: true)
  @override
  _$$PublishMessage_VisitRequestCopyWith<_$PublishMessage_VisitRequest>
      get copyWith => __$$PublishMessage_VisitRequestCopyWithImpl<
          _$PublishMessage_VisitRequest>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() streamClosed,
    required TResult Function(String activeDeviceId, String passiveDeviceId,
            ResourceType resourceType)
        visitRequest,
  }) {
    return visitRequest(activeDeviceId, passiveDeviceId, resourceType);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? streamClosed,
    TResult Function(String activeDeviceId, String passiveDeviceId,
            ResourceType resourceType)?
        visitRequest,
  }) {
    return visitRequest?.call(activeDeviceId, passiveDeviceId, resourceType);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? streamClosed,
    TResult Function(String activeDeviceId, String passiveDeviceId,
            ResourceType resourceType)?
        visitRequest,
    required TResult orElse(),
  }) {
    if (visitRequest != null) {
      return visitRequest(activeDeviceId, passiveDeviceId, resourceType);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(PublishMessage_StreamClosed value) streamClosed,
    required TResult Function(PublishMessage_VisitRequest value) visitRequest,
  }) {
    return visitRequest(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(PublishMessage_StreamClosed value)? streamClosed,
    TResult Function(PublishMessage_VisitRequest value)? visitRequest,
  }) {
    return visitRequest?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(PublishMessage_StreamClosed value)? streamClosed,
    TResult Function(PublishMessage_VisitRequest value)? visitRequest,
    required TResult orElse(),
  }) {
    if (visitRequest != null) {
      return visitRequest(this);
    }
    return orElse();
  }
}

abstract class PublishMessage_VisitRequest implements PublishMessage {
  const factory PublishMessage_VisitRequest(
          {required final String activeDeviceId,
          required final String passiveDeviceId,
          required final ResourceType resourceType}) =
      _$PublishMessage_VisitRequest;

  String get activeDeviceId;
  String get passiveDeviceId;
  ResourceType get resourceType;
  @JsonKey(ignore: true)
  _$$PublishMessage_VisitRequestCopyWith<_$PublishMessage_VisitRequest>
      get copyWith => throw _privateConstructorUsedError;
}

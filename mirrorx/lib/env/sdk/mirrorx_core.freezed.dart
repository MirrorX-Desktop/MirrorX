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
mixin _$MouseEvent {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(MouseKey field0) up,
    required TResult Function(MouseKey field0) down,
    required TResult Function(MouseKey field0) move,
    required TResult Function(double field0) scrollWheel,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(MouseKey field0)? up,
    TResult Function(MouseKey field0)? down,
    TResult Function(MouseKey field0)? move,
    TResult Function(double field0)? scrollWheel,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(MouseKey field0)? up,
    TResult Function(MouseKey field0)? down,
    TResult Function(MouseKey field0)? move,
    TResult Function(double field0)? scrollWheel,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Up value) up,
    required TResult Function(Down value) down,
    required TResult Function(Move value) move,
    required TResult Function(ScrollWheel value) scrollWheel,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Up value)? up,
    TResult Function(Down value)? down,
    TResult Function(Move value)? move,
    TResult Function(ScrollWheel value)? scrollWheel,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Up value)? up,
    TResult Function(Down value)? down,
    TResult Function(Move value)? move,
    TResult Function(ScrollWheel value)? scrollWheel,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $MouseEventCopyWith<$Res> {
  factory $MouseEventCopyWith(
          MouseEvent value, $Res Function(MouseEvent) then) =
      _$MouseEventCopyWithImpl<$Res>;
}

/// @nodoc
class _$MouseEventCopyWithImpl<$Res> implements $MouseEventCopyWith<$Res> {
  _$MouseEventCopyWithImpl(this._value, this._then);

  final MouseEvent _value;
  // ignore: unused_field
  final $Res Function(MouseEvent) _then;
}

/// @nodoc
abstract class _$$UpCopyWith<$Res> {
  factory _$$UpCopyWith(_$Up value, $Res Function(_$Up) then) =
      __$$UpCopyWithImpl<$Res>;
  $Res call({MouseKey field0});
}

/// @nodoc
class __$$UpCopyWithImpl<$Res> extends _$MouseEventCopyWithImpl<$Res>
    implements _$$UpCopyWith<$Res> {
  __$$UpCopyWithImpl(_$Up _value, $Res Function(_$Up) _then)
      : super(_value, (v) => _then(v as _$Up));

  @override
  _$Up get _value => super._value as _$Up;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$Up(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as MouseKey,
    ));
  }
}

/// @nodoc

class _$Up implements Up {
  const _$Up(this.field0);

  @override
  final MouseKey field0;

  @override
  String toString() {
    return 'MouseEvent.up(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Up &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$UpCopyWith<_$Up> get copyWith =>
      __$$UpCopyWithImpl<_$Up>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(MouseKey field0) up,
    required TResult Function(MouseKey field0) down,
    required TResult Function(MouseKey field0) move,
    required TResult Function(double field0) scrollWheel,
  }) {
    return up(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(MouseKey field0)? up,
    TResult Function(MouseKey field0)? down,
    TResult Function(MouseKey field0)? move,
    TResult Function(double field0)? scrollWheel,
  }) {
    return up?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(MouseKey field0)? up,
    TResult Function(MouseKey field0)? down,
    TResult Function(MouseKey field0)? move,
    TResult Function(double field0)? scrollWheel,
    required TResult orElse(),
  }) {
    if (up != null) {
      return up(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Up value) up,
    required TResult Function(Down value) down,
    required TResult Function(Move value) move,
    required TResult Function(ScrollWheel value) scrollWheel,
  }) {
    return up(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Up value)? up,
    TResult Function(Down value)? down,
    TResult Function(Move value)? move,
    TResult Function(ScrollWheel value)? scrollWheel,
  }) {
    return up?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Up value)? up,
    TResult Function(Down value)? down,
    TResult Function(Move value)? move,
    TResult Function(ScrollWheel value)? scrollWheel,
    required TResult orElse(),
  }) {
    if (up != null) {
      return up(this);
    }
    return orElse();
  }
}

abstract class Up implements MouseEvent {
  const factory Up(final MouseKey field0) = _$Up;

  MouseKey get field0;
  @JsonKey(ignore: true)
  _$$UpCopyWith<_$Up> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$DownCopyWith<$Res> {
  factory _$$DownCopyWith(_$Down value, $Res Function(_$Down) then) =
      __$$DownCopyWithImpl<$Res>;
  $Res call({MouseKey field0});
}

/// @nodoc
class __$$DownCopyWithImpl<$Res> extends _$MouseEventCopyWithImpl<$Res>
    implements _$$DownCopyWith<$Res> {
  __$$DownCopyWithImpl(_$Down _value, $Res Function(_$Down) _then)
      : super(_value, (v) => _then(v as _$Down));

  @override
  _$Down get _value => super._value as _$Down;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$Down(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as MouseKey,
    ));
  }
}

/// @nodoc

class _$Down implements Down {
  const _$Down(this.field0);

  @override
  final MouseKey field0;

  @override
  String toString() {
    return 'MouseEvent.down(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Down &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$DownCopyWith<_$Down> get copyWith =>
      __$$DownCopyWithImpl<_$Down>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(MouseKey field0) up,
    required TResult Function(MouseKey field0) down,
    required TResult Function(MouseKey field0) move,
    required TResult Function(double field0) scrollWheel,
  }) {
    return down(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(MouseKey field0)? up,
    TResult Function(MouseKey field0)? down,
    TResult Function(MouseKey field0)? move,
    TResult Function(double field0)? scrollWheel,
  }) {
    return down?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(MouseKey field0)? up,
    TResult Function(MouseKey field0)? down,
    TResult Function(MouseKey field0)? move,
    TResult Function(double field0)? scrollWheel,
    required TResult orElse(),
  }) {
    if (down != null) {
      return down(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Up value) up,
    required TResult Function(Down value) down,
    required TResult Function(Move value) move,
    required TResult Function(ScrollWheel value) scrollWheel,
  }) {
    return down(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Up value)? up,
    TResult Function(Down value)? down,
    TResult Function(Move value)? move,
    TResult Function(ScrollWheel value)? scrollWheel,
  }) {
    return down?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Up value)? up,
    TResult Function(Down value)? down,
    TResult Function(Move value)? move,
    TResult Function(ScrollWheel value)? scrollWheel,
    required TResult orElse(),
  }) {
    if (down != null) {
      return down(this);
    }
    return orElse();
  }
}

abstract class Down implements MouseEvent {
  const factory Down(final MouseKey field0) = _$Down;

  MouseKey get field0;
  @JsonKey(ignore: true)
  _$$DownCopyWith<_$Down> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$MoveCopyWith<$Res> {
  factory _$$MoveCopyWith(_$Move value, $Res Function(_$Move) then) =
      __$$MoveCopyWithImpl<$Res>;
  $Res call({MouseKey field0});
}

/// @nodoc
class __$$MoveCopyWithImpl<$Res> extends _$MouseEventCopyWithImpl<$Res>
    implements _$$MoveCopyWith<$Res> {
  __$$MoveCopyWithImpl(_$Move _value, $Res Function(_$Move) _then)
      : super(_value, (v) => _then(v as _$Move));

  @override
  _$Move get _value => super._value as _$Move;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$Move(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as MouseKey,
    ));
  }
}

/// @nodoc

class _$Move implements Move {
  const _$Move(this.field0);

  @override
  final MouseKey field0;

  @override
  String toString() {
    return 'MouseEvent.move(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Move &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$MoveCopyWith<_$Move> get copyWith =>
      __$$MoveCopyWithImpl<_$Move>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(MouseKey field0) up,
    required TResult Function(MouseKey field0) down,
    required TResult Function(MouseKey field0) move,
    required TResult Function(double field0) scrollWheel,
  }) {
    return move(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(MouseKey field0)? up,
    TResult Function(MouseKey field0)? down,
    TResult Function(MouseKey field0)? move,
    TResult Function(double field0)? scrollWheel,
  }) {
    return move?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(MouseKey field0)? up,
    TResult Function(MouseKey field0)? down,
    TResult Function(MouseKey field0)? move,
    TResult Function(double field0)? scrollWheel,
    required TResult orElse(),
  }) {
    if (move != null) {
      return move(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Up value) up,
    required TResult Function(Down value) down,
    required TResult Function(Move value) move,
    required TResult Function(ScrollWheel value) scrollWheel,
  }) {
    return move(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Up value)? up,
    TResult Function(Down value)? down,
    TResult Function(Move value)? move,
    TResult Function(ScrollWheel value)? scrollWheel,
  }) {
    return move?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Up value)? up,
    TResult Function(Down value)? down,
    TResult Function(Move value)? move,
    TResult Function(ScrollWheel value)? scrollWheel,
    required TResult orElse(),
  }) {
    if (move != null) {
      return move(this);
    }
    return orElse();
  }
}

abstract class Move implements MouseEvent {
  const factory Move(final MouseKey field0) = _$Move;

  MouseKey get field0;
  @JsonKey(ignore: true)
  _$$MoveCopyWith<_$Move> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$ScrollWheelCopyWith<$Res> {
  factory _$$ScrollWheelCopyWith(
          _$ScrollWheel value, $Res Function(_$ScrollWheel) then) =
      __$$ScrollWheelCopyWithImpl<$Res>;
  $Res call({double field0});
}

/// @nodoc
class __$$ScrollWheelCopyWithImpl<$Res> extends _$MouseEventCopyWithImpl<$Res>
    implements _$$ScrollWheelCopyWith<$Res> {
  __$$ScrollWheelCopyWithImpl(
      _$ScrollWheel _value, $Res Function(_$ScrollWheel) _then)
      : super(_value, (v) => _then(v as _$ScrollWheel));

  @override
  _$ScrollWheel get _value => super._value as _$ScrollWheel;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$ScrollWheel(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as double,
    ));
  }
}

/// @nodoc

class _$ScrollWheel implements ScrollWheel {
  const _$ScrollWheel(this.field0);

  @override
  final double field0;

  @override
  String toString() {
    return 'MouseEvent.scrollWheel(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ScrollWheel &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$ScrollWheelCopyWith<_$ScrollWheel> get copyWith =>
      __$$ScrollWheelCopyWithImpl<_$ScrollWheel>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(MouseKey field0) up,
    required TResult Function(MouseKey field0) down,
    required TResult Function(MouseKey field0) move,
    required TResult Function(double field0) scrollWheel,
  }) {
    return scrollWheel(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(MouseKey field0)? up,
    TResult Function(MouseKey field0)? down,
    TResult Function(MouseKey field0)? move,
    TResult Function(double field0)? scrollWheel,
  }) {
    return scrollWheel?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(MouseKey field0)? up,
    TResult Function(MouseKey field0)? down,
    TResult Function(MouseKey field0)? move,
    TResult Function(double field0)? scrollWheel,
    required TResult orElse(),
  }) {
    if (scrollWheel != null) {
      return scrollWheel(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Up value) up,
    required TResult Function(Down value) down,
    required TResult Function(Move value) move,
    required TResult Function(ScrollWheel value) scrollWheel,
  }) {
    return scrollWheel(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Up value)? up,
    TResult Function(Down value)? down,
    TResult Function(Move value)? move,
    TResult Function(ScrollWheel value)? scrollWheel,
  }) {
    return scrollWheel?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Up value)? up,
    TResult Function(Down value)? down,
    TResult Function(Move value)? move,
    TResult Function(ScrollWheel value)? scrollWheel,
    required TResult orElse(),
  }) {
    if (scrollWheel != null) {
      return scrollWheel(this);
    }
    return orElse();
  }
}

abstract class ScrollWheel implements MouseEvent {
  const factory ScrollWheel(final double field0) = _$ScrollWheel;

  double get field0;
  @JsonKey(ignore: true)
  _$$ScrollWheelCopyWith<_$ScrollWheel> get copyWith =>
      throw _privateConstructorUsedError;
}

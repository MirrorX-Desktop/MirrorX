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
mixin _$InputEvent {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(MouseEvent field0) mouse,
    required TResult Function(KeyboardEvent field0) keyboard,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(MouseEvent field0)? mouse,
    TResult Function(KeyboardEvent field0)? keyboard,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(MouseEvent field0)? mouse,
    TResult Function(KeyboardEvent field0)? keyboard,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Mouse value) mouse,
    required TResult Function(Keyboard value) keyboard,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Mouse value)? mouse,
    TResult Function(Keyboard value)? keyboard,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Mouse value)? mouse,
    TResult Function(Keyboard value)? keyboard,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $InputEventCopyWith<$Res> {
  factory $InputEventCopyWith(
          InputEvent value, $Res Function(InputEvent) then) =
      _$InputEventCopyWithImpl<$Res>;
}

/// @nodoc
class _$InputEventCopyWithImpl<$Res> implements $InputEventCopyWith<$Res> {
  _$InputEventCopyWithImpl(this._value, this._then);

  final InputEvent _value;
  // ignore: unused_field
  final $Res Function(InputEvent) _then;
}

/// @nodoc
abstract class _$$MouseCopyWith<$Res> {
  factory _$$MouseCopyWith(_$Mouse value, $Res Function(_$Mouse) then) =
      __$$MouseCopyWithImpl<$Res>;
  $Res call({MouseEvent field0});

  $MouseEventCopyWith<$Res> get field0;
}

/// @nodoc
class __$$MouseCopyWithImpl<$Res> extends _$InputEventCopyWithImpl<$Res>
    implements _$$MouseCopyWith<$Res> {
  __$$MouseCopyWithImpl(_$Mouse _value, $Res Function(_$Mouse) _then)
      : super(_value, (v) => _then(v as _$Mouse));

  @override
  _$Mouse get _value => super._value as _$Mouse;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$Mouse(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as MouseEvent,
    ));
  }

  @override
  $MouseEventCopyWith<$Res> get field0 {
    return $MouseEventCopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc

class _$Mouse implements Mouse {
  const _$Mouse(this.field0);

  @override
  final MouseEvent field0;

  @override
  String toString() {
    return 'InputEvent.mouse(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Mouse &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$MouseCopyWith<_$Mouse> get copyWith =>
      __$$MouseCopyWithImpl<_$Mouse>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(MouseEvent field0) mouse,
    required TResult Function(KeyboardEvent field0) keyboard,
  }) {
    return mouse(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(MouseEvent field0)? mouse,
    TResult Function(KeyboardEvent field0)? keyboard,
  }) {
    return mouse?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(MouseEvent field0)? mouse,
    TResult Function(KeyboardEvent field0)? keyboard,
    required TResult orElse(),
  }) {
    if (mouse != null) {
      return mouse(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Mouse value) mouse,
    required TResult Function(Keyboard value) keyboard,
  }) {
    return mouse(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Mouse value)? mouse,
    TResult Function(Keyboard value)? keyboard,
  }) {
    return mouse?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Mouse value)? mouse,
    TResult Function(Keyboard value)? keyboard,
    required TResult orElse(),
  }) {
    if (mouse != null) {
      return mouse(this);
    }
    return orElse();
  }
}

abstract class Mouse implements InputEvent {
  const factory Mouse(final MouseEvent field0) = _$Mouse;

  MouseEvent get field0;
  @JsonKey(ignore: true)
  _$$MouseCopyWith<_$Mouse> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$KeyboardCopyWith<$Res> {
  factory _$$KeyboardCopyWith(
          _$Keyboard value, $Res Function(_$Keyboard) then) =
      __$$KeyboardCopyWithImpl<$Res>;
  $Res call({KeyboardEvent field0});

  $KeyboardEventCopyWith<$Res> get field0;
}

/// @nodoc
class __$$KeyboardCopyWithImpl<$Res> extends _$InputEventCopyWithImpl<$Res>
    implements _$$KeyboardCopyWith<$Res> {
  __$$KeyboardCopyWithImpl(_$Keyboard _value, $Res Function(_$Keyboard) _then)
      : super(_value, (v) => _then(v as _$Keyboard));

  @override
  _$Keyboard get _value => super._value as _$Keyboard;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$Keyboard(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as KeyboardEvent,
    ));
  }

  @override
  $KeyboardEventCopyWith<$Res> get field0 {
    return $KeyboardEventCopyWith<$Res>(_value.field0, (value) {
      return _then(_value.copyWith(field0: value));
    });
  }
}

/// @nodoc

class _$Keyboard implements Keyboard {
  const _$Keyboard(this.field0);

  @override
  final KeyboardEvent field0;

  @override
  String toString() {
    return 'InputEvent.keyboard(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Keyboard &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$KeyboardCopyWith<_$Keyboard> get copyWith =>
      __$$KeyboardCopyWithImpl<_$Keyboard>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(MouseEvent field0) mouse,
    required TResult Function(KeyboardEvent field0) keyboard,
  }) {
    return keyboard(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(MouseEvent field0)? mouse,
    TResult Function(KeyboardEvent field0)? keyboard,
  }) {
    return keyboard?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(MouseEvent field0)? mouse,
    TResult Function(KeyboardEvent field0)? keyboard,
    required TResult orElse(),
  }) {
    if (keyboard != null) {
      return keyboard(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Mouse value) mouse,
    required TResult Function(Keyboard value) keyboard,
  }) {
    return keyboard(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Mouse value)? mouse,
    TResult Function(Keyboard value)? keyboard,
  }) {
    return keyboard?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Mouse value)? mouse,
    TResult Function(Keyboard value)? keyboard,
    required TResult orElse(),
  }) {
    if (keyboard != null) {
      return keyboard(this);
    }
    return orElse();
  }
}

abstract class Keyboard implements InputEvent {
  const factory Keyboard(final KeyboardEvent field0) = _$Keyboard;

  KeyboardEvent get field0;
  @JsonKey(ignore: true)
  _$$KeyboardCopyWith<_$Keyboard> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$KeyboardEvent {
  KeyboardKey get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(KeyboardKey field0) keyUp,
    required TResult Function(KeyboardKey field0) keyDown,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(KeyboardKey field0)? keyUp,
    TResult Function(KeyboardKey field0)? keyDown,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(KeyboardKey field0)? keyUp,
    TResult Function(KeyboardKey field0)? keyDown,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(KeyUp value) keyUp,
    required TResult Function(KeyDown value) keyDown,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(KeyUp value)? keyUp,
    TResult Function(KeyDown value)? keyDown,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KeyUp value)? keyUp,
    TResult Function(KeyDown value)? keyDown,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $KeyboardEventCopyWith<KeyboardEvent> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $KeyboardEventCopyWith<$Res> {
  factory $KeyboardEventCopyWith(
          KeyboardEvent value, $Res Function(KeyboardEvent) then) =
      _$KeyboardEventCopyWithImpl<$Res>;
  $Res call({KeyboardKey field0});
}

/// @nodoc
class _$KeyboardEventCopyWithImpl<$Res>
    implements $KeyboardEventCopyWith<$Res> {
  _$KeyboardEventCopyWithImpl(this._value, this._then);

  final KeyboardEvent _value;
  // ignore: unused_field
  final $Res Function(KeyboardEvent) _then;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_value.copyWith(
      field0: field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as KeyboardKey,
    ));
  }
}

/// @nodoc
abstract class _$$KeyUpCopyWith<$Res> implements $KeyboardEventCopyWith<$Res> {
  factory _$$KeyUpCopyWith(_$KeyUp value, $Res Function(_$KeyUp) then) =
      __$$KeyUpCopyWithImpl<$Res>;
  @override
  $Res call({KeyboardKey field0});
}

/// @nodoc
class __$$KeyUpCopyWithImpl<$Res> extends _$KeyboardEventCopyWithImpl<$Res>
    implements _$$KeyUpCopyWith<$Res> {
  __$$KeyUpCopyWithImpl(_$KeyUp _value, $Res Function(_$KeyUp) _then)
      : super(_value, (v) => _then(v as _$KeyUp));

  @override
  _$KeyUp get _value => super._value as _$KeyUp;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$KeyUp(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as KeyboardKey,
    ));
  }
}

/// @nodoc

class _$KeyUp implements KeyUp {
  const _$KeyUp(this.field0);

  @override
  final KeyboardKey field0;

  @override
  String toString() {
    return 'KeyboardEvent.keyUp(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$KeyUp &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$KeyUpCopyWith<_$KeyUp> get copyWith =>
      __$$KeyUpCopyWithImpl<_$KeyUp>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(KeyboardKey field0) keyUp,
    required TResult Function(KeyboardKey field0) keyDown,
  }) {
    return keyUp(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(KeyboardKey field0)? keyUp,
    TResult Function(KeyboardKey field0)? keyDown,
  }) {
    return keyUp?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(KeyboardKey field0)? keyUp,
    TResult Function(KeyboardKey field0)? keyDown,
    required TResult orElse(),
  }) {
    if (keyUp != null) {
      return keyUp(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(KeyUp value) keyUp,
    required TResult Function(KeyDown value) keyDown,
  }) {
    return keyUp(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(KeyUp value)? keyUp,
    TResult Function(KeyDown value)? keyDown,
  }) {
    return keyUp?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KeyUp value)? keyUp,
    TResult Function(KeyDown value)? keyDown,
    required TResult orElse(),
  }) {
    if (keyUp != null) {
      return keyUp(this);
    }
    return orElse();
  }
}

abstract class KeyUp implements KeyboardEvent {
  const factory KeyUp(final KeyboardKey field0) = _$KeyUp;

  @override
  KeyboardKey get field0;
  @override
  @JsonKey(ignore: true)
  _$$KeyUpCopyWith<_$KeyUp> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$KeyDownCopyWith<$Res>
    implements $KeyboardEventCopyWith<$Res> {
  factory _$$KeyDownCopyWith(_$KeyDown value, $Res Function(_$KeyDown) then) =
      __$$KeyDownCopyWithImpl<$Res>;
  @override
  $Res call({KeyboardKey field0});
}

/// @nodoc
class __$$KeyDownCopyWithImpl<$Res> extends _$KeyboardEventCopyWithImpl<$Res>
    implements _$$KeyDownCopyWith<$Res> {
  __$$KeyDownCopyWithImpl(_$KeyDown _value, $Res Function(_$KeyDown) _then)
      : super(_value, (v) => _then(v as _$KeyDown));

  @override
  _$KeyDown get _value => super._value as _$KeyDown;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$KeyDown(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as KeyboardKey,
    ));
  }
}

/// @nodoc

class _$KeyDown implements KeyDown {
  const _$KeyDown(this.field0);

  @override
  final KeyboardKey field0;

  @override
  String toString() {
    return 'KeyboardEvent.keyDown(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$KeyDown &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$KeyDownCopyWith<_$KeyDown> get copyWith =>
      __$$KeyDownCopyWithImpl<_$KeyDown>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(KeyboardKey field0) keyUp,
    required TResult Function(KeyboardKey field0) keyDown,
  }) {
    return keyDown(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(KeyboardKey field0)? keyUp,
    TResult Function(KeyboardKey field0)? keyDown,
  }) {
    return keyDown?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(KeyboardKey field0)? keyUp,
    TResult Function(KeyboardKey field0)? keyDown,
    required TResult orElse(),
  }) {
    if (keyDown != null) {
      return keyDown(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(KeyUp value) keyUp,
    required TResult Function(KeyDown value) keyDown,
  }) {
    return keyDown(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(KeyUp value)? keyUp,
    TResult Function(KeyDown value)? keyDown,
  }) {
    return keyDown?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KeyUp value)? keyUp,
    TResult Function(KeyDown value)? keyDown,
    required TResult orElse(),
  }) {
    if (keyDown != null) {
      return keyDown(this);
    }
    return orElse();
  }
}

abstract class KeyDown implements KeyboardEvent {
  const factory KeyDown(final KeyboardKey field0) = _$KeyDown;

  @override
  KeyboardKey get field0;
  @override
  @JsonKey(ignore: true)
  _$$KeyDownCopyWith<_$KeyDown> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$MouseEvent {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(MouseKey field0, double field1, double field2)
        mouseUp,
    required TResult Function(MouseKey field0, double field1, double field2)
        mouseDown,
    required TResult Function(MouseKey field0, double field1, double field2)
        mouseMove,
    required TResult Function(double field0) mouseScrollWheel,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(MouseKey field0, double field1, double field2)? mouseUp,
    TResult Function(MouseKey field0, double field1, double field2)? mouseDown,
    TResult Function(MouseKey field0, double field1, double field2)? mouseMove,
    TResult Function(double field0)? mouseScrollWheel,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(MouseKey field0, double field1, double field2)? mouseUp,
    TResult Function(MouseKey field0, double field1, double field2)? mouseDown,
    TResult Function(MouseKey field0, double field1, double field2)? mouseMove,
    TResult Function(double field0)? mouseScrollWheel,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(MouseUp value) mouseUp,
    required TResult Function(MouseDown value) mouseDown,
    required TResult Function(MouseMove value) mouseMove,
    required TResult Function(MouseScrollWheel value) mouseScrollWheel,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(MouseUp value)? mouseUp,
    TResult Function(MouseDown value)? mouseDown,
    TResult Function(MouseMove value)? mouseMove,
    TResult Function(MouseScrollWheel value)? mouseScrollWheel,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MouseUp value)? mouseUp,
    TResult Function(MouseDown value)? mouseDown,
    TResult Function(MouseMove value)? mouseMove,
    TResult Function(MouseScrollWheel value)? mouseScrollWheel,
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
abstract class _$$MouseUpCopyWith<$Res> {
  factory _$$MouseUpCopyWith(_$MouseUp value, $Res Function(_$MouseUp) then) =
      __$$MouseUpCopyWithImpl<$Res>;
  $Res call({MouseKey field0, double field1, double field2});
}

/// @nodoc
class __$$MouseUpCopyWithImpl<$Res> extends _$MouseEventCopyWithImpl<$Res>
    implements _$$MouseUpCopyWith<$Res> {
  __$$MouseUpCopyWithImpl(_$MouseUp _value, $Res Function(_$MouseUp) _then)
      : super(_value, (v) => _then(v as _$MouseUp));

  @override
  _$MouseUp get _value => super._value as _$MouseUp;

  @override
  $Res call({
    Object? field0 = freezed,
    Object? field1 = freezed,
    Object? field2 = freezed,
  }) {
    return _then(_$MouseUp(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as MouseKey,
      field1 == freezed
          ? _value.field1
          : field1 // ignore: cast_nullable_to_non_nullable
              as double,
      field2 == freezed
          ? _value.field2
          : field2 // ignore: cast_nullable_to_non_nullable
              as double,
    ));
  }
}

/// @nodoc

class _$MouseUp implements MouseUp {
  const _$MouseUp(this.field0, this.field1, this.field2);

  @override
  final MouseKey field0;
  @override
  final double field1;
  @override
  final double field2;

  @override
  String toString() {
    return 'MouseEvent.mouseUp(field0: $field0, field1: $field1, field2: $field2)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$MouseUp &&
            const DeepCollectionEquality().equals(other.field0, field0) &&
            const DeepCollectionEquality().equals(other.field1, field1) &&
            const DeepCollectionEquality().equals(other.field2, field2));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType,
      const DeepCollectionEquality().hash(field0),
      const DeepCollectionEquality().hash(field1),
      const DeepCollectionEquality().hash(field2));

  @JsonKey(ignore: true)
  @override
  _$$MouseUpCopyWith<_$MouseUp> get copyWith =>
      __$$MouseUpCopyWithImpl<_$MouseUp>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(MouseKey field0, double field1, double field2)
        mouseUp,
    required TResult Function(MouseKey field0, double field1, double field2)
        mouseDown,
    required TResult Function(MouseKey field0, double field1, double field2)
        mouseMove,
    required TResult Function(double field0) mouseScrollWheel,
  }) {
    return mouseUp(field0, field1, field2);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(MouseKey field0, double field1, double field2)? mouseUp,
    TResult Function(MouseKey field0, double field1, double field2)? mouseDown,
    TResult Function(MouseKey field0, double field1, double field2)? mouseMove,
    TResult Function(double field0)? mouseScrollWheel,
  }) {
    return mouseUp?.call(field0, field1, field2);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(MouseKey field0, double field1, double field2)? mouseUp,
    TResult Function(MouseKey field0, double field1, double field2)? mouseDown,
    TResult Function(MouseKey field0, double field1, double field2)? mouseMove,
    TResult Function(double field0)? mouseScrollWheel,
    required TResult orElse(),
  }) {
    if (mouseUp != null) {
      return mouseUp(field0, field1, field2);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(MouseUp value) mouseUp,
    required TResult Function(MouseDown value) mouseDown,
    required TResult Function(MouseMove value) mouseMove,
    required TResult Function(MouseScrollWheel value) mouseScrollWheel,
  }) {
    return mouseUp(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(MouseUp value)? mouseUp,
    TResult Function(MouseDown value)? mouseDown,
    TResult Function(MouseMove value)? mouseMove,
    TResult Function(MouseScrollWheel value)? mouseScrollWheel,
  }) {
    return mouseUp?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MouseUp value)? mouseUp,
    TResult Function(MouseDown value)? mouseDown,
    TResult Function(MouseMove value)? mouseMove,
    TResult Function(MouseScrollWheel value)? mouseScrollWheel,
    required TResult orElse(),
  }) {
    if (mouseUp != null) {
      return mouseUp(this);
    }
    return orElse();
  }
}

abstract class MouseUp implements MouseEvent {
  const factory MouseUp(
          final MouseKey field0, final double field1, final double field2) =
      _$MouseUp;

  MouseKey get field0;
  double get field1;
  double get field2;
  @JsonKey(ignore: true)
  _$$MouseUpCopyWith<_$MouseUp> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$MouseDownCopyWith<$Res> {
  factory _$$MouseDownCopyWith(
          _$MouseDown value, $Res Function(_$MouseDown) then) =
      __$$MouseDownCopyWithImpl<$Res>;
  $Res call({MouseKey field0, double field1, double field2});
}

/// @nodoc
class __$$MouseDownCopyWithImpl<$Res> extends _$MouseEventCopyWithImpl<$Res>
    implements _$$MouseDownCopyWith<$Res> {
  __$$MouseDownCopyWithImpl(
      _$MouseDown _value, $Res Function(_$MouseDown) _then)
      : super(_value, (v) => _then(v as _$MouseDown));

  @override
  _$MouseDown get _value => super._value as _$MouseDown;

  @override
  $Res call({
    Object? field0 = freezed,
    Object? field1 = freezed,
    Object? field2 = freezed,
  }) {
    return _then(_$MouseDown(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as MouseKey,
      field1 == freezed
          ? _value.field1
          : field1 // ignore: cast_nullable_to_non_nullable
              as double,
      field2 == freezed
          ? _value.field2
          : field2 // ignore: cast_nullable_to_non_nullable
              as double,
    ));
  }
}

/// @nodoc

class _$MouseDown implements MouseDown {
  const _$MouseDown(this.field0, this.field1, this.field2);

  @override
  final MouseKey field0;
  @override
  final double field1;
  @override
  final double field2;

  @override
  String toString() {
    return 'MouseEvent.mouseDown(field0: $field0, field1: $field1, field2: $field2)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$MouseDown &&
            const DeepCollectionEquality().equals(other.field0, field0) &&
            const DeepCollectionEquality().equals(other.field1, field1) &&
            const DeepCollectionEquality().equals(other.field2, field2));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType,
      const DeepCollectionEquality().hash(field0),
      const DeepCollectionEquality().hash(field1),
      const DeepCollectionEquality().hash(field2));

  @JsonKey(ignore: true)
  @override
  _$$MouseDownCopyWith<_$MouseDown> get copyWith =>
      __$$MouseDownCopyWithImpl<_$MouseDown>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(MouseKey field0, double field1, double field2)
        mouseUp,
    required TResult Function(MouseKey field0, double field1, double field2)
        mouseDown,
    required TResult Function(MouseKey field0, double field1, double field2)
        mouseMove,
    required TResult Function(double field0) mouseScrollWheel,
  }) {
    return mouseDown(field0, field1, field2);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(MouseKey field0, double field1, double field2)? mouseUp,
    TResult Function(MouseKey field0, double field1, double field2)? mouseDown,
    TResult Function(MouseKey field0, double field1, double field2)? mouseMove,
    TResult Function(double field0)? mouseScrollWheel,
  }) {
    return mouseDown?.call(field0, field1, field2);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(MouseKey field0, double field1, double field2)? mouseUp,
    TResult Function(MouseKey field0, double field1, double field2)? mouseDown,
    TResult Function(MouseKey field0, double field1, double field2)? mouseMove,
    TResult Function(double field0)? mouseScrollWheel,
    required TResult orElse(),
  }) {
    if (mouseDown != null) {
      return mouseDown(field0, field1, field2);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(MouseUp value) mouseUp,
    required TResult Function(MouseDown value) mouseDown,
    required TResult Function(MouseMove value) mouseMove,
    required TResult Function(MouseScrollWheel value) mouseScrollWheel,
  }) {
    return mouseDown(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(MouseUp value)? mouseUp,
    TResult Function(MouseDown value)? mouseDown,
    TResult Function(MouseMove value)? mouseMove,
    TResult Function(MouseScrollWheel value)? mouseScrollWheel,
  }) {
    return mouseDown?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MouseUp value)? mouseUp,
    TResult Function(MouseDown value)? mouseDown,
    TResult Function(MouseMove value)? mouseMove,
    TResult Function(MouseScrollWheel value)? mouseScrollWheel,
    required TResult orElse(),
  }) {
    if (mouseDown != null) {
      return mouseDown(this);
    }
    return orElse();
  }
}

abstract class MouseDown implements MouseEvent {
  const factory MouseDown(
          final MouseKey field0, final double field1, final double field2) =
      _$MouseDown;

  MouseKey get field0;
  double get field1;
  double get field2;
  @JsonKey(ignore: true)
  _$$MouseDownCopyWith<_$MouseDown> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$MouseMoveCopyWith<$Res> {
  factory _$$MouseMoveCopyWith(
          _$MouseMove value, $Res Function(_$MouseMove) then) =
      __$$MouseMoveCopyWithImpl<$Res>;
  $Res call({MouseKey field0, double field1, double field2});
}

/// @nodoc
class __$$MouseMoveCopyWithImpl<$Res> extends _$MouseEventCopyWithImpl<$Res>
    implements _$$MouseMoveCopyWith<$Res> {
  __$$MouseMoveCopyWithImpl(
      _$MouseMove _value, $Res Function(_$MouseMove) _then)
      : super(_value, (v) => _then(v as _$MouseMove));

  @override
  _$MouseMove get _value => super._value as _$MouseMove;

  @override
  $Res call({
    Object? field0 = freezed,
    Object? field1 = freezed,
    Object? field2 = freezed,
  }) {
    return _then(_$MouseMove(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as MouseKey,
      field1 == freezed
          ? _value.field1
          : field1 // ignore: cast_nullable_to_non_nullable
              as double,
      field2 == freezed
          ? _value.field2
          : field2 // ignore: cast_nullable_to_non_nullable
              as double,
    ));
  }
}

/// @nodoc

class _$MouseMove implements MouseMove {
  const _$MouseMove(this.field0, this.field1, this.field2);

  @override
  final MouseKey field0;
  @override
  final double field1;
  @override
  final double field2;

  @override
  String toString() {
    return 'MouseEvent.mouseMove(field0: $field0, field1: $field1, field2: $field2)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$MouseMove &&
            const DeepCollectionEquality().equals(other.field0, field0) &&
            const DeepCollectionEquality().equals(other.field1, field1) &&
            const DeepCollectionEquality().equals(other.field2, field2));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType,
      const DeepCollectionEquality().hash(field0),
      const DeepCollectionEquality().hash(field1),
      const DeepCollectionEquality().hash(field2));

  @JsonKey(ignore: true)
  @override
  _$$MouseMoveCopyWith<_$MouseMove> get copyWith =>
      __$$MouseMoveCopyWithImpl<_$MouseMove>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(MouseKey field0, double field1, double field2)
        mouseUp,
    required TResult Function(MouseKey field0, double field1, double field2)
        mouseDown,
    required TResult Function(MouseKey field0, double field1, double field2)
        mouseMove,
    required TResult Function(double field0) mouseScrollWheel,
  }) {
    return mouseMove(field0, field1, field2);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(MouseKey field0, double field1, double field2)? mouseUp,
    TResult Function(MouseKey field0, double field1, double field2)? mouseDown,
    TResult Function(MouseKey field0, double field1, double field2)? mouseMove,
    TResult Function(double field0)? mouseScrollWheel,
  }) {
    return mouseMove?.call(field0, field1, field2);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(MouseKey field0, double field1, double field2)? mouseUp,
    TResult Function(MouseKey field0, double field1, double field2)? mouseDown,
    TResult Function(MouseKey field0, double field1, double field2)? mouseMove,
    TResult Function(double field0)? mouseScrollWheel,
    required TResult orElse(),
  }) {
    if (mouseMove != null) {
      return mouseMove(field0, field1, field2);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(MouseUp value) mouseUp,
    required TResult Function(MouseDown value) mouseDown,
    required TResult Function(MouseMove value) mouseMove,
    required TResult Function(MouseScrollWheel value) mouseScrollWheel,
  }) {
    return mouseMove(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(MouseUp value)? mouseUp,
    TResult Function(MouseDown value)? mouseDown,
    TResult Function(MouseMove value)? mouseMove,
    TResult Function(MouseScrollWheel value)? mouseScrollWheel,
  }) {
    return mouseMove?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MouseUp value)? mouseUp,
    TResult Function(MouseDown value)? mouseDown,
    TResult Function(MouseMove value)? mouseMove,
    TResult Function(MouseScrollWheel value)? mouseScrollWheel,
    required TResult orElse(),
  }) {
    if (mouseMove != null) {
      return mouseMove(this);
    }
    return orElse();
  }
}

abstract class MouseMove implements MouseEvent {
  const factory MouseMove(
          final MouseKey field0, final double field1, final double field2) =
      _$MouseMove;

  MouseKey get field0;
  double get field1;
  double get field2;
  @JsonKey(ignore: true)
  _$$MouseMoveCopyWith<_$MouseMove> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$MouseScrollWheelCopyWith<$Res> {
  factory _$$MouseScrollWheelCopyWith(
          _$MouseScrollWheel value, $Res Function(_$MouseScrollWheel) then) =
      __$$MouseScrollWheelCopyWithImpl<$Res>;
  $Res call({double field0});
}

/// @nodoc
class __$$MouseScrollWheelCopyWithImpl<$Res>
    extends _$MouseEventCopyWithImpl<$Res>
    implements _$$MouseScrollWheelCopyWith<$Res> {
  __$$MouseScrollWheelCopyWithImpl(
      _$MouseScrollWheel _value, $Res Function(_$MouseScrollWheel) _then)
      : super(_value, (v) => _then(v as _$MouseScrollWheel));

  @override
  _$MouseScrollWheel get _value => super._value as _$MouseScrollWheel;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$MouseScrollWheel(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as double,
    ));
  }
}

/// @nodoc

class _$MouseScrollWheel implements MouseScrollWheel {
  const _$MouseScrollWheel(this.field0);

  @override
  final double field0;

  @override
  String toString() {
    return 'MouseEvent.mouseScrollWheel(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$MouseScrollWheel &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$MouseScrollWheelCopyWith<_$MouseScrollWheel> get copyWith =>
      __$$MouseScrollWheelCopyWithImpl<_$MouseScrollWheel>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(MouseKey field0, double field1, double field2)
        mouseUp,
    required TResult Function(MouseKey field0, double field1, double field2)
        mouseDown,
    required TResult Function(MouseKey field0, double field1, double field2)
        mouseMove,
    required TResult Function(double field0) mouseScrollWheel,
  }) {
    return mouseScrollWheel(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(MouseKey field0, double field1, double field2)? mouseUp,
    TResult Function(MouseKey field0, double field1, double field2)? mouseDown,
    TResult Function(MouseKey field0, double field1, double field2)? mouseMove,
    TResult Function(double field0)? mouseScrollWheel,
  }) {
    return mouseScrollWheel?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(MouseKey field0, double field1, double field2)? mouseUp,
    TResult Function(MouseKey field0, double field1, double field2)? mouseDown,
    TResult Function(MouseKey field0, double field1, double field2)? mouseMove,
    TResult Function(double field0)? mouseScrollWheel,
    required TResult orElse(),
  }) {
    if (mouseScrollWheel != null) {
      return mouseScrollWheel(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(MouseUp value) mouseUp,
    required TResult Function(MouseDown value) mouseDown,
    required TResult Function(MouseMove value) mouseMove,
    required TResult Function(MouseScrollWheel value) mouseScrollWheel,
  }) {
    return mouseScrollWheel(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(MouseUp value)? mouseUp,
    TResult Function(MouseDown value)? mouseDown,
    TResult Function(MouseMove value)? mouseMove,
    TResult Function(MouseScrollWheel value)? mouseScrollWheel,
  }) {
    return mouseScrollWheel?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MouseUp value)? mouseUp,
    TResult Function(MouseDown value)? mouseDown,
    TResult Function(MouseMove value)? mouseMove,
    TResult Function(MouseScrollWheel value)? mouseScrollWheel,
    required TResult orElse(),
  }) {
    if (mouseScrollWheel != null) {
      return mouseScrollWheel(this);
    }
    return orElse();
  }
}

abstract class MouseScrollWheel implements MouseEvent {
  const factory MouseScrollWheel(final double field0) = _$MouseScrollWheel;

  double get field0;
  @JsonKey(ignore: true)
  _$$MouseScrollWheelCopyWith<_$MouseScrollWheel> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$OperatingSystemType {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() windows,
    required TResult Function() macOs,
    required TResult Function() iOs,
    required TResult Function() android,
    required TResult Function(LinuxType field0) linux,
    required TResult Function() unknown,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? windows,
    TResult Function()? macOs,
    TResult Function()? iOs,
    TResult Function()? android,
    TResult Function(LinuxType field0)? linux,
    TResult Function()? unknown,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? windows,
    TResult Function()? macOs,
    TResult Function()? iOs,
    TResult Function()? android,
    TResult Function(LinuxType field0)? linux,
    TResult Function()? unknown,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Windows value) windows,
    required TResult Function(macOS value) macOs,
    required TResult Function(iOS value) iOs,
    required TResult Function(Android value) android,
    required TResult Function(Linux value) linux,
    required TResult Function(Unknown value) unknown,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Windows value)? windows,
    TResult Function(macOS value)? macOs,
    TResult Function(iOS value)? iOs,
    TResult Function(Android value)? android,
    TResult Function(Linux value)? linux,
    TResult Function(Unknown value)? unknown,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Windows value)? windows,
    TResult Function(macOS value)? macOs,
    TResult Function(iOS value)? iOs,
    TResult Function(Android value)? android,
    TResult Function(Linux value)? linux,
    TResult Function(Unknown value)? unknown,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $OperatingSystemTypeCopyWith<$Res> {
  factory $OperatingSystemTypeCopyWith(
          OperatingSystemType value, $Res Function(OperatingSystemType) then) =
      _$OperatingSystemTypeCopyWithImpl<$Res>;
}

/// @nodoc
class _$OperatingSystemTypeCopyWithImpl<$Res>
    implements $OperatingSystemTypeCopyWith<$Res> {
  _$OperatingSystemTypeCopyWithImpl(this._value, this._then);

  final OperatingSystemType _value;
  // ignore: unused_field
  final $Res Function(OperatingSystemType) _then;
}

/// @nodoc
abstract class _$$WindowsCopyWith<$Res> {
  factory _$$WindowsCopyWith(_$Windows value, $Res Function(_$Windows) then) =
      __$$WindowsCopyWithImpl<$Res>;
}

/// @nodoc
class __$$WindowsCopyWithImpl<$Res>
    extends _$OperatingSystemTypeCopyWithImpl<$Res>
    implements _$$WindowsCopyWith<$Res> {
  __$$WindowsCopyWithImpl(_$Windows _value, $Res Function(_$Windows) _then)
      : super(_value, (v) => _then(v as _$Windows));

  @override
  _$Windows get _value => super._value as _$Windows;
}

/// @nodoc

class _$Windows implements Windows {
  const _$Windows();

  @override
  String toString() {
    return 'OperatingSystemType.windows()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$Windows);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() windows,
    required TResult Function() macOs,
    required TResult Function() iOs,
    required TResult Function() android,
    required TResult Function(LinuxType field0) linux,
    required TResult Function() unknown,
  }) {
    return windows();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? windows,
    TResult Function()? macOs,
    TResult Function()? iOs,
    TResult Function()? android,
    TResult Function(LinuxType field0)? linux,
    TResult Function()? unknown,
  }) {
    return windows?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? windows,
    TResult Function()? macOs,
    TResult Function()? iOs,
    TResult Function()? android,
    TResult Function(LinuxType field0)? linux,
    TResult Function()? unknown,
    required TResult orElse(),
  }) {
    if (windows != null) {
      return windows();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Windows value) windows,
    required TResult Function(macOS value) macOs,
    required TResult Function(iOS value) iOs,
    required TResult Function(Android value) android,
    required TResult Function(Linux value) linux,
    required TResult Function(Unknown value) unknown,
  }) {
    return windows(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Windows value)? windows,
    TResult Function(macOS value)? macOs,
    TResult Function(iOS value)? iOs,
    TResult Function(Android value)? android,
    TResult Function(Linux value)? linux,
    TResult Function(Unknown value)? unknown,
  }) {
    return windows?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Windows value)? windows,
    TResult Function(macOS value)? macOs,
    TResult Function(iOS value)? iOs,
    TResult Function(Android value)? android,
    TResult Function(Linux value)? linux,
    TResult Function(Unknown value)? unknown,
    required TResult orElse(),
  }) {
    if (windows != null) {
      return windows(this);
    }
    return orElse();
  }
}

abstract class Windows implements OperatingSystemType {
  const factory Windows() = _$Windows;
}

/// @nodoc
abstract class _$$macOSCopyWith<$Res> {
  factory _$$macOSCopyWith(_$macOS value, $Res Function(_$macOS) then) =
      __$$macOSCopyWithImpl<$Res>;
}

/// @nodoc
class __$$macOSCopyWithImpl<$Res>
    extends _$OperatingSystemTypeCopyWithImpl<$Res>
    implements _$$macOSCopyWith<$Res> {
  __$$macOSCopyWithImpl(_$macOS _value, $Res Function(_$macOS) _then)
      : super(_value, (v) => _then(v as _$macOS));

  @override
  _$macOS get _value => super._value as _$macOS;
}

/// @nodoc

class _$macOS implements macOS {
  const _$macOS();

  @override
  String toString() {
    return 'OperatingSystemType.macOs()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$macOS);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() windows,
    required TResult Function() macOs,
    required TResult Function() iOs,
    required TResult Function() android,
    required TResult Function(LinuxType field0) linux,
    required TResult Function() unknown,
  }) {
    return macOs();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? windows,
    TResult Function()? macOs,
    TResult Function()? iOs,
    TResult Function()? android,
    TResult Function(LinuxType field0)? linux,
    TResult Function()? unknown,
  }) {
    return macOs?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? windows,
    TResult Function()? macOs,
    TResult Function()? iOs,
    TResult Function()? android,
    TResult Function(LinuxType field0)? linux,
    TResult Function()? unknown,
    required TResult orElse(),
  }) {
    if (macOs != null) {
      return macOs();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Windows value) windows,
    required TResult Function(macOS value) macOs,
    required TResult Function(iOS value) iOs,
    required TResult Function(Android value) android,
    required TResult Function(Linux value) linux,
    required TResult Function(Unknown value) unknown,
  }) {
    return macOs(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Windows value)? windows,
    TResult Function(macOS value)? macOs,
    TResult Function(iOS value)? iOs,
    TResult Function(Android value)? android,
    TResult Function(Linux value)? linux,
    TResult Function(Unknown value)? unknown,
  }) {
    return macOs?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Windows value)? windows,
    TResult Function(macOS value)? macOs,
    TResult Function(iOS value)? iOs,
    TResult Function(Android value)? android,
    TResult Function(Linux value)? linux,
    TResult Function(Unknown value)? unknown,
    required TResult orElse(),
  }) {
    if (macOs != null) {
      return macOs(this);
    }
    return orElse();
  }
}

abstract class macOS implements OperatingSystemType {
  const factory macOS() = _$macOS;
}

/// @nodoc
abstract class _$$iOSCopyWith<$Res> {
  factory _$$iOSCopyWith(_$iOS value, $Res Function(_$iOS) then) =
      __$$iOSCopyWithImpl<$Res>;
}

/// @nodoc
class __$$iOSCopyWithImpl<$Res> extends _$OperatingSystemTypeCopyWithImpl<$Res>
    implements _$$iOSCopyWith<$Res> {
  __$$iOSCopyWithImpl(_$iOS _value, $Res Function(_$iOS) _then)
      : super(_value, (v) => _then(v as _$iOS));

  @override
  _$iOS get _value => super._value as _$iOS;
}

/// @nodoc

class _$iOS implements iOS {
  const _$iOS();

  @override
  String toString() {
    return 'OperatingSystemType.iOs()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$iOS);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() windows,
    required TResult Function() macOs,
    required TResult Function() iOs,
    required TResult Function() android,
    required TResult Function(LinuxType field0) linux,
    required TResult Function() unknown,
  }) {
    return iOs();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? windows,
    TResult Function()? macOs,
    TResult Function()? iOs,
    TResult Function()? android,
    TResult Function(LinuxType field0)? linux,
    TResult Function()? unknown,
  }) {
    return iOs?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? windows,
    TResult Function()? macOs,
    TResult Function()? iOs,
    TResult Function()? android,
    TResult Function(LinuxType field0)? linux,
    TResult Function()? unknown,
    required TResult orElse(),
  }) {
    if (iOs != null) {
      return iOs();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Windows value) windows,
    required TResult Function(macOS value) macOs,
    required TResult Function(iOS value) iOs,
    required TResult Function(Android value) android,
    required TResult Function(Linux value) linux,
    required TResult Function(Unknown value) unknown,
  }) {
    return iOs(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Windows value)? windows,
    TResult Function(macOS value)? macOs,
    TResult Function(iOS value)? iOs,
    TResult Function(Android value)? android,
    TResult Function(Linux value)? linux,
    TResult Function(Unknown value)? unknown,
  }) {
    return iOs?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Windows value)? windows,
    TResult Function(macOS value)? macOs,
    TResult Function(iOS value)? iOs,
    TResult Function(Android value)? android,
    TResult Function(Linux value)? linux,
    TResult Function(Unknown value)? unknown,
    required TResult orElse(),
  }) {
    if (iOs != null) {
      return iOs(this);
    }
    return orElse();
  }
}

abstract class iOS implements OperatingSystemType {
  const factory iOS() = _$iOS;
}

/// @nodoc
abstract class _$$AndroidCopyWith<$Res> {
  factory _$$AndroidCopyWith(_$Android value, $Res Function(_$Android) then) =
      __$$AndroidCopyWithImpl<$Res>;
}

/// @nodoc
class __$$AndroidCopyWithImpl<$Res>
    extends _$OperatingSystemTypeCopyWithImpl<$Res>
    implements _$$AndroidCopyWith<$Res> {
  __$$AndroidCopyWithImpl(_$Android _value, $Res Function(_$Android) _then)
      : super(_value, (v) => _then(v as _$Android));

  @override
  _$Android get _value => super._value as _$Android;
}

/// @nodoc

class _$Android implements Android {
  const _$Android();

  @override
  String toString() {
    return 'OperatingSystemType.android()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$Android);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() windows,
    required TResult Function() macOs,
    required TResult Function() iOs,
    required TResult Function() android,
    required TResult Function(LinuxType field0) linux,
    required TResult Function() unknown,
  }) {
    return android();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? windows,
    TResult Function()? macOs,
    TResult Function()? iOs,
    TResult Function()? android,
    TResult Function(LinuxType field0)? linux,
    TResult Function()? unknown,
  }) {
    return android?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? windows,
    TResult Function()? macOs,
    TResult Function()? iOs,
    TResult Function()? android,
    TResult Function(LinuxType field0)? linux,
    TResult Function()? unknown,
    required TResult orElse(),
  }) {
    if (android != null) {
      return android();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Windows value) windows,
    required TResult Function(macOS value) macOs,
    required TResult Function(iOS value) iOs,
    required TResult Function(Android value) android,
    required TResult Function(Linux value) linux,
    required TResult Function(Unknown value) unknown,
  }) {
    return android(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Windows value)? windows,
    TResult Function(macOS value)? macOs,
    TResult Function(iOS value)? iOs,
    TResult Function(Android value)? android,
    TResult Function(Linux value)? linux,
    TResult Function(Unknown value)? unknown,
  }) {
    return android?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Windows value)? windows,
    TResult Function(macOS value)? macOs,
    TResult Function(iOS value)? iOs,
    TResult Function(Android value)? android,
    TResult Function(Linux value)? linux,
    TResult Function(Unknown value)? unknown,
    required TResult orElse(),
  }) {
    if (android != null) {
      return android(this);
    }
    return orElse();
  }
}

abstract class Android implements OperatingSystemType {
  const factory Android() = _$Android;
}

/// @nodoc
abstract class _$$LinuxCopyWith<$Res> {
  factory _$$LinuxCopyWith(_$Linux value, $Res Function(_$Linux) then) =
      __$$LinuxCopyWithImpl<$Res>;
  $Res call({LinuxType field0});
}

/// @nodoc
class __$$LinuxCopyWithImpl<$Res>
    extends _$OperatingSystemTypeCopyWithImpl<$Res>
    implements _$$LinuxCopyWith<$Res> {
  __$$LinuxCopyWithImpl(_$Linux _value, $Res Function(_$Linux) _then)
      : super(_value, (v) => _then(v as _$Linux));

  @override
  _$Linux get _value => super._value as _$Linux;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$Linux(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as LinuxType,
    ));
  }
}

/// @nodoc

class _$Linux implements Linux {
  const _$Linux(this.field0);

  @override
  final LinuxType field0;

  @override
  String toString() {
    return 'OperatingSystemType.linux(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Linux &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$LinuxCopyWith<_$Linux> get copyWith =>
      __$$LinuxCopyWithImpl<_$Linux>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() windows,
    required TResult Function() macOs,
    required TResult Function() iOs,
    required TResult Function() android,
    required TResult Function(LinuxType field0) linux,
    required TResult Function() unknown,
  }) {
    return linux(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? windows,
    TResult Function()? macOs,
    TResult Function()? iOs,
    TResult Function()? android,
    TResult Function(LinuxType field0)? linux,
    TResult Function()? unknown,
  }) {
    return linux?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? windows,
    TResult Function()? macOs,
    TResult Function()? iOs,
    TResult Function()? android,
    TResult Function(LinuxType field0)? linux,
    TResult Function()? unknown,
    required TResult orElse(),
  }) {
    if (linux != null) {
      return linux(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Windows value) windows,
    required TResult Function(macOS value) macOs,
    required TResult Function(iOS value) iOs,
    required TResult Function(Android value) android,
    required TResult Function(Linux value) linux,
    required TResult Function(Unknown value) unknown,
  }) {
    return linux(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Windows value)? windows,
    TResult Function(macOS value)? macOs,
    TResult Function(iOS value)? iOs,
    TResult Function(Android value)? android,
    TResult Function(Linux value)? linux,
    TResult Function(Unknown value)? unknown,
  }) {
    return linux?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Windows value)? windows,
    TResult Function(macOS value)? macOs,
    TResult Function(iOS value)? iOs,
    TResult Function(Android value)? android,
    TResult Function(Linux value)? linux,
    TResult Function(Unknown value)? unknown,
    required TResult orElse(),
  }) {
    if (linux != null) {
      return linux(this);
    }
    return orElse();
  }
}

abstract class Linux implements OperatingSystemType {
  const factory Linux(final LinuxType field0) = _$Linux;

  LinuxType get field0;
  @JsonKey(ignore: true)
  _$$LinuxCopyWith<_$Linux> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$UnknownCopyWith<$Res> {
  factory _$$UnknownCopyWith(_$Unknown value, $Res Function(_$Unknown) then) =
      __$$UnknownCopyWithImpl<$Res>;
}

/// @nodoc
class __$$UnknownCopyWithImpl<$Res>
    extends _$OperatingSystemTypeCopyWithImpl<$Res>
    implements _$$UnknownCopyWith<$Res> {
  __$$UnknownCopyWithImpl(_$Unknown _value, $Res Function(_$Unknown) _then)
      : super(_value, (v) => _then(v as _$Unknown));

  @override
  _$Unknown get _value => super._value as _$Unknown;
}

/// @nodoc

class _$Unknown implements Unknown {
  const _$Unknown();

  @override
  String toString() {
    return 'OperatingSystemType.unknown()';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$Unknown);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() windows,
    required TResult Function() macOs,
    required TResult Function() iOs,
    required TResult Function() android,
    required TResult Function(LinuxType field0) linux,
    required TResult Function() unknown,
  }) {
    return unknown();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? windows,
    TResult Function()? macOs,
    TResult Function()? iOs,
    TResult Function()? android,
    TResult Function(LinuxType field0)? linux,
    TResult Function()? unknown,
  }) {
    return unknown?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? windows,
    TResult Function()? macOs,
    TResult Function()? iOs,
    TResult Function()? android,
    TResult Function(LinuxType field0)? linux,
    TResult Function()? unknown,
    required TResult orElse(),
  }) {
    if (unknown != null) {
      return unknown();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Windows value) windows,
    required TResult Function(macOS value) macOs,
    required TResult Function(iOS value) iOs,
    required TResult Function(Android value) android,
    required TResult Function(Linux value) linux,
    required TResult Function(Unknown value) unknown,
  }) {
    return unknown(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Windows value)? windows,
    TResult Function(macOS value)? macOs,
    TResult Function(iOS value)? iOs,
    TResult Function(Android value)? android,
    TResult Function(Linux value)? linux,
    TResult Function(Unknown value)? unknown,
  }) {
    return unknown?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Windows value)? windows,
    TResult Function(macOS value)? macOs,
    TResult Function(iOS value)? iOs,
    TResult Function(Android value)? android,
    TResult Function(Linux value)? linux,
    TResult Function(Unknown value)? unknown,
    required TResult orElse(),
  }) {
    if (unknown != null) {
      return unknown(this);
    }
    return orElse();
  }
}

abstract class Unknown implements OperatingSystemType {
  const factory Unknown() = _$Unknown;
}

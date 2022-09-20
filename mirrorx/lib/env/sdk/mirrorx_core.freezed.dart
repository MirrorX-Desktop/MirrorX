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
mixin _$FlutterMediaMessage {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(DesktopDecodeFrame field0) video,
    required TResult Function(int field0, int field1, Uint8List field2) audio,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(DesktopDecodeFrame field0)? video,
    TResult Function(int field0, int field1, Uint8List field2)? audio,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(DesktopDecodeFrame field0)? video,
    TResult Function(int field0, int field1, Uint8List field2)? audio,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(FlutterMediaMessage_Video value) video,
    required TResult Function(FlutterMediaMessage_Audio value) audio,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(FlutterMediaMessage_Video value)? video,
    TResult Function(FlutterMediaMessage_Audio value)? audio,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(FlutterMediaMessage_Video value)? video,
    TResult Function(FlutterMediaMessage_Audio value)? audio,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $FlutterMediaMessageCopyWith<$Res> {
  factory $FlutterMediaMessageCopyWith(
          FlutterMediaMessage value, $Res Function(FlutterMediaMessage) then) =
      _$FlutterMediaMessageCopyWithImpl<$Res>;
}

/// @nodoc
class _$FlutterMediaMessageCopyWithImpl<$Res>
    implements $FlutterMediaMessageCopyWith<$Res> {
  _$FlutterMediaMessageCopyWithImpl(this._value, this._then);

  final FlutterMediaMessage _value;
  // ignore: unused_field
  final $Res Function(FlutterMediaMessage) _then;
}

/// @nodoc
abstract class _$$FlutterMediaMessage_VideoCopyWith<$Res> {
  factory _$$FlutterMediaMessage_VideoCopyWith(
          _$FlutterMediaMessage_Video value,
          $Res Function(_$FlutterMediaMessage_Video) then) =
      __$$FlutterMediaMessage_VideoCopyWithImpl<$Res>;
  $Res call({DesktopDecodeFrame field0});
}

/// @nodoc
class __$$FlutterMediaMessage_VideoCopyWithImpl<$Res>
    extends _$FlutterMediaMessageCopyWithImpl<$Res>
    implements _$$FlutterMediaMessage_VideoCopyWith<$Res> {
  __$$FlutterMediaMessage_VideoCopyWithImpl(_$FlutterMediaMessage_Video _value,
      $Res Function(_$FlutterMediaMessage_Video) _then)
      : super(_value, (v) => _then(v as _$FlutterMediaMessage_Video));

  @override
  _$FlutterMediaMessage_Video get _value =>
      super._value as _$FlutterMediaMessage_Video;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$FlutterMediaMessage_Video(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as DesktopDecodeFrame,
    ));
  }
}

/// @nodoc

class _$FlutterMediaMessage_Video implements FlutterMediaMessage_Video {
  const _$FlutterMediaMessage_Video(this.field0);

  @override
  final DesktopDecodeFrame field0;

  @override
  String toString() {
    return 'FlutterMediaMessage.video(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$FlutterMediaMessage_Video &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$FlutterMediaMessage_VideoCopyWith<_$FlutterMediaMessage_Video>
      get copyWith => __$$FlutterMediaMessage_VideoCopyWithImpl<
          _$FlutterMediaMessage_Video>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(DesktopDecodeFrame field0) video,
    required TResult Function(int field0, int field1, Uint8List field2) audio,
  }) {
    return video(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(DesktopDecodeFrame field0)? video,
    TResult Function(int field0, int field1, Uint8List field2)? audio,
  }) {
    return video?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(DesktopDecodeFrame field0)? video,
    TResult Function(int field0, int field1, Uint8List field2)? audio,
    required TResult orElse(),
  }) {
    if (video != null) {
      return video(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(FlutterMediaMessage_Video value) video,
    required TResult Function(FlutterMediaMessage_Audio value) audio,
  }) {
    return video(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(FlutterMediaMessage_Video value)? video,
    TResult Function(FlutterMediaMessage_Audio value)? audio,
  }) {
    return video?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(FlutterMediaMessage_Video value)? video,
    TResult Function(FlutterMediaMessage_Audio value)? audio,
    required TResult orElse(),
  }) {
    if (video != null) {
      return video(this);
    }
    return orElse();
  }
}

abstract class FlutterMediaMessage_Video implements FlutterMediaMessage {
  const factory FlutterMediaMessage_Video(final DesktopDecodeFrame field0) =
      _$FlutterMediaMessage_Video;

  DesktopDecodeFrame get field0;
  @JsonKey(ignore: true)
  _$$FlutterMediaMessage_VideoCopyWith<_$FlutterMediaMessage_Video>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$FlutterMediaMessage_AudioCopyWith<$Res> {
  factory _$$FlutterMediaMessage_AudioCopyWith(
          _$FlutterMediaMessage_Audio value,
          $Res Function(_$FlutterMediaMessage_Audio) then) =
      __$$FlutterMediaMessage_AudioCopyWithImpl<$Res>;
  $Res call({int field0, int field1, Uint8List field2});
}

/// @nodoc
class __$$FlutterMediaMessage_AudioCopyWithImpl<$Res>
    extends _$FlutterMediaMessageCopyWithImpl<$Res>
    implements _$$FlutterMediaMessage_AudioCopyWith<$Res> {
  __$$FlutterMediaMessage_AudioCopyWithImpl(_$FlutterMediaMessage_Audio _value,
      $Res Function(_$FlutterMediaMessage_Audio) _then)
      : super(_value, (v) => _then(v as _$FlutterMediaMessage_Audio));

  @override
  _$FlutterMediaMessage_Audio get _value =>
      super._value as _$FlutterMediaMessage_Audio;

  @override
  $Res call({
    Object? field0 = freezed,
    Object? field1 = freezed,
    Object? field2 = freezed,
  }) {
    return _then(_$FlutterMediaMessage_Audio(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as int,
      field1 == freezed
          ? _value.field1
          : field1 // ignore: cast_nullable_to_non_nullable
              as int,
      field2 == freezed
          ? _value.field2
          : field2 // ignore: cast_nullable_to_non_nullable
              as Uint8List,
    ));
  }
}

/// @nodoc

class _$FlutterMediaMessage_Audio implements FlutterMediaMessage_Audio {
  const _$FlutterMediaMessage_Audio(this.field0, this.field1, this.field2);

  @override
  final int field0;
  @override
  final int field1;
  @override
  final Uint8List field2;

  @override
  String toString() {
    return 'FlutterMediaMessage.audio(field0: $field0, field1: $field1, field2: $field2)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$FlutterMediaMessage_Audio &&
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
  _$$FlutterMediaMessage_AudioCopyWith<_$FlutterMediaMessage_Audio>
      get copyWith => __$$FlutterMediaMessage_AudioCopyWithImpl<
          _$FlutterMediaMessage_Audio>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(DesktopDecodeFrame field0) video,
    required TResult Function(int field0, int field1, Uint8List field2) audio,
  }) {
    return audio(field0, field1, field2);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(DesktopDecodeFrame field0)? video,
    TResult Function(int field0, int field1, Uint8List field2)? audio,
  }) {
    return audio?.call(field0, field1, field2);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(DesktopDecodeFrame field0)? video,
    TResult Function(int field0, int field1, Uint8List field2)? audio,
    required TResult orElse(),
  }) {
    if (audio != null) {
      return audio(field0, field1, field2);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(FlutterMediaMessage_Video value) video,
    required TResult Function(FlutterMediaMessage_Audio value) audio,
  }) {
    return audio(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(FlutterMediaMessage_Video value)? video,
    TResult Function(FlutterMediaMessage_Audio value)? audio,
  }) {
    return audio?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(FlutterMediaMessage_Video value)? video,
    TResult Function(FlutterMediaMessage_Audio value)? audio,
    required TResult orElse(),
  }) {
    if (audio != null) {
      return audio(this);
    }
    return orElse();
  }
}

abstract class FlutterMediaMessage_Audio implements FlutterMediaMessage {
  const factory FlutterMediaMessage_Audio(
          final int field0, final int field1, final Uint8List field2) =
      _$FlutterMediaMessage_Audio;

  int get field0;
  int get field1;
  Uint8List get field2;
  @JsonKey(ignore: true)
  _$$FlutterMediaMessage_AudioCopyWith<_$FlutterMediaMessage_Audio>
      get copyWith => throw _privateConstructorUsedError;
}

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
    required TResult Function(InputEvent_Mouse value) mouse,
    required TResult Function(InputEvent_Keyboard value) keyboard,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(InputEvent_Mouse value)? mouse,
    TResult Function(InputEvent_Keyboard value)? keyboard,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(InputEvent_Mouse value)? mouse,
    TResult Function(InputEvent_Keyboard value)? keyboard,
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
abstract class _$$InputEvent_MouseCopyWith<$Res> {
  factory _$$InputEvent_MouseCopyWith(
          _$InputEvent_Mouse value, $Res Function(_$InputEvent_Mouse) then) =
      __$$InputEvent_MouseCopyWithImpl<$Res>;
  $Res call({MouseEvent field0});

  $MouseEventCopyWith<$Res> get field0;
}

/// @nodoc
class __$$InputEvent_MouseCopyWithImpl<$Res>
    extends _$InputEventCopyWithImpl<$Res>
    implements _$$InputEvent_MouseCopyWith<$Res> {
  __$$InputEvent_MouseCopyWithImpl(
      _$InputEvent_Mouse _value, $Res Function(_$InputEvent_Mouse) _then)
      : super(_value, (v) => _then(v as _$InputEvent_Mouse));

  @override
  _$InputEvent_Mouse get _value => super._value as _$InputEvent_Mouse;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$InputEvent_Mouse(
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

class _$InputEvent_Mouse implements InputEvent_Mouse {
  const _$InputEvent_Mouse(this.field0);

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
            other is _$InputEvent_Mouse &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$InputEvent_MouseCopyWith<_$InputEvent_Mouse> get copyWith =>
      __$$InputEvent_MouseCopyWithImpl<_$InputEvent_Mouse>(this, _$identity);

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
    required TResult Function(InputEvent_Mouse value) mouse,
    required TResult Function(InputEvent_Keyboard value) keyboard,
  }) {
    return mouse(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(InputEvent_Mouse value)? mouse,
    TResult Function(InputEvent_Keyboard value)? keyboard,
  }) {
    return mouse?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(InputEvent_Mouse value)? mouse,
    TResult Function(InputEvent_Keyboard value)? keyboard,
    required TResult orElse(),
  }) {
    if (mouse != null) {
      return mouse(this);
    }
    return orElse();
  }
}

abstract class InputEvent_Mouse implements InputEvent {
  const factory InputEvent_Mouse(final MouseEvent field0) = _$InputEvent_Mouse;

  MouseEvent get field0;
  @JsonKey(ignore: true)
  _$$InputEvent_MouseCopyWith<_$InputEvent_Mouse> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$InputEvent_KeyboardCopyWith<$Res> {
  factory _$$InputEvent_KeyboardCopyWith(_$InputEvent_Keyboard value,
          $Res Function(_$InputEvent_Keyboard) then) =
      __$$InputEvent_KeyboardCopyWithImpl<$Res>;
  $Res call({KeyboardEvent field0});

  $KeyboardEventCopyWith<$Res> get field0;
}

/// @nodoc
class __$$InputEvent_KeyboardCopyWithImpl<$Res>
    extends _$InputEventCopyWithImpl<$Res>
    implements _$$InputEvent_KeyboardCopyWith<$Res> {
  __$$InputEvent_KeyboardCopyWithImpl(
      _$InputEvent_Keyboard _value, $Res Function(_$InputEvent_Keyboard) _then)
      : super(_value, (v) => _then(v as _$InputEvent_Keyboard));

  @override
  _$InputEvent_Keyboard get _value => super._value as _$InputEvent_Keyboard;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$InputEvent_Keyboard(
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

class _$InputEvent_Keyboard implements InputEvent_Keyboard {
  const _$InputEvent_Keyboard(this.field0);

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
            other is _$InputEvent_Keyboard &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$InputEvent_KeyboardCopyWith<_$InputEvent_Keyboard> get copyWith =>
      __$$InputEvent_KeyboardCopyWithImpl<_$InputEvent_Keyboard>(
          this, _$identity);

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
    required TResult Function(InputEvent_Mouse value) mouse,
    required TResult Function(InputEvent_Keyboard value) keyboard,
  }) {
    return keyboard(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(InputEvent_Mouse value)? mouse,
    TResult Function(InputEvent_Keyboard value)? keyboard,
  }) {
    return keyboard?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(InputEvent_Mouse value)? mouse,
    TResult Function(InputEvent_Keyboard value)? keyboard,
    required TResult orElse(),
  }) {
    if (keyboard != null) {
      return keyboard(this);
    }
    return orElse();
  }
}

abstract class InputEvent_Keyboard implements InputEvent {
  const factory InputEvent_Keyboard(final KeyboardEvent field0) =
      _$InputEvent_Keyboard;

  KeyboardEvent get field0;
  @JsonKey(ignore: true)
  _$$InputEvent_KeyboardCopyWith<_$InputEvent_Keyboard> get copyWith =>
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
    required TResult Function(KeyboardEvent_KeyUp value) keyUp,
    required TResult Function(KeyboardEvent_KeyDown value) keyDown,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(KeyboardEvent_KeyUp value)? keyUp,
    TResult Function(KeyboardEvent_KeyDown value)? keyDown,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KeyboardEvent_KeyUp value)? keyUp,
    TResult Function(KeyboardEvent_KeyDown value)? keyDown,
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
abstract class _$$KeyboardEvent_KeyUpCopyWith<$Res>
    implements $KeyboardEventCopyWith<$Res> {
  factory _$$KeyboardEvent_KeyUpCopyWith(_$KeyboardEvent_KeyUp value,
          $Res Function(_$KeyboardEvent_KeyUp) then) =
      __$$KeyboardEvent_KeyUpCopyWithImpl<$Res>;
  @override
  $Res call({KeyboardKey field0});
}

/// @nodoc
class __$$KeyboardEvent_KeyUpCopyWithImpl<$Res>
    extends _$KeyboardEventCopyWithImpl<$Res>
    implements _$$KeyboardEvent_KeyUpCopyWith<$Res> {
  __$$KeyboardEvent_KeyUpCopyWithImpl(
      _$KeyboardEvent_KeyUp _value, $Res Function(_$KeyboardEvent_KeyUp) _then)
      : super(_value, (v) => _then(v as _$KeyboardEvent_KeyUp));

  @override
  _$KeyboardEvent_KeyUp get _value => super._value as _$KeyboardEvent_KeyUp;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$KeyboardEvent_KeyUp(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as KeyboardKey,
    ));
  }
}

/// @nodoc

class _$KeyboardEvent_KeyUp implements KeyboardEvent_KeyUp {
  const _$KeyboardEvent_KeyUp(this.field0);

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
            other is _$KeyboardEvent_KeyUp &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$KeyboardEvent_KeyUpCopyWith<_$KeyboardEvent_KeyUp> get copyWith =>
      __$$KeyboardEvent_KeyUpCopyWithImpl<_$KeyboardEvent_KeyUp>(
          this, _$identity);

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
    required TResult Function(KeyboardEvent_KeyUp value) keyUp,
    required TResult Function(KeyboardEvent_KeyDown value) keyDown,
  }) {
    return keyUp(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(KeyboardEvent_KeyUp value)? keyUp,
    TResult Function(KeyboardEvent_KeyDown value)? keyDown,
  }) {
    return keyUp?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KeyboardEvent_KeyUp value)? keyUp,
    TResult Function(KeyboardEvent_KeyDown value)? keyDown,
    required TResult orElse(),
  }) {
    if (keyUp != null) {
      return keyUp(this);
    }
    return orElse();
  }
}

abstract class KeyboardEvent_KeyUp implements KeyboardEvent {
  const factory KeyboardEvent_KeyUp(final KeyboardKey field0) =
      _$KeyboardEvent_KeyUp;

  @override
  KeyboardKey get field0;
  @override
  @JsonKey(ignore: true)
  _$$KeyboardEvent_KeyUpCopyWith<_$KeyboardEvent_KeyUp> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$KeyboardEvent_KeyDownCopyWith<$Res>
    implements $KeyboardEventCopyWith<$Res> {
  factory _$$KeyboardEvent_KeyDownCopyWith(_$KeyboardEvent_KeyDown value,
          $Res Function(_$KeyboardEvent_KeyDown) then) =
      __$$KeyboardEvent_KeyDownCopyWithImpl<$Res>;
  @override
  $Res call({KeyboardKey field0});
}

/// @nodoc
class __$$KeyboardEvent_KeyDownCopyWithImpl<$Res>
    extends _$KeyboardEventCopyWithImpl<$Res>
    implements _$$KeyboardEvent_KeyDownCopyWith<$Res> {
  __$$KeyboardEvent_KeyDownCopyWithImpl(_$KeyboardEvent_KeyDown _value,
      $Res Function(_$KeyboardEvent_KeyDown) _then)
      : super(_value, (v) => _then(v as _$KeyboardEvent_KeyDown));

  @override
  _$KeyboardEvent_KeyDown get _value => super._value as _$KeyboardEvent_KeyDown;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$KeyboardEvent_KeyDown(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as KeyboardKey,
    ));
  }
}

/// @nodoc

class _$KeyboardEvent_KeyDown implements KeyboardEvent_KeyDown {
  const _$KeyboardEvent_KeyDown(this.field0);

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
            other is _$KeyboardEvent_KeyDown &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$KeyboardEvent_KeyDownCopyWith<_$KeyboardEvent_KeyDown> get copyWith =>
      __$$KeyboardEvent_KeyDownCopyWithImpl<_$KeyboardEvent_KeyDown>(
          this, _$identity);

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
    required TResult Function(KeyboardEvent_KeyUp value) keyUp,
    required TResult Function(KeyboardEvent_KeyDown value) keyDown,
  }) {
    return keyDown(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(KeyboardEvent_KeyUp value)? keyUp,
    TResult Function(KeyboardEvent_KeyDown value)? keyDown,
  }) {
    return keyDown?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(KeyboardEvent_KeyUp value)? keyUp,
    TResult Function(KeyboardEvent_KeyDown value)? keyDown,
    required TResult orElse(),
  }) {
    if (keyDown != null) {
      return keyDown(this);
    }
    return orElse();
  }
}

abstract class KeyboardEvent_KeyDown implements KeyboardEvent {
  const factory KeyboardEvent_KeyDown(final KeyboardKey field0) =
      _$KeyboardEvent_KeyDown;

  @override
  KeyboardKey get field0;
  @override
  @JsonKey(ignore: true)
  _$$KeyboardEvent_KeyDownCopyWith<_$KeyboardEvent_KeyDown> get copyWith =>
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
    required TResult Function(MouseEvent_MouseUp value) mouseUp,
    required TResult Function(MouseEvent_MouseDown value) mouseDown,
    required TResult Function(MouseEvent_MouseMove value) mouseMove,
    required TResult Function(MouseEvent_MouseScrollWheel value)
        mouseScrollWheel,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(MouseEvent_MouseUp value)? mouseUp,
    TResult Function(MouseEvent_MouseDown value)? mouseDown,
    TResult Function(MouseEvent_MouseMove value)? mouseMove,
    TResult Function(MouseEvent_MouseScrollWheel value)? mouseScrollWheel,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MouseEvent_MouseUp value)? mouseUp,
    TResult Function(MouseEvent_MouseDown value)? mouseDown,
    TResult Function(MouseEvent_MouseMove value)? mouseMove,
    TResult Function(MouseEvent_MouseScrollWheel value)? mouseScrollWheel,
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
abstract class _$$MouseEvent_MouseUpCopyWith<$Res> {
  factory _$$MouseEvent_MouseUpCopyWith(_$MouseEvent_MouseUp value,
          $Res Function(_$MouseEvent_MouseUp) then) =
      __$$MouseEvent_MouseUpCopyWithImpl<$Res>;
  $Res call({MouseKey field0, double field1, double field2});
}

/// @nodoc
class __$$MouseEvent_MouseUpCopyWithImpl<$Res>
    extends _$MouseEventCopyWithImpl<$Res>
    implements _$$MouseEvent_MouseUpCopyWith<$Res> {
  __$$MouseEvent_MouseUpCopyWithImpl(
      _$MouseEvent_MouseUp _value, $Res Function(_$MouseEvent_MouseUp) _then)
      : super(_value, (v) => _then(v as _$MouseEvent_MouseUp));

  @override
  _$MouseEvent_MouseUp get _value => super._value as _$MouseEvent_MouseUp;

  @override
  $Res call({
    Object? field0 = freezed,
    Object? field1 = freezed,
    Object? field2 = freezed,
  }) {
    return _then(_$MouseEvent_MouseUp(
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

class _$MouseEvent_MouseUp implements MouseEvent_MouseUp {
  const _$MouseEvent_MouseUp(this.field0, this.field1, this.field2);

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
            other is _$MouseEvent_MouseUp &&
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
  _$$MouseEvent_MouseUpCopyWith<_$MouseEvent_MouseUp> get copyWith =>
      __$$MouseEvent_MouseUpCopyWithImpl<_$MouseEvent_MouseUp>(
          this, _$identity);

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
    required TResult Function(MouseEvent_MouseUp value) mouseUp,
    required TResult Function(MouseEvent_MouseDown value) mouseDown,
    required TResult Function(MouseEvent_MouseMove value) mouseMove,
    required TResult Function(MouseEvent_MouseScrollWheel value)
        mouseScrollWheel,
  }) {
    return mouseUp(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(MouseEvent_MouseUp value)? mouseUp,
    TResult Function(MouseEvent_MouseDown value)? mouseDown,
    TResult Function(MouseEvent_MouseMove value)? mouseMove,
    TResult Function(MouseEvent_MouseScrollWheel value)? mouseScrollWheel,
  }) {
    return mouseUp?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MouseEvent_MouseUp value)? mouseUp,
    TResult Function(MouseEvent_MouseDown value)? mouseDown,
    TResult Function(MouseEvent_MouseMove value)? mouseMove,
    TResult Function(MouseEvent_MouseScrollWheel value)? mouseScrollWheel,
    required TResult orElse(),
  }) {
    if (mouseUp != null) {
      return mouseUp(this);
    }
    return orElse();
  }
}

abstract class MouseEvent_MouseUp implements MouseEvent {
  const factory MouseEvent_MouseUp(
          final MouseKey field0, final double field1, final double field2) =
      _$MouseEvent_MouseUp;

  MouseKey get field0;
  double get field1;
  double get field2;
  @JsonKey(ignore: true)
  _$$MouseEvent_MouseUpCopyWith<_$MouseEvent_MouseUp> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$MouseEvent_MouseDownCopyWith<$Res> {
  factory _$$MouseEvent_MouseDownCopyWith(_$MouseEvent_MouseDown value,
          $Res Function(_$MouseEvent_MouseDown) then) =
      __$$MouseEvent_MouseDownCopyWithImpl<$Res>;
  $Res call({MouseKey field0, double field1, double field2});
}

/// @nodoc
class __$$MouseEvent_MouseDownCopyWithImpl<$Res>
    extends _$MouseEventCopyWithImpl<$Res>
    implements _$$MouseEvent_MouseDownCopyWith<$Res> {
  __$$MouseEvent_MouseDownCopyWithImpl(_$MouseEvent_MouseDown _value,
      $Res Function(_$MouseEvent_MouseDown) _then)
      : super(_value, (v) => _then(v as _$MouseEvent_MouseDown));

  @override
  _$MouseEvent_MouseDown get _value => super._value as _$MouseEvent_MouseDown;

  @override
  $Res call({
    Object? field0 = freezed,
    Object? field1 = freezed,
    Object? field2 = freezed,
  }) {
    return _then(_$MouseEvent_MouseDown(
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

class _$MouseEvent_MouseDown implements MouseEvent_MouseDown {
  const _$MouseEvent_MouseDown(this.field0, this.field1, this.field2);

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
            other is _$MouseEvent_MouseDown &&
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
  _$$MouseEvent_MouseDownCopyWith<_$MouseEvent_MouseDown> get copyWith =>
      __$$MouseEvent_MouseDownCopyWithImpl<_$MouseEvent_MouseDown>(
          this, _$identity);

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
    required TResult Function(MouseEvent_MouseUp value) mouseUp,
    required TResult Function(MouseEvent_MouseDown value) mouseDown,
    required TResult Function(MouseEvent_MouseMove value) mouseMove,
    required TResult Function(MouseEvent_MouseScrollWheel value)
        mouseScrollWheel,
  }) {
    return mouseDown(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(MouseEvent_MouseUp value)? mouseUp,
    TResult Function(MouseEvent_MouseDown value)? mouseDown,
    TResult Function(MouseEvent_MouseMove value)? mouseMove,
    TResult Function(MouseEvent_MouseScrollWheel value)? mouseScrollWheel,
  }) {
    return mouseDown?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MouseEvent_MouseUp value)? mouseUp,
    TResult Function(MouseEvent_MouseDown value)? mouseDown,
    TResult Function(MouseEvent_MouseMove value)? mouseMove,
    TResult Function(MouseEvent_MouseScrollWheel value)? mouseScrollWheel,
    required TResult orElse(),
  }) {
    if (mouseDown != null) {
      return mouseDown(this);
    }
    return orElse();
  }
}

abstract class MouseEvent_MouseDown implements MouseEvent {
  const factory MouseEvent_MouseDown(
          final MouseKey field0, final double field1, final double field2) =
      _$MouseEvent_MouseDown;

  MouseKey get field0;
  double get field1;
  double get field2;
  @JsonKey(ignore: true)
  _$$MouseEvent_MouseDownCopyWith<_$MouseEvent_MouseDown> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$MouseEvent_MouseMoveCopyWith<$Res> {
  factory _$$MouseEvent_MouseMoveCopyWith(_$MouseEvent_MouseMove value,
          $Res Function(_$MouseEvent_MouseMove) then) =
      __$$MouseEvent_MouseMoveCopyWithImpl<$Res>;
  $Res call({MouseKey field0, double field1, double field2});
}

/// @nodoc
class __$$MouseEvent_MouseMoveCopyWithImpl<$Res>
    extends _$MouseEventCopyWithImpl<$Res>
    implements _$$MouseEvent_MouseMoveCopyWith<$Res> {
  __$$MouseEvent_MouseMoveCopyWithImpl(_$MouseEvent_MouseMove _value,
      $Res Function(_$MouseEvent_MouseMove) _then)
      : super(_value, (v) => _then(v as _$MouseEvent_MouseMove));

  @override
  _$MouseEvent_MouseMove get _value => super._value as _$MouseEvent_MouseMove;

  @override
  $Res call({
    Object? field0 = freezed,
    Object? field1 = freezed,
    Object? field2 = freezed,
  }) {
    return _then(_$MouseEvent_MouseMove(
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

class _$MouseEvent_MouseMove implements MouseEvent_MouseMove {
  const _$MouseEvent_MouseMove(this.field0, this.field1, this.field2);

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
            other is _$MouseEvent_MouseMove &&
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
  _$$MouseEvent_MouseMoveCopyWith<_$MouseEvent_MouseMove> get copyWith =>
      __$$MouseEvent_MouseMoveCopyWithImpl<_$MouseEvent_MouseMove>(
          this, _$identity);

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
    required TResult Function(MouseEvent_MouseUp value) mouseUp,
    required TResult Function(MouseEvent_MouseDown value) mouseDown,
    required TResult Function(MouseEvent_MouseMove value) mouseMove,
    required TResult Function(MouseEvent_MouseScrollWheel value)
        mouseScrollWheel,
  }) {
    return mouseMove(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(MouseEvent_MouseUp value)? mouseUp,
    TResult Function(MouseEvent_MouseDown value)? mouseDown,
    TResult Function(MouseEvent_MouseMove value)? mouseMove,
    TResult Function(MouseEvent_MouseScrollWheel value)? mouseScrollWheel,
  }) {
    return mouseMove?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MouseEvent_MouseUp value)? mouseUp,
    TResult Function(MouseEvent_MouseDown value)? mouseDown,
    TResult Function(MouseEvent_MouseMove value)? mouseMove,
    TResult Function(MouseEvent_MouseScrollWheel value)? mouseScrollWheel,
    required TResult orElse(),
  }) {
    if (mouseMove != null) {
      return mouseMove(this);
    }
    return orElse();
  }
}

abstract class MouseEvent_MouseMove implements MouseEvent {
  const factory MouseEvent_MouseMove(
          final MouseKey field0, final double field1, final double field2) =
      _$MouseEvent_MouseMove;

  MouseKey get field0;
  double get field1;
  double get field2;
  @JsonKey(ignore: true)
  _$$MouseEvent_MouseMoveCopyWith<_$MouseEvent_MouseMove> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$MouseEvent_MouseScrollWheelCopyWith<$Res> {
  factory _$$MouseEvent_MouseScrollWheelCopyWith(
          _$MouseEvent_MouseScrollWheel value,
          $Res Function(_$MouseEvent_MouseScrollWheel) then) =
      __$$MouseEvent_MouseScrollWheelCopyWithImpl<$Res>;
  $Res call({double field0});
}

/// @nodoc
class __$$MouseEvent_MouseScrollWheelCopyWithImpl<$Res>
    extends _$MouseEventCopyWithImpl<$Res>
    implements _$$MouseEvent_MouseScrollWheelCopyWith<$Res> {
  __$$MouseEvent_MouseScrollWheelCopyWithImpl(
      _$MouseEvent_MouseScrollWheel _value,
      $Res Function(_$MouseEvent_MouseScrollWheel) _then)
      : super(_value, (v) => _then(v as _$MouseEvent_MouseScrollWheel));

  @override
  _$MouseEvent_MouseScrollWheel get _value =>
      super._value as _$MouseEvent_MouseScrollWheel;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$MouseEvent_MouseScrollWheel(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as double,
    ));
  }
}

/// @nodoc

class _$MouseEvent_MouseScrollWheel implements MouseEvent_MouseScrollWheel {
  const _$MouseEvent_MouseScrollWheel(this.field0);

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
            other is _$MouseEvent_MouseScrollWheel &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$MouseEvent_MouseScrollWheelCopyWith<_$MouseEvent_MouseScrollWheel>
      get copyWith => __$$MouseEvent_MouseScrollWheelCopyWithImpl<
          _$MouseEvent_MouseScrollWheel>(this, _$identity);

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
    required TResult Function(MouseEvent_MouseUp value) mouseUp,
    required TResult Function(MouseEvent_MouseDown value) mouseDown,
    required TResult Function(MouseEvent_MouseMove value) mouseMove,
    required TResult Function(MouseEvent_MouseScrollWheel value)
        mouseScrollWheel,
  }) {
    return mouseScrollWheel(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(MouseEvent_MouseUp value)? mouseUp,
    TResult Function(MouseEvent_MouseDown value)? mouseDown,
    TResult Function(MouseEvent_MouseMove value)? mouseMove,
    TResult Function(MouseEvent_MouseScrollWheel value)? mouseScrollWheel,
  }) {
    return mouseScrollWheel?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MouseEvent_MouseUp value)? mouseUp,
    TResult Function(MouseEvent_MouseDown value)? mouseDown,
    TResult Function(MouseEvent_MouseMove value)? mouseMove,
    TResult Function(MouseEvent_MouseScrollWheel value)? mouseScrollWheel,
    required TResult orElse(),
  }) {
    if (mouseScrollWheel != null) {
      return mouseScrollWheel(this);
    }
    return orElse();
  }
}

abstract class MouseEvent_MouseScrollWheel implements MouseEvent {
  const factory MouseEvent_MouseScrollWheel(final double field0) =
      _$MouseEvent_MouseScrollWheel;

  double get field0;
  @JsonKey(ignore: true)
  _$$MouseEvent_MouseScrollWheelCopyWith<_$MouseEvent_MouseScrollWheel>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$PublishMessage {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function() streamClosed,
    required TResult Function(
            int activeDeviceId, int passiveDeviceId, ResourceType resourceType)
        visitRequest,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? streamClosed,
    TResult Function(
            int activeDeviceId, int passiveDeviceId, ResourceType resourceType)?
        visitRequest,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? streamClosed,
    TResult Function(
            int activeDeviceId, int passiveDeviceId, ResourceType resourceType)?
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
    required TResult Function(
            int activeDeviceId, int passiveDeviceId, ResourceType resourceType)
        visitRequest,
  }) {
    return streamClosed();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? streamClosed,
    TResult Function(
            int activeDeviceId, int passiveDeviceId, ResourceType resourceType)?
        visitRequest,
  }) {
    return streamClosed?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? streamClosed,
    TResult Function(
            int activeDeviceId, int passiveDeviceId, ResourceType resourceType)?
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
      {int activeDeviceId, int passiveDeviceId, ResourceType resourceType});
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
              as int,
      passiveDeviceId: passiveDeviceId == freezed
          ? _value.passiveDeviceId
          : passiveDeviceId // ignore: cast_nullable_to_non_nullable
              as int,
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
  final int activeDeviceId;
  @override
  final int passiveDeviceId;
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
    required TResult Function(
            int activeDeviceId, int passiveDeviceId, ResourceType resourceType)
        visitRequest,
  }) {
    return visitRequest(activeDeviceId, passiveDeviceId, resourceType);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function()? streamClosed,
    TResult Function(
            int activeDeviceId, int passiveDeviceId, ResourceType resourceType)?
        visitRequest,
  }) {
    return visitRequest?.call(activeDeviceId, passiveDeviceId, resourceType);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function()? streamClosed,
    TResult Function(
            int activeDeviceId, int passiveDeviceId, ResourceType resourceType)?
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
          {required final int activeDeviceId,
          required final int passiveDeviceId,
          required final ResourceType resourceType}) =
      _$PublishMessage_VisitRequest;

  int get activeDeviceId;
  int get passiveDeviceId;
  ResourceType get resourceType;
  @JsonKey(ignore: true)
  _$$PublishMessage_VisitRequestCopyWith<_$PublishMessage_VisitRequest>
      get copyWith => throw _privateConstructorUsedError;
}

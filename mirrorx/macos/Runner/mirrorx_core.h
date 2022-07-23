#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define BPP 4

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct MouseEvent_MouseUp {
  int32_t field0;
  float field1;
  float field2;
} MouseEvent_MouseUp;

typedef struct MouseEvent_MouseDown {
  int32_t field0;
  float field1;
  float field2;
} MouseEvent_MouseDown;

typedef struct MouseEvent_MouseMove {
  int32_t field0;
  float field1;
  float field2;
} MouseEvent_MouseMove;

typedef struct MouseEvent_MouseScrollWheel {
  float field0;
} MouseEvent_MouseScrollWheel;

typedef union MouseEventKind {
  struct MouseEvent_MouseUp *MouseUp;
  struct MouseEvent_MouseDown *MouseDown;
  struct MouseEvent_MouseMove *MouseMove;
  struct MouseEvent_MouseScrollWheel *MouseScrollWheel;
} MouseEventKind;

typedef struct wire_MouseEvent {
  int32_t tag;
  union MouseEventKind *kind;
} wire_MouseEvent;

typedef struct InputEvent_Mouse {
  struct wire_MouseEvent *field0;
} InputEvent_Mouse;

typedef struct KeyboardEvent_KeyUp {
  int32_t field0;
} KeyboardEvent_KeyUp;

typedef struct KeyboardEvent_KeyDown {
  int32_t field0;
} KeyboardEvent_KeyDown;

typedef union KeyboardEventKind {
  struct KeyboardEvent_KeyUp *KeyUp;
  struct KeyboardEvent_KeyDown *KeyDown;
} KeyboardEventKind;

typedef struct wire_KeyboardEvent {
  int32_t tag;
  union KeyboardEventKind *kind;
} wire_KeyboardEvent;

typedef struct InputEvent_Keyboard {
  struct wire_KeyboardEvent *field0;
} InputEvent_Keyboard;

typedef union InputEventKind {
  struct InputEvent_Mouse *Mouse;
  struct InputEvent_Keyboard *Keyboard;
} InputEventKind;

typedef struct wire_InputEvent {
  int32_t tag;
  union InputEventKind *kind;
} wire_InputEvent;

typedef struct WireSyncReturnStruct {
  uint8_t *ptr;
  int32_t len;
  bool success;
} WireSyncReturnStruct;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

#define kVK_ANSI_A 0

#define kVK_ANSI_S 1

#define kVK_ANSI_D 2

#define kVK_ANSI_F 3

#define kVK_ANSI_H 4

#define kVK_ANSI_G 5

#define kVK_ANSI_Z 6

#define kVK_ANSI_X 7

#define kVK_ANSI_C 8

#define kVK_ANSI_V 9

#define kVK_ANSI_B 11

#define kVK_ANSI_Q 12

#define kVK_ANSI_W 13

#define kVK_ANSI_E 14

#define kVK_ANSI_R 15

#define kVK_ANSI_Y 16

#define kVK_ANSI_T 17

#define kVK_ANSI_1 18

#define kVK_ANSI_2 19

#define kVK_ANSI_3 20

#define kVK_ANSI_4 21

#define kVK_ANSI_6 22

#define kVK_ANSI_5 23

#define kVK_ANSI_Equal 24

#define kVK_ANSI_9 25

#define kVK_ANSI_7 26

#define kVK_ANSI_Minus 27

#define kVK_ANSI_8 28

#define kVK_ANSI_0 29

#define kVK_ANSI_RightBracket 30

#define kVK_ANSI_O 31

#define kVK_ANSI_U 32

#define kVK_ANSI_LeftBracket 33

#define kVK_ANSI_I 34

#define kVK_ANSI_P 35

#define kVK_ANSI_L 37

#define kVK_ANSI_J 38

#define kVK_ANSI_Quote 39

#define kVK_ANSI_K 40

#define kVK_ANSI_Semicolon 41

#define kVK_ANSI_Backslash 42

#define kVK_ANSI_Comma 43

#define kVK_ANSI_Slash 44

#define kVK_ANSI_N 45

#define kVK_ANSI_M 46

#define kVK_ANSI_Period 47

#define kVK_ANSI_Grave 50

#define kVK_ANSI_KeypadDecimal 65

#define kVK_ANSI_KeypadMultiply 67

#define kVK_ANSI_KeypadPlus 69

#define kVK_ANSI_KeypadClear 71

#define kVK_ANSI_KeypadDivide 75

#define kVK_ANSI_KeypadEnter 76

#define kVK_ANSI_KeypadMinus 78

#define kVK_ANSI_KeypadEquals 81

#define kVK_ANSI_Keypad0 82

#define kVK_ANSI_Keypad1 83

#define kVK_ANSI_Keypad2 84

#define kVK_ANSI_Keypad3 85

#define kVK_ANSI_Keypad4 86

#define kVK_ANSI_Keypad5 87

#define kVK_ANSI_Keypad6 88

#define kVK_ANSI_Keypad7 89

#define kVK_ANSI_Keypad8 91

#define kVK_ANSI_Keypad9 92

#define kVK_Return 36

#define kVK_Tab 48

#define kVK_Space 49

#define kVK_Delete 51

#define kVK_Escape 53

#define kVK_Command 55

#define kVK_Shift 56

#define kVK_CapsLock 57

#define kVK_Option 58

#define kVK_Control 59

#define kVK_RightCommand 54

#define kVK_RightShift 60

#define kVK_RightOption 61

#define kVK_RightControl 62

#define kVK_Function 63

#define kVK_F17 64

#define kVK_VolumeUp 72

#define kVK_VolumeDown 73

#define kVK_Mute 74

#define kVK_F18 79

#define kVK_F19 80

#define kVK_F20 90

#define kVK_F5 96

#define kVK_F6 97

#define kVK_F7 98

#define kVK_F3 99

#define kVK_F8 100

#define kVK_F9 101

#define kVK_F11 103

#define kVK_F13 105

#define kVK_F16 106

#define kVK_F14 107

#define kVK_F10 109

#define kVK_F12 111

#define kVK_F15 113

#define kVK_Help 114

#define kVK_Home 115

#define kVK_PageUp 116

#define kVK_ForwardDelete 117

#define kVK_F4 118

#define kVK_End 119

#define kVK_F2 120

#define kVK_PageDown 121

#define kVK_F1 122

#define kVK_LeftArrow 123

#define kVK_RightArrow 124

#define kVK_DownArrow 125

#define kVK_UpArrow 126

#define kVK_ISO_Section 10

#define kVK_JIS_Yen 93

#define kVK_JIS_Underscore 94

#define kVK_JIS_KeypadComma 95

#define kVK_JIS_Eisu 102

#define kVK_JIS_Kana 104

void wire_init(int64_t port_,
               struct wire_uint_8_list *os_version,
               struct wire_uint_8_list *config_dir);

void wire_config_read_device_id(int64_t port_);

void wire_config_save_device_id(int64_t port_, struct wire_uint_8_list *device_id);

void wire_config_read_device_id_expiration(int64_t port_);

void wire_config_save_device_id_expiration(int64_t port_, int32_t time_stamp);

void wire_config_read_device_password(int64_t port_);

void wire_config_save_device_password(int64_t port_, struct wire_uint_8_list *device_password);

void wire_signaling_connect(int64_t port_, struct wire_uint_8_list *remote_device_id);

void wire_signaling_connection_key_exchange(int64_t port_,
                                            struct wire_uint_8_list *remote_device_id,
                                            struct wire_uint_8_list *password);

void wire_endpoint_get_display_info(int64_t port_, struct wire_uint_8_list *remote_device_id);

void wire_endpoint_start_media_transmission(int64_t port_,
                                            struct wire_uint_8_list *remote_device_id,
                                            uint8_t expect_fps,
                                            struct wire_uint_8_list *expect_display_id,
                                            int64_t texture_id,
                                            int64_t video_texture_ptr,
                                            int64_t update_frame_callback_ptr);

void wire_endpoint_input(int64_t port_,
                         struct wire_uint_8_list *remote_device_id,
                         struct wire_InputEvent *event);

void wire_endpoint_manually_close(int64_t port_, struct wire_uint_8_list *remote_device_id);

void wire_endpoint_close_notify(int64_t port_, struct wire_uint_8_list *remote_device_id);

struct wire_InputEvent *new_box_autoadd_input_event_0(void);

struct wire_KeyboardEvent *new_box_autoadd_keyboard_event_0(void);

struct wire_MouseEvent *new_box_autoadd_mouse_event_0(void);

struct wire_uint_8_list *new_uint_8_list_0(int32_t len);

union InputEventKind *inflate_InputEvent_Mouse(void);

union InputEventKind *inflate_InputEvent_Keyboard(void);

union KeyboardEventKind *inflate_KeyboardEvent_KeyUp(void);

union KeyboardEventKind *inflate_KeyboardEvent_KeyDown(void);

union MouseEventKind *inflate_MouseEvent_MouseUp(void);

union MouseEventKind *inflate_MouseEvent_MouseDown(void);

union MouseEventKind *inflate_MouseEvent_MouseMove(void);

union MouseEventKind *inflate_MouseEvent_MouseScrollWheel(void);

void free_WireSyncReturnStruct(struct WireSyncReturnStruct val);

void store_dart_post_cobject(DartPostCObjectFnType ptr);

static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_init);
    dummy_var ^= ((int64_t) (void*) wire_config_read_device_id);
    dummy_var ^= ((int64_t) (void*) wire_config_save_device_id);
    dummy_var ^= ((int64_t) (void*) wire_config_read_device_id_expiration);
    dummy_var ^= ((int64_t) (void*) wire_config_save_device_id_expiration);
    dummy_var ^= ((int64_t) (void*) wire_config_read_device_password);
    dummy_var ^= ((int64_t) (void*) wire_config_save_device_password);
    dummy_var ^= ((int64_t) (void*) wire_signaling_connect);
    dummy_var ^= ((int64_t) (void*) wire_signaling_connection_key_exchange);
    dummy_var ^= ((int64_t) (void*) wire_endpoint_get_display_info);
    dummy_var ^= ((int64_t) (void*) wire_endpoint_start_media_transmission);
    dummy_var ^= ((int64_t) (void*) wire_endpoint_input);
    dummy_var ^= ((int64_t) (void*) wire_endpoint_manually_close);
    dummy_var ^= ((int64_t) (void*) wire_endpoint_close_notify);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_input_event_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_keyboard_event_0);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_mouse_event_0);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list_0);
    dummy_var ^= ((int64_t) (void*) inflate_InputEvent_Mouse);
    dummy_var ^= ((int64_t) (void*) inflate_InputEvent_Keyboard);
    dummy_var ^= ((int64_t) (void*) inflate_KeyboardEvent_KeyUp);
    dummy_var ^= ((int64_t) (void*) inflate_KeyboardEvent_KeyDown);
    dummy_var ^= ((int64_t) (void*) inflate_MouseEvent_MouseUp);
    dummy_var ^= ((int64_t) (void*) inflate_MouseEvent_MouseDown);
    dummy_var ^= ((int64_t) (void*) inflate_MouseEvent_MouseMove);
    dummy_var ^= ((int64_t) (void*) inflate_MouseEvent_MouseScrollWheel);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturnStruct);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}
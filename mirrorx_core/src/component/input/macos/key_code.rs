/*
 *  Summary:
 *    Virtual keycodes
 *
 *  Discussion:
 *    These constants are the virtual keycodes defined originally in
 *    Inside Mac Volume V, pg. V-191. They identify physical keys on a
 *    keyboard. Those constants with "ANSI" in the name are labeled
 *    according to the key position on an ANSI-standard US keyboard.
 *    For example, kVK_ANSI_A indicates the virtual keycode for the key
 *    with the letter 'A' in the US keyboard layout. Other keyboard
 *    layouts may have the 'A' key label on a different physical key;
 *    in this case, pressing 'A' will generate a different virtual
 *    keycode.
 */

#![allow(non_upper_case_globals, unused)]
use core_graphics::event::CGKeyCode;

pub const kVK_ANSI_A: CGKeyCode = 0x00;
pub const kVK_ANSI_S: CGKeyCode = 0x01;
pub const kVK_ANSI_D: CGKeyCode = 0x02;
pub const kVK_ANSI_F: CGKeyCode = 0x03;
pub const kVK_ANSI_H: CGKeyCode = 0x04;
pub const kVK_ANSI_G: CGKeyCode = 0x05;
pub const kVK_ANSI_Z: CGKeyCode = 0x06;
pub const kVK_ANSI_X: CGKeyCode = 0x07;
pub const kVK_ANSI_C: CGKeyCode = 0x08;
pub const kVK_ANSI_V: CGKeyCode = 0x09;
pub const kVK_ANSI_B: CGKeyCode = 0x0B;
pub const kVK_ANSI_Q: CGKeyCode = 0x0C;
pub const kVK_ANSI_W: CGKeyCode = 0x0D;
pub const kVK_ANSI_E: CGKeyCode = 0x0E;
pub const kVK_ANSI_R: CGKeyCode = 0x0F;
pub const kVK_ANSI_Y: CGKeyCode = 0x10;
pub const kVK_ANSI_T: CGKeyCode = 0x11;
pub const kVK_ANSI_1: CGKeyCode = 0x12;
pub const kVK_ANSI_2: CGKeyCode = 0x13;
pub const kVK_ANSI_3: CGKeyCode = 0x14;
pub const kVK_ANSI_4: CGKeyCode = 0x15;
pub const kVK_ANSI_6: CGKeyCode = 0x16;
pub const kVK_ANSI_5: CGKeyCode = 0x17;
pub const kVK_ANSI_Equal: CGKeyCode = 0x18;
pub const kVK_ANSI_9: CGKeyCode = 0x19;
pub const kVK_ANSI_7: CGKeyCode = 0x1A;
pub const kVK_ANSI_Minus: CGKeyCode = 0x1B;
pub const kVK_ANSI_8: CGKeyCode = 0x1C;
pub const kVK_ANSI_0: CGKeyCode = 0x1D;
pub const kVK_ANSI_RightBracket: CGKeyCode = 0x1E;
pub const kVK_ANSI_O: CGKeyCode = 0x1F;
pub const kVK_ANSI_U: CGKeyCode = 0x20;
pub const kVK_ANSI_LeftBracket: CGKeyCode = 0x21;
pub const kVK_ANSI_I: CGKeyCode = 0x22;
pub const kVK_ANSI_P: CGKeyCode = 0x23;
pub const kVK_ANSI_L: CGKeyCode = 0x25;
pub const kVK_ANSI_J: CGKeyCode = 0x26;
pub const kVK_ANSI_Quote: CGKeyCode = 0x27;
pub const kVK_ANSI_K: CGKeyCode = 0x28;
pub const kVK_ANSI_Semicolon: CGKeyCode = 0x29;
pub const kVK_ANSI_Backslash: CGKeyCode = 0x2A;
pub const kVK_ANSI_Comma: CGKeyCode = 0x2B;
pub const kVK_ANSI_Slash: CGKeyCode = 0x2C;
pub const kVK_ANSI_N: CGKeyCode = 0x2D;
pub const kVK_ANSI_M: CGKeyCode = 0x2E;
pub const kVK_ANSI_Period: CGKeyCode = 0x2F;
pub const kVK_ANSI_Grave: CGKeyCode = 0x32;
pub const kVK_ANSI_KeypadDecimal: CGKeyCode = 0x41;
pub const kVK_ANSI_KeypadMultiply: CGKeyCode = 0x43;
pub const kVK_ANSI_KeypadPlus: CGKeyCode = 0x45;
pub const kVK_ANSI_KeypadClear: CGKeyCode = 0x47;
pub const kVK_ANSI_KeypadDivide: CGKeyCode = 0x4B;
pub const kVK_ANSI_KeypadEnter: CGKeyCode = 0x4C;
pub const kVK_ANSI_KeypadMinus: CGKeyCode = 0x4E;
pub const kVK_ANSI_KeypadEquals: CGKeyCode = 0x51;
pub const kVK_ANSI_Keypad0: CGKeyCode = 0x52;
pub const kVK_ANSI_Keypad1: CGKeyCode = 0x53;
pub const kVK_ANSI_Keypad2: CGKeyCode = 0x54;
pub const kVK_ANSI_Keypad3: CGKeyCode = 0x55;
pub const kVK_ANSI_Keypad4: CGKeyCode = 0x56;
pub const kVK_ANSI_Keypad5: CGKeyCode = 0x57;
pub const kVK_ANSI_Keypad6: CGKeyCode = 0x58;
pub const kVK_ANSI_Keypad7: CGKeyCode = 0x59;
pub const kVK_ANSI_Keypad8: CGKeyCode = 0x5B;
pub const kVK_ANSI_Keypad9: CGKeyCode = 0x5C;

/* keycodes for keys that are independent of keyboard layout*/

pub const kVK_Return: CGKeyCode = 0x24;
pub const kVK_Tab: CGKeyCode = 0x30;
pub const kVK_Space: CGKeyCode = 0x31;
pub const kVK_Delete: CGKeyCode = 0x33;
pub const kVK_Escape: CGKeyCode = 0x35;
pub const kVK_Command: CGKeyCode = 0x37;
pub const kVK_Shift: CGKeyCode = 0x38;
pub const kVK_CapsLock: CGKeyCode = 0x39;
pub const kVK_Option: CGKeyCode = 0x3A;
pub const kVK_Control: CGKeyCode = 0x3B;
pub const kVK_RightCommand: CGKeyCode = 0x36;
pub const kVK_RightShift: CGKeyCode = 0x3C;
pub const kVK_RightOption: CGKeyCode = 0x3D;
pub const kVK_RightControl: CGKeyCode = 0x3E;
pub const kVK_Function: CGKeyCode = 0x3F;
pub const kVK_F17: CGKeyCode = 0x40;
pub const kVK_VolumeUp: CGKeyCode = 0x48;
pub const kVK_VolumeDown: CGKeyCode = 0x49;
pub const kVK_Mute: CGKeyCode = 0x4A;
pub const kVK_F18: CGKeyCode = 0x4F;
pub const kVK_F19: CGKeyCode = 0x50;
pub const kVK_F20: CGKeyCode = 0x5A;
pub const kVK_F5: CGKeyCode = 0x60;
pub const kVK_F6: CGKeyCode = 0x61;
pub const kVK_F7: CGKeyCode = 0x62;
pub const kVK_F3: CGKeyCode = 0x63;
pub const kVK_F8: CGKeyCode = 0x64;
pub const kVK_F9: CGKeyCode = 0x65;
pub const kVK_F11: CGKeyCode = 0x67;
pub const kVK_F13: CGKeyCode = 0x69;
pub const kVK_F16: CGKeyCode = 0x6A;
pub const kVK_F14: CGKeyCode = 0x6B;
pub const kVK_F10: CGKeyCode = 0x6D;
pub const kVK_F12: CGKeyCode = 0x6F;
pub const kVK_F15: CGKeyCode = 0x71;
pub const kVK_Help: CGKeyCode = 0x72;
pub const kVK_Home: CGKeyCode = 0x73;
pub const kVK_PageUp: CGKeyCode = 0x74;
pub const kVK_ForwardDelete: CGKeyCode = 0x75;
pub const kVK_F4: CGKeyCode = 0x76;
pub const kVK_End: CGKeyCode = 0x77;
pub const kVK_F2: CGKeyCode = 0x78;
pub const kVK_PageDown: CGKeyCode = 0x79;
pub const kVK_F1: CGKeyCode = 0x7A;
pub const kVK_LeftArrow: CGKeyCode = 0x7B;
pub const kVK_RightArrow: CGKeyCode = 0x7C;
pub const kVK_DownArrow: CGKeyCode = 0x7D;
pub const kVK_UpArrow: CGKeyCode = 0x7E;

/* ISO keyboards only*/

pub const kVK_ISO_Section: CGKeyCode = 0x0A;

/* JIS keyboards only*/

pub const kVK_JIS_Yen: CGKeyCode = 0x5D;
pub const kVK_JIS_Underscore: CGKeyCode = 0x5E;
pub const kVK_JIS_KeypadComma: CGKeyCode = 0x5F;
pub const kVK_JIS_Eisu: CGKeyCode = 0x66;
pub const kVK_JIS_Kana: CGKeyCode = 0x68;

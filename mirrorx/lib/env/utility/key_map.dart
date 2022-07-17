import 'package:flutter/services.dart';
import 'package:mirrorx/env/sdk/mirrorx_core.dart' as sdk;

sdk.KeyboardKey? mapLogicalKey(LogicalKeyboardKey logicalKey) {
  if (logicalKey.keyId == LogicalKeyboardKey.keyA.keyId) {
    return sdk.KeyboardKey.A;
  } else if (logicalKey.keyId == LogicalKeyboardKey.keyB.keyId) {
    return sdk.KeyboardKey.B;
  } else if (logicalKey.keyId == LogicalKeyboardKey.keyC.keyId) {
    return sdk.KeyboardKey.C;
  } else if (logicalKey.keyId == LogicalKeyboardKey.keyD.keyId) {
    return sdk.KeyboardKey.D;
  } else if (logicalKey.keyId == LogicalKeyboardKey.keyE.keyId) {
    return sdk.KeyboardKey.E;
  } else if (logicalKey.keyId == LogicalKeyboardKey.keyF.keyId) {
    return sdk.KeyboardKey.F;
  } else if (logicalKey.keyId == LogicalKeyboardKey.keyG.keyId) {
    return sdk.KeyboardKey.G;
  } else if (logicalKey.keyId == LogicalKeyboardKey.keyH.keyId) {
    return sdk.KeyboardKey.H;
  } else if (logicalKey.keyId == LogicalKeyboardKey.keyI.keyId) {
    return sdk.KeyboardKey.I;
  } else if (logicalKey.keyId == LogicalKeyboardKey.keyJ.keyId) {
    return sdk.KeyboardKey.J;
  } else if (logicalKey.keyId == LogicalKeyboardKey.keyK.keyId) {
    return sdk.KeyboardKey.K;
  } else if (logicalKey.keyId == LogicalKeyboardKey.keyL.keyId) {
    return sdk.KeyboardKey.L;
  } else if (logicalKey.keyId == LogicalKeyboardKey.keyM.keyId) {
    return sdk.KeyboardKey.M;
  } else if (logicalKey.keyId == LogicalKeyboardKey.keyN.keyId) {
    return sdk.KeyboardKey.N;
  } else if (logicalKey.keyId == LogicalKeyboardKey.keyO.keyId) {
    return sdk.KeyboardKey.O;
  } else if (logicalKey.keyId == LogicalKeyboardKey.keyP.keyId) {
    return sdk.KeyboardKey.P;
  } else if (logicalKey.keyId == LogicalKeyboardKey.keyQ.keyId) {
    return sdk.KeyboardKey.Q;
  } else if (logicalKey.keyId == LogicalKeyboardKey.keyR.keyId) {
    return sdk.KeyboardKey.R;
  } else if (logicalKey.keyId == LogicalKeyboardKey.keyS.keyId) {
    return sdk.KeyboardKey.S;
  } else if (logicalKey.keyId == LogicalKeyboardKey.keyT.keyId) {
    return sdk.KeyboardKey.T;
  } else if (logicalKey.keyId == LogicalKeyboardKey.keyU.keyId) {
    return sdk.KeyboardKey.U;
  } else if (logicalKey.keyId == LogicalKeyboardKey.keyV.keyId) {
    return sdk.KeyboardKey.V;
  } else if (logicalKey.keyId == LogicalKeyboardKey.keyW.keyId) {
    return sdk.KeyboardKey.W;
  } else if (logicalKey.keyId == LogicalKeyboardKey.keyX.keyId) {
    return sdk.KeyboardKey.X;
  } else if (logicalKey.keyId == LogicalKeyboardKey.keyY.keyId) {
    return sdk.KeyboardKey.Y;
  } else if (logicalKey.keyId == LogicalKeyboardKey.keyZ.keyId) {
    return sdk.KeyboardKey.Z;
  } else if (logicalKey.keyId == LogicalKeyboardKey.backquote.keyId) {
    return sdk.KeyboardKey.BackQuote;
  } else if (logicalKey.keyId == LogicalKeyboardKey.digit0.keyId) {
    return sdk.KeyboardKey.Digit0;
  } else if (logicalKey.keyId == LogicalKeyboardKey.digit1.keyId) {
    return sdk.KeyboardKey.Digit1;
  } else if (logicalKey.keyId == LogicalKeyboardKey.digit2.keyId) {
    return sdk.KeyboardKey.Digit2;
  } else if (logicalKey.keyId == LogicalKeyboardKey.digit3.keyId) {
    return sdk.KeyboardKey.Digit3;
  } else if (logicalKey.keyId == LogicalKeyboardKey.digit4.keyId) {
    return sdk.KeyboardKey.Digit4;
  } else if (logicalKey.keyId == LogicalKeyboardKey.digit5.keyId) {
    return sdk.KeyboardKey.Digit5;
  } else if (logicalKey.keyId == LogicalKeyboardKey.digit6.keyId) {
    return sdk.KeyboardKey.Digit6;
  } else if (logicalKey.keyId == LogicalKeyboardKey.digit7.keyId) {
    return sdk.KeyboardKey.Digit7;
  } else if (logicalKey.keyId == LogicalKeyboardKey.digit8.keyId) {
    return sdk.KeyboardKey.Digit8;
  } else if (logicalKey.keyId == LogicalKeyboardKey.digit9.keyId) {
    return sdk.KeyboardKey.Digit9;
  } else if (logicalKey.keyId == LogicalKeyboardKey.minus.keyId) {
    return sdk.KeyboardKey.Minus;
  } else if (logicalKey.keyId == LogicalKeyboardKey.equal.keyId) {
    return sdk.KeyboardKey.Equal;
  } else if (logicalKey.keyId == LogicalKeyboardKey.tab.keyId) {
    return sdk.KeyboardKey.Tab;
  } else if (logicalKey.keyId == LogicalKeyboardKey.capsLock.keyId) {
    return sdk.KeyboardKey.CapsLock;
  } else if (logicalKey.keyId == LogicalKeyboardKey.shiftLeft.keyId) {
    return sdk.KeyboardKey.LeftShift;
  } else if (logicalKey.keyId == LogicalKeyboardKey.controlLeft.keyId) {
    return sdk.KeyboardKey.LeftControl;
  } else if (logicalKey.keyId == LogicalKeyboardKey.altLeft.keyId) {
    return sdk.KeyboardKey.LeftAlt;
  } else if (logicalKey.keyId == LogicalKeyboardKey.metaLeft.keyId) {
    return sdk.KeyboardKey.LeftMeta;
  } else if (logicalKey.keyId == LogicalKeyboardKey.space.keyId) {
    return sdk.KeyboardKey.Space;
  } else if (logicalKey.keyId == LogicalKeyboardKey.metaRight.keyId) {
    return sdk.KeyboardKey.RightMeta;
  } else if (logicalKey.keyId == LogicalKeyboardKey.controlRight.keyId) {
    return sdk.KeyboardKey.RightControl;
  } else if (logicalKey.keyId == LogicalKeyboardKey.altRight.keyId) {
    return sdk.KeyboardKey.RightAlt;
  } else if (logicalKey.keyId == LogicalKeyboardKey.shiftRight.keyId) {
    return sdk.KeyboardKey.RightShift;
  } else if (logicalKey.keyId == LogicalKeyboardKey.comma.keyId) {
    return sdk.KeyboardKey.Comma;
  } else if (logicalKey.keyId == LogicalKeyboardKey.period.keyId) {
    return sdk.KeyboardKey.Period;
  } else if (logicalKey.keyId == LogicalKeyboardKey.slash.keyId) {
    return sdk.KeyboardKey.Slash;
  } else if (logicalKey.keyId == LogicalKeyboardKey.semicolon.keyId) {
    return sdk.KeyboardKey.Semicolon;
  } else if (logicalKey.keyId == LogicalKeyboardKey.quoteSingle.keyId) {
    return sdk.KeyboardKey.QuoteSingle;
  } else if (logicalKey.keyId == LogicalKeyboardKey.enter.keyId) {
    return sdk.KeyboardKey.Enter;
  } else if (logicalKey.keyId == LogicalKeyboardKey.bracketLeft.keyId) {
    return sdk.KeyboardKey.BracketLeft;
  } else if (logicalKey.keyId == LogicalKeyboardKey.bracketRight.keyId) {
    return sdk.KeyboardKey.BracketRight;
  } else if (logicalKey.keyId == LogicalKeyboardKey.backslash.keyId) {
    return sdk.KeyboardKey.BackSlash;
  } else if (logicalKey.keyId == LogicalKeyboardKey.backspace.keyId) {
    return sdk.KeyboardKey.Backspace;
  } else if (logicalKey.keyId == LogicalKeyboardKey.numLock.keyId) {
    return sdk.KeyboardKey.NumLock;
  } else if (logicalKey.keyId == LogicalKeyboardKey.numpadEqual.keyId) {
    return sdk.KeyboardKey.NumpadEquals;
  } else if (logicalKey.keyId == LogicalKeyboardKey.numpadDivide.keyId) {
    return sdk.KeyboardKey.NumpadDivide;
  } else if (logicalKey.keyId == LogicalKeyboardKey.numpadMultiply.keyId) {
    return sdk.KeyboardKey.NumpadMultiply;
  } else if (logicalKey.keyId == LogicalKeyboardKey.numpadSubtract.keyId) {
    return sdk.KeyboardKey.NumpadSubtract;
  } else if (logicalKey.keyId == LogicalKeyboardKey.numpadAdd.keyId) {
    return sdk.KeyboardKey.NumpadAdd;
  } else if (logicalKey.keyId == LogicalKeyboardKey.numpadEnter.keyId) {
    return sdk.KeyboardKey.NumpadEnter;
  } else if (logicalKey.keyId == LogicalKeyboardKey.numpad0.keyId) {
    return sdk.KeyboardKey.Numpad0;
  } else if (logicalKey.keyId == LogicalKeyboardKey.numpad1.keyId) {
    return sdk.KeyboardKey.Numpad1;
  } else if (logicalKey.keyId == LogicalKeyboardKey.numpad2.keyId) {
    return sdk.KeyboardKey.Numpad2;
  } else if (logicalKey.keyId == LogicalKeyboardKey.numpad3.keyId) {
    return sdk.KeyboardKey.Numpad3;
  } else if (logicalKey.keyId == LogicalKeyboardKey.numpad4.keyId) {
    return sdk.KeyboardKey.Numpad4;
  } else if (logicalKey.keyId == LogicalKeyboardKey.numpad5.keyId) {
    return sdk.KeyboardKey.Numpad5;
  } else if (logicalKey.keyId == LogicalKeyboardKey.numpad6.keyId) {
    return sdk.KeyboardKey.Numpad6;
  } else if (logicalKey.keyId == LogicalKeyboardKey.numpad7.keyId) {
    return sdk.KeyboardKey.Numpad7;
  } else if (logicalKey.keyId == LogicalKeyboardKey.numpad8.keyId) {
    return sdk.KeyboardKey.Numpad8;
  } else if (logicalKey.keyId == LogicalKeyboardKey.numpad9.keyId) {
    return sdk.KeyboardKey.Numpad9;
  } else if (logicalKey.keyId == LogicalKeyboardKey.numpadDecimal.keyId) {
    return sdk.KeyboardKey.NumpadDecimal;
  } else if (logicalKey.keyId == LogicalKeyboardKey.arrowLeft.keyId) {
    return sdk.KeyboardKey.ArrowLeft;
  } else if (logicalKey.keyId == LogicalKeyboardKey.arrowUp.keyId) {
    return sdk.KeyboardKey.ArrowUp;
  } else if (logicalKey.keyId == LogicalKeyboardKey.arrowRight.keyId) {
    return sdk.KeyboardKey.ArrowRight;
  } else if (logicalKey.keyId == LogicalKeyboardKey.arrowDown.keyId) {
    return sdk.KeyboardKey.ArrowDown;
  } else if (logicalKey.keyId == LogicalKeyboardKey.escape.keyId) {
    return sdk.KeyboardKey.Escape;
  } else if (logicalKey.keyId == LogicalKeyboardKey.printScreen.keyId ||
      logicalKey.keyId == LogicalKeyboardKey.f13.keyId) {
    return sdk.KeyboardKey.PrintScreen;
  } else if (logicalKey.keyId == LogicalKeyboardKey.scrollLock.keyId) {
    return sdk.KeyboardKey.ScrollLock;
  } else if (logicalKey.keyId == LogicalKeyboardKey.pause.keyId) {
    return sdk.KeyboardKey.Pause;
  } else if (logicalKey.keyId == LogicalKeyboardKey.insert.keyId) {
    return sdk.KeyboardKey.Insert;
  } else if (logicalKey.keyId == LogicalKeyboardKey.delete.keyId) {
    return sdk.KeyboardKey.Delete;
  } else if (logicalKey.keyId == LogicalKeyboardKey.home.keyId) {
    return sdk.KeyboardKey.Home;
  } else if (logicalKey.keyId == LogicalKeyboardKey.end.keyId) {
    return sdk.KeyboardKey.End;
  } else if (logicalKey.keyId == LogicalKeyboardKey.pageUp.keyId) {
    return sdk.KeyboardKey.PageUp;
  } else if (logicalKey.keyId == LogicalKeyboardKey.pageDown.keyId) {
    return sdk.KeyboardKey.PageDown;
  } else if (logicalKey.keyId == LogicalKeyboardKey.f1.keyId) {
    return sdk.KeyboardKey.F1;
  } else if (logicalKey.keyId == LogicalKeyboardKey.f2.keyId) {
    return sdk.KeyboardKey.F2;
  } else if (logicalKey.keyId == LogicalKeyboardKey.f3.keyId) {
    return sdk.KeyboardKey.F3;
  } else if (logicalKey.keyId == LogicalKeyboardKey.f4.keyId) {
    return sdk.KeyboardKey.F4;
  } else if (logicalKey.keyId == LogicalKeyboardKey.f5.keyId) {
    return sdk.KeyboardKey.F5;
  } else if (logicalKey.keyId == LogicalKeyboardKey.f6.keyId) {
    return sdk.KeyboardKey.F6;
  } else if (logicalKey.keyId == LogicalKeyboardKey.f7.keyId) {
    return sdk.KeyboardKey.F7;
  } else if (logicalKey.keyId == LogicalKeyboardKey.f8.keyId) {
    return sdk.KeyboardKey.F8;
  } else if (logicalKey.keyId == LogicalKeyboardKey.f9.keyId) {
    return sdk.KeyboardKey.F9;
  } else if (logicalKey.keyId == LogicalKeyboardKey.f10.keyId) {
    return sdk.KeyboardKey.F10;
  } else if (logicalKey.keyId == LogicalKeyboardKey.f11.keyId) {
    return sdk.KeyboardKey.F11;
  } else if (logicalKey.keyId == LogicalKeyboardKey.f12.keyId) {
    return sdk.KeyboardKey.F12;
  } else if (logicalKey.keyId == LogicalKeyboardKey.fn.keyId) {
    return sdk.KeyboardKey.Fn;
  }
  return null;
}

import 'dart:math';

const _passwordLength = 24;
const strongPasswordAlphabet =
    r"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789@#$%^*?!=+<>(){}";

String generatePassword() {
  while (true) {
    final result = List.generate(_passwordLength, (index) {
      final indexRandom =
          Random.secure().nextInt(strongPasswordAlphabet.length);
      return strongPasswordAlphabet[indexRandom];
    }).join('');

    if (RegExp(r'(?=.*[A-Z]{1,})(?=.*[@#$%^*?!=+<>(){}]{1,})')
        .hasMatch(result)) {
      return result;
    }
  }
}

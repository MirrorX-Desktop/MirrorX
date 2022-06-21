import 'dart:math';

const _passwordLength = 16;
const _alphabet =
    r"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789@#%^*>\$@?/[]=+";

String generatePassword() {
  return List.generate(_passwordLength, (index) {
    final indexRandom = Random.secure().nextInt(_alphabet.length);
    return _alphabet[indexRandom];
  }).join('');
}

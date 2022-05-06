import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:video_texture/video_texture.dart';

void main() {
  // const MethodChannel channel = MethodChannel('video_texture');

  TestWidgetsFlutterBinding.ensureInitialized();

  // setUp(() {
  //   channel.setMockMethodCallHandler((MethodCall methodCall) async {
  //     return '42';
  //   });
  // });

  // tearDown(() {
  //   channel.setMockMethodCallHandler(null);
  // });

  test('register_texture', () async {
    var textureID = await VideoTexture.registerTextureID();
    if (kDebugMode) {
      print(textureID);
    }
    expect(textureID, isNot(0));
  });
}

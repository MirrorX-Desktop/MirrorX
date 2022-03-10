import 'package:mirrorx_sdk/bridge_generated.dart';
import 'package:path_provider/path_provider.dart';

class Config {
  final MirrorXCore _core;

  Config(MirrorXCore core) : _core = core;

  Future<String?> readConfig(String key) => _core.readConfig(key: key);

  Future<void> storeConfig(String key, String value) =>
      _core.storeConfig(key: key, value: value);
}

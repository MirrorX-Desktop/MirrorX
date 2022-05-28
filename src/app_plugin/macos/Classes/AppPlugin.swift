import Cocoa
import FlutterMacOS


public class AppPlugin: NSObject, FlutterPlugin {
    var textureRegistry: FlutterTextureRegistry
    
    init(textureRegistry: FlutterTextureRegistry) {
        self.textureRegistry = textureRegistry
        super.init()
    }
    
    public static func register(with registrar: FlutterPluginRegistrar) {
        let channel = FlutterMethodChannel(name: "app_plugin", binaryMessenger: registrar.messenger)
        let instance = AppPlugin(textureRegistry: registrar.textures)
        registrar.addMethodCallDelegate(instance, channel: channel)
    }
    
    public func handle(_ call: FlutterMethodCall, result: @escaping FlutterResult) {
        print("handle: \(call)")
        let args: NSDictionary? = call.arguments as? NSDictionary
        
        switch call.method {
        case "video_texture_register":
            let textureID = createVideoTexture(with: self.textureRegistry)
            let a = unsafeBitCast(update_frame_callback, to: Int64.self)
            
            var res = Dictionary<String,String>.init()
            res["textureID"]=String(textureID)
            res["callbackPtr"]=String(a)
            
            result(res)
        case "video_texture_deregister":
            guard let textureID: Int64 = args?["texture_id"] as? Int64 else {
                result(Void.self)
                return
            }
            
            removeVideoTexture(with: self.textureRegistry, textureID: textureID)
            result(Void.self)
        default:
            result(FlutterMethodNotImplemented)
        }
    }
}

func dummy_link_headers() {
    dummy_method_to_enforce_bundling()
}

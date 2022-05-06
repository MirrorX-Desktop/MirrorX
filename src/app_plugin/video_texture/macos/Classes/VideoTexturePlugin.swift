import Cocoa
import FlutterMacOS

public class VideoTexturePlugin: NSObject, FlutterPlugin {
    var textureIDs: [Int64: FlutterTexture]
    var textureRegistry: FlutterTextureRegistry
    
    init(textureRegistry: FlutterTextureRegistry) {
        self.textureIDs = [:]
        self.textureRegistry = textureRegistry
        super.init()
    }
    
    public static func register(with registrar: FlutterPluginRegistrar) {
        let channel = FlutterMethodChannel(name: "video_texture", binaryMessenger: registrar.messenger)
        let instance = VideoTexturePlugin(textureRegistry: registrar.textures)
        registrar.addMethodCallDelegate(instance, channel: channel)
    }
    
    public func handle(_ call: FlutterMethodCall, result: @escaping FlutterResult) {
        print("handle: \(call)")
        let args:NSDictionary? = call.arguments as? NSDictionary
        
        switch call.method {
        case "register_texture":
            var textureID: Int64 = 0
            let texture = VideoTextureController(frameAvailableClosure: {self.textureRegistry.textureFrameAvailable(textureID)})
            
            textureID = self.textureRegistry.register(texture)
            
            self.textureIDs[textureID]=texture
            
            result(textureID)
        case "dispose_texture":
            guard let textureID:Int64 = args?["texture_id"] as? Int64 else {
                result(Void.self)
                return
            }
            
            self.textureIDs.removeValue(forKey: textureID)
            self.textureRegistry.unregisterTexture(textureID)
            result(Void.self)
        default:
            result(FlutterMethodNotImplemented)
        }
        
    }
}

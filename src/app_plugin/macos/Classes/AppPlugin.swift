import Cocoa
import FlutterMacOS

let update_frame_callback:@convention(c)(Int64, UnsafeMutableRawPointer, UnsafeMutableRawPointer) -> Void = { (textureID,videoTexturePointer,newFramePointer) in
    let videoTexture = Unmanaged<VideoTexture>.fromOpaque(videoTexturePointer).takeUnretainedValue()
    let newFrame = Unmanaged<CVPixelBuffer>.fromOpaque(newFramePointer)
    videoTexture.updateFrame(textureID: textureID, pixelBuffer: newFrame)
}

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
        let args = call.arguments as? NSDictionary
        
        switch call.method {
        case "register_video_texture":
            let videoTexture = VideoTexture.init(registry: self.textureRegistry)
            let textureID = self.textureRegistry.register(videoTexture)
            let updateFrameCallbackPointer = unsafeBitCast(update_frame_callback, to: Int64.self)
            let videoTexturePointer = Int64(Int(bitPattern: Unmanaged.passRetained(videoTexture).toOpaque()))
            
            var res = Dictionary<String, Int64>.init()
            res["texture_id"] = textureID
            res["video_texture_ptr"] = videoTexturePointer
            res["update_frame_callback_ptr"] = updateFrameCallbackPointer
            
            result(res)
        case "deregister_video_texture":
            guard let textureID: Int64 = args?["texture_id"] as? Int64 else {
                result("texture_id is invalid")
                return
            }
            
            guard let videoTexturePointerAddress:Int64 = args?["video_texture_ptr"] as? Int64 else{
                result("video_texture_ptr is invalid")
                return
            }
            
            self.textureRegistry.unregisterTexture(textureID)
            
            guard let videoTexturePointer = UnsafeMutablePointer<VideoTexture>.init(bitPattern: Int(videoTexturePointerAddress)) else{
                result("parse video texture pointer failed")
                return
            }
            
            Unmanaged<VideoTexture>.fromOpaque(videoTexturePointer).release()
            
            result(Void.self)
        default:
            result(FlutterMethodNotImplemented)
        }
    }
}

func dummy_link_headers() {
    dummy_method_to_enforce_bundling()
}

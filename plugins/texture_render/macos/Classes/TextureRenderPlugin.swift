import Cocoa
import FlutterMacOS

let update_frame_callback:
@convention(c) (UnsafeRawPointer, UnsafeRawPointer) -> Void = {
    (videoTexturePointer, pixelBufferPointer) in
    let videoTexture = Unmanaged<VideoTexture>.fromOpaque(videoTexturePointer).takeUnretainedValue()
    videoTexture.updateFrame(pixelBufferPointer: pixelBufferPointer)
}

public class TextureRenderPlugin: NSObject, FlutterPlugin {
    var textureRegistry: FlutterTextureRegistry
    
    init(textureRegistry: FlutterTextureRegistry) {
        self.textureRegistry = textureRegistry
        super.init()
    }
    
    public static func register(with registrar: FlutterPluginRegistrar) {
        let channel = FlutterMethodChannel(name: "texture_render", binaryMessenger: registrar.messenger)
        let instance = TextureRenderPlugin(textureRegistry: registrar.textures)
        registrar.addMethodCallDelegate(instance, channel: channel)
    }
    
    public func handle(_ call: FlutterMethodCall, result: @escaping FlutterResult) {
        let args = call.arguments as? NSDictionary
        
        switch call.method {
        case "register_texture":
            guard let videoTexture = VideoTexture.init(self.textureRegistry) else {
                result("create texture failed")
                return
            }
            
            let textureID = videoTexture.textureID
            let updateFrameCallbackPointer = unsafeBitCast(update_frame_callback, to: Int64.self)
            let videoTexturePointer = Int64(
                Int(bitPattern: Unmanaged.passRetained(videoTexture).toOpaque()))
            
            var res = Dictionary<String, Int64>.init()
            res["texture_id"] = textureID
            res["video_texture_ptr"] = videoTexturePointer
            res["update_frame_callback_ptr"] = updateFrameCallbackPointer
            
            result(res)
        case "deregister_texture":
            guard let videoTexturePointerAddress: Int64 = args?["video_texture_ptr"] as? Int64 else {
                result("video_texture_ptr is invalid")
                return
            }
            
            guard
                let videoTexturePointer = UnsafeMutablePointer<VideoTexture>.init(
                    bitPattern: Int(videoTexturePointerAddress))
            else {
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

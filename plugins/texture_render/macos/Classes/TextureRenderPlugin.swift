import Cocoa
import FlutterMacOS
import NIOCore
import NIOFoundationCompat

//let update_frame_callback:
//@convention(c) (UnsafeRawPointer, UnsafeRawPointer) -> Void = {
//    (videoTexturePointer, pixelBufferPointer) in
//    let videoTexture = Unmanaged<VideoTexture>.fromOpaque(videoTexturePointer).takeUnretainedValue()
//    videoTexture.updateFrame(pixelBufferPointer: pixelBufferPointer)
//}

public class TextureRenderPlugin: NSObject, FlutterPlugin {
    var textureRegistry: FlutterTextureRegistry
    var videoTextures:[Int64:VideoTexture]
    var videoTexturesRWLock:pthread_rwlock_t
    var videoBufferChannel:FlutterBasicMessageChannel
    
    init(textureRegistry: FlutterTextureRegistry, binaryMessager:FlutterBinaryMessenger) {
        self.textureRegistry = textureRegistry
        self.videoBufferChannel = FlutterBasicMessageChannel(name: "texture_render_binary_channel", binaryMessenger: binaryMessager, codec: FlutterBinaryCodec())
        self.videoTextures = [Int64:VideoTexture]()
        self.videoTexturesRWLock = pthread_rwlock_t()
        
        pthread_rwlock_init(&self.videoTexturesRWLock, nil)
        
        super.init()
        
        self.videoBufferChannel.setMessageHandler{message,reply in
            guard let buffer = message as? Data else {
                return
            }
            
            self.handleVideoBuffer(buffer)
        }
    }
    
    public static func register(with registrar: FlutterPluginRegistrar) {
        let instance = TextureRenderPlugin(textureRegistry: registrar.textures,binaryMessager: registrar.messenger)
        let channel = FlutterMethodChannel(name: "texture_render_method_channel", binaryMessenger: registrar.messenger)
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
            
            let textureId = videoTexture.textureId
            
            pthread_rwlock_wrlock(&self.videoTexturesRWLock)
            self.videoTextures[textureId]=videoTexture
            pthread_rwlock_unlock(&self.videoTexturesRWLock)
            
            var res = Dictionary<String, Int64>.init()
            res["texture_id"] = textureId
//            res["video_texture_ptr"] = videoTexturePointer
//            res["update_frame_callback_ptr"] = updateFrameCallbackPointer
            
            result(res)
        case "deregister_texture":
            guard let textureId = args?["texture_id"] as? Int64 else{
                result(Void.self)
                return
            }
            
            pthread_rwlock_wrlock(&self.videoTexturesRWLock)
            self.videoTextures.removeValue(forKey: textureId)
            pthread_rwlock_unlock(&self.videoTexturesRWLock)
            
            result(Void.self)
        default:
            result(FlutterMethodNotImplemented)
        }
    }
    
    public func handleVideoBuffer(_ buffer:Data){
        
        // 8: id
        // 4: width
        // 4: height
        // 4: lumina stride
        // 4: chroma stride
        // 4: lumina body length
        // n: lumina body
        // 4: chroma body length
        // n: chroma body
        
        var byteBuffer = ByteBuffer(data: buffer)
       
        guard let id = byteBuffer.readInteger(endianness: .little, as: Int64.self) else{
            return
        }
        
        guard let width = byteBuffer.readInteger(endianness: .little, as: Int32.self) else{
            return
        }
        
        guard let height = byteBuffer.readInteger(endianness: .little, as: Int32.self) else{
            return
        }
        
        guard let luminaStride = byteBuffer.readInteger(endianness: .little, as: Int32.self) else{
            return
        }
        
        guard let chromaStride = byteBuffer.readInteger(endianness: .little, as: Int32.self) else{
            return
        }
        
        guard let luminaBody = byteBuffer.readLengthPrefixedSlice(endianness: .little, as: Int32.self) else{
            return
        }
        
        guard let chromaBody = byteBuffer.readLengthPrefixedSlice(endianness: .little, as: Int32.self) else{
            return
        }
        
        pthread_rwlock_rdlock(&self.videoTexturesRWLock)
        defer{
            pthread_rwlock_unlock(&self.videoTexturesRWLock)
        }
        
        guard let videoTexture = videoTextures[id] else{
            return
        }
        
        videoTexture.updateFrame(width, height, luminaStride, chromaStride, luminaBody, chromaBody)
    }
    
    deinit{
        pthread_rwlock_destroy(&self.videoTexturesRWLock)
    }
}

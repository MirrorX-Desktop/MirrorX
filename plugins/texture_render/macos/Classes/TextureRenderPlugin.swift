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
        self.videoBufferChannel = FlutterBasicMessageChannel(name: "texture_render_binary_channel", binaryMessenger: binaryMessager, codec: FlutterBinaryCodec.sharedInstance())
        self.videoTextures = [Int64:VideoTexture]()
        self.videoTexturesRWLock = pthread_rwlock_t()
        
        pthread_rwlock_init(&self.videoTexturesRWLock, nil)
        
        super.init()
        
        self.videoBufferChannel.setMessageHandler{ message, reply in
            guard let message = message else {
                return
            }
            
            guard let buffer = message as? Data else {
                return
            }

            self.handleVideoBuffer(buffer)
            
            reply(nil)
        }
    }
    
    public static func register(with registrar: FlutterPluginRegistrar) {
        let instance = TextureRenderPlugin(textureRegistry: registrar.textures, binaryMessager: registrar.messenger)
        let channel = FlutterMethodChannel(name: "texture_render_method_channel", binaryMessenger: registrar.messenger)
        registrar.addMethodCallDelegate(instance, channel: channel)
    }
    
    public func handle(_ call: FlutterMethodCall, result: @escaping FlutterResult) {
        switch call.method {
        case "register_texture":
            print("texture render plugin: register_texture")
            guard let videoTexture = VideoTexture.init(self.textureRegistry) else {
                result("create texture failed")
                return
            }
            
            let textureId = videoTexture.textureId
            
            pthread_rwlock_wrlock(&self.videoTexturesRWLock)
            self.videoTextures[textureId]=videoTexture
            pthread_rwlock_unlock(&self.videoTexturesRWLock)
            
            result(textureId)
        case "deregister_texture":
            print("texture render plugin: deregister_texture")
            guard let textureId = call.arguments as? Int64 else{
                result(false.self)
                return
            }
            
            pthread_rwlock_wrlock(&self.videoTexturesRWLock)
            self.videoTextures.removeValue(forKey: textureId)
            pthread_rwlock_unlock(&self.videoTexturesRWLock)
            
            result(true.self)
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
        guard let videoTexture = videoTextures[id] else{
            pthread_rwlock_unlock(&self.videoTexturesRWLock)
            return
        }
        pthread_rwlock_unlock(&self.videoTexturesRWLock)
        
        
        videoTexture.updateFrame(width, height, luminaStride, chromaStride, luminaBody, chromaBody)
    }
    
    deinit{
        pthread_rwlock_destroy(&self.videoTexturesRWLock)
    }
}

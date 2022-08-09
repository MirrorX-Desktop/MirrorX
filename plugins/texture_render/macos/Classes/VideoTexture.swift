import CoreVideo
import FlutterMacOS

class VideoTexture: NSObject, FlutterTexture {
    
    var textureID:Int64 = 0
    private var registry: FlutterTextureRegistry
    private var pixelBuffer: Unmanaged<CVPixelBuffer>?
    private var semaphore:DispatchSemaphore = DispatchSemaphore.init(value: 1)
    
    init?(_ registry: FlutterTextureRegistry) {
        self.registry = registry
        
        super.init()
        
        self.textureID = self.registry.register(self)
    }
    
    deinit {
        if self.textureID > 0 {
            self.registry.unregisterTexture(self.textureID)
        }
        
        if self.pixelBuffer != nil {
            self.pixelBuffer!.release()
        }
    }
    
    
    func updateFrame(pixelBufferPointer: UnsafeRawPointer) {
        self.semaphore.wait()
        defer {
            self.semaphore.signal()
        }
        
        if  self.pixelBuffer != nil {
            self.pixelBuffer!.release()
        }
        
        self.pixelBuffer = Unmanaged.fromOpaque(pixelBufferPointer)
        self.registry.textureFrameAvailable(self.textureID)
    }
    
    func copyPixelBuffer() -> Unmanaged<CVPixelBuffer>? {
        self.semaphore.wait()
        defer {
            self.semaphore.signal()
        }
        
        if self.pixelBuffer != nil {
            return self.pixelBuffer;
        }
        
        return nil
    }
}

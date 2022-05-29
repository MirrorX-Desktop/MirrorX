import CoreVideo
import FlutterMacOS

class VideoTexture: NSObject, FlutterTexture {
    
    var registry: FlutterTextureRegistry
    var sem: DispatchSemaphore
    var currentPixelBuffer: Unmanaged<CVPixelBuffer>?
    
    init(registry: FlutterTextureRegistry) {
        self.registry = registry
        self.sem =  DispatchSemaphore.init(value: 0)
    }
    
    func updateFrame(textureID:Int64, pixelBuffer: Unmanaged<CVPixelBuffer>) {
        self.currentPixelBuffer = pixelBuffer
        self.registry.textureFrameAvailable(textureID)
        self.sem.wait()
    }
    
    func copyPixelBuffer() -> Unmanaged<CVPixelBuffer>? {
        defer {
            self.sem.signal()
        }
        
        return self.currentPixelBuffer
    }
}

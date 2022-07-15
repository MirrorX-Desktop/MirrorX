import CoreVideo
import FlutterMacOS

class VideoTexture: NSObject, FlutterTexture {
    
    var registry: FlutterTextureRegistry
    var currentPixelBuffer: Unmanaged<CVPixelBuffer>?
    var semaphore:DispatchSemaphore
    
    init(registry: FlutterTextureRegistry) {
        self.registry = registry
        self.semaphore = DispatchSemaphore.init(value: 1)
    }
    
    func updateFrame(textureID: Int64, pixelBuffer: Unmanaged<CVPixelBuffer>) {
        self.currentPixelBuffer = pixelBuffer
        self.registry.textureFrameAvailable(textureID)
        self.semaphore.wait()
    }
    
    func copyPixelBuffer() -> Unmanaged<CVPixelBuffer>? {
        defer{
            self.semaphore.signal()
        }
        
        return self.currentPixelBuffer
    }
}

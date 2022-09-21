import CoreVideo
import NIOCore
import FlutterMacOS

class VideoTexture: NSObject, FlutterTexture {
    
    var textureID:Int64 = 0
    private var registry: FlutterTextureRegistry
    private var semaphore:DispatchSemaphore = DispatchSemaphore.init(value: 1)
    private var pixelBuffer: Unmanaged<CVPixelBuffer>?
    private var pixelBufferPool:CVPixelBufferPool?
    private var width:Int32 = 0
    private var height:Int32 = 0
    
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
    
    
    func updateFrame(_ width:Int32, _ height:Int32,_ luminaStride:Int32,_ chromaStride:Int32, _ luminaBytebuffer : ByteBuffer,_ chromaByteBuffer : ByteBuffer) {
        
        self.semaphore.wait()
        defer {
            self.semaphore.signal()
        }
        
        if self.pixelBufferPool == nil || self.width != width || self.height != height{
            if self.createPixelBufferPool(width, height) != kCVReturnSuccess{
                return
            }
        }
        
        self.width = width
        self.height = height
        
        var pixelBuffer:CVPixelBuffer?
        if CVPixelBufferPoolCreatePixelBuffer(kCFAllocatorDefault, self.pixelBufferPool!, &pixelBuffer) != kCVReturnSuccess{
            return
        }
        
        guard let pixelBuffer = pixelBuffer else{
            return
        }
        
        CVPixelBufferLockBaseAddress(pixelBuffer, CVPixelBufferLockFlags.init(rawValue: 0))
        
        let luminaBaseAddress = CVPixelBufferGetBaseAddressOfPlane(pixelBuffer, 0)
        let _ = luminaBytebuffer.withVeryUnsafeBytes({luminaByteBufferAddress in
            memcpy(luminaBaseAddress, luminaByteBufferAddress.baseAddress, Int(height * luminaStride))
        })
        
        let chromaBaseAddress = CVPixelBufferGetBaseAddressOfPlane(pixelBuffer, 1)
        let _ = chromaByteBuffer.withVeryUnsafeBytes(   { chromaByteBufferAddress in
            memcpy(chromaBaseAddress, chromaByteBufferAddress.baseAddress, Int(height * chromaStride / 2))
        })
        
        CVPixelBufferUnlockBaseAddress(pixelBuffer, CVPixelBufferLockFlags.init(rawValue: 0))
        
        self.pixelBuffer = Unmanaged.passRetained(pixelBuffer)
        self.registry.textureFrameAvailable(self.textureID)
    }
    
    func copyPixelBuffer() -> Unmanaged<CVPixelBuffer>? {
        self.semaphore.wait()
        defer {
            self.semaphore.signal()
        }
        
        guard let pixelBuffer = self.pixelBuffer else{
            return nil
        }
        
        return pixelBuffer
    }
    
    func createPixelBufferPool(_ width:Int32, _ height:Int32) -> CVReturn{
        
        let attributes:[String:Any] = [
            kCVPixelBufferIOSurfacePropertiesKey as String: [:] ,
            kCVPixelBufferMetalCompatibilityKey as String : true,
            kCVPixelBufferOpenGLCompatibilityKey as String: true,
            kCVPixelBufferPixelFormatTypeKey as String : kCVPixelFormatType_420YpCbCr8BiPlanarFullRange,
            kCVPixelBufferWidthKey as String: Int(width) ,
            kCVPixelBufferHeightKey as String :Int(height),
        ]
        
        return CVPixelBufferPoolCreate(kCFAllocatorDefault, nil,attributes as NSDictionary?, &self.pixelBufferPool)
    }
}

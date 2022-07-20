import CoreVideo
import FlutterMacOS

class VideoTexture: NSObject, FlutterTexture {
    
    var textureID:Int64 = 0
    private var width:UInt = 0
    private var height:UInt = 0
    private var registry: FlutterTextureRegistry
    private var pixelBuffer: CVPixelBuffer?
    private var semaphore:DispatchSemaphore = DispatchSemaphore.init(value: 1)
    private var pixelBufferPool:CVPixelBufferPool?
    
    init?(_ registry: FlutterTextureRegistry) {
        self.registry = registry
        
        super.init()
        
        self.textureID = self.registry.register(self)
    }
    
    deinit {
        if self.textureID > 0 {
            self.registry.unregisterTexture(self.textureID)
        }
        
//        self.semaphore.wait()
//        defer {
//            self.semaphore.signal()
//        }
//
//        self.pixelBuffer?.release()
    }
    
    
    func updateFrame(buffer: UnsafeRawPointer, width:UInt, height:UInt) {
        if self.pixelBufferPool == nil || width != self.width || height != self.height {
            guard let pixelBufferPool = allocCVPixelBufferPool(width, height) else {
                print("realloc CVPixelBufferPool failed")
                return
            }
            
            self.pixelBufferPool = pixelBufferPool;
        }
        
        self.semaphore.wait()
        defer {
            self.semaphore.signal()
        }
        
        var pixelBuffer:CVPixelBuffer?
        let ret = CVPixelBufferPoolCreatePixelBuffer(kCFAllocatorDefault, self.pixelBufferPool!, &pixelBuffer)
        if ret != kCVReturnSuccess {
            print("create CVPixelBuffer failed (\(ret)")
            return
        }
        
        guard let pixelBuffer = pixelBuffer else {
            print("create CVPixelBuffer success but pointer is nil")
            return
        }
        
        CVPixelBufferLockBaseAddress(pixelBuffer, CVPixelBufferLockFlags.init(rawValue: 0))
        
        guard let baseAddress = CVPixelBufferGetBaseAddress(pixelBuffer) else {
            CVPixelBufferUnlockBaseAddress(pixelBuffer, CVPixelBufferLockFlags.init(rawValue: 0))
            return
        }
        
        memcpy(baseAddress, buffer, Int(width * height * 4))
        
        CVPixelBufferUnlockBaseAddress(pixelBuffer, CVPixelBufferLockFlags.init(rawValue: 0))

        self.pixelBuffer = pixelBuffer
        self.registry.textureFrameAvailable(self.textureID)
    }
    
    func copyPixelBuffer() -> Unmanaged<CVPixelBuffer>? {
        self.semaphore.wait()
        defer {
            self.semaphore.signal()
        }
        
        if self.pixelBuffer != nil {
            return Unmanaged.passRetained(self.pixelBuffer!);
        }
        
        return nil
    }
}

private func allocCVPixelBufferPool(_ width:UInt, _ height:UInt) -> CVPixelBufferPool? {
    let pixelBufferAttributes = [
        kCVPixelBufferWidthKey as String:width,
        kCVPixelBufferHeightKey as String:height,
        kCVPixelBufferPixelFormatTypeKey as String: kCVPixelFormatType_32BGRA,
        kCVPixelBufferIOSurfacePropertiesKey as String:[:] as CFDictionary,
//        kCVPixelBufferOpenGLCompatibilityKey as String:true,
//        kCVPixelBufferMetalCompatibilityKey as String:true,
//        kCVPixelBufferIOSurfaceOpenGLTextureCompatibilityKey as String:true
    ] as CFDictionary;
    
    var pixelBufferPool: CVPixelBufferPool?
    
    let ret = CVPixelBufferPoolCreate(kCFAllocatorDefault, nil, pixelBufferAttributes, &pixelBufferPool);
    if ret != kCVReturnSuccess{
        print("create CVPixelBufferPool failed (\(ret))")
        return nil;
    }
    
    return pixelBufferPool;
}

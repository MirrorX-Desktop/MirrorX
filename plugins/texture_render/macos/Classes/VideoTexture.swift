import CoreVideo
import NIOCore
import FlutterMacOS

class VideoTexture: NSObject, FlutterTexture {
    
    var textureId:Int64 = 0
    private var registry: FlutterTextureRegistry
    //    private var nextPixelBuffer: CVPixelBuffer?
    //    private var renderPixelBuffer:CVPixelBuffer?
    
    private var pixelBuffer:Unmanaged<CVPixelBuffer>?
    
    private var pixelBufferPool:CVPixelBufferPool?
    private var width:Int32 = 0
    private var height:Int32 = 0
    private var locker:DispatchSemaphore = DispatchSemaphore(value: 1)
    
    init?(_ registry: FlutterTextureRegistry) {
        self.registry = registry
        
        super.init()
        
        let textureId = self.registry.register(self)
        if textureId == 0 {
            return nil
        }
        self.textureId = textureId
    }
    
    deinit {
        self.pixelBuffer?.release()
        
        if self.textureId > 0 {
            self.registry.unregisterTexture(self.textureId)
        }
    }
    
    
    func updateFrame(_ width:Int32, _ height:Int32,_ luminaStride:Int32,_ chromaStride:Int32, _ luminaBytebuffer : ByteBuffer,_ chromaByteBuffer : ByteBuffer) {
        
        //        if self.pixelBufferPool == nil || self.width != width || self.height != height{
        //            if self.createPixelBufferPool(width, height) != kCVReturnSuccess{
        //                return
        //            }
        //        }
        //
        //        guard let pixelBufferPool = self.pixelBufferPool else {
        //            return
        //        }
        //
        //        var pixelBuffer:CVPixelBuffer?
        //        if CVPixelBufferPoolCreatePixelBuffer(kCFAllocatorDefault, pixelBufferPool, &pixelBuffer) != kCVReturnSuccess{
        //            return
        //        }
        //
        //        guard let pixelBuffer = pixelBuffer else{
        //            return
        //        }
        //
        //        self.width = width
        //        self.height = height
        //
        //        CVPixelBufferLockBaseAddress(pixelBuffer, CVPixelBufferLockFlags.init(rawValue: 0))
        //
        //        let luminaBaseAddress = CVPixelBufferGetBaseAddressOfPlane(pixelBuffer, 0)
        //        let _ = luminaBytebuffer.withUnsafeReadableBytes({luminaByteBufferAddress in
        //            memcpy(luminaBaseAddress, luminaByteBufferAddress.baseAddress, Int(height * luminaStride))
        //        })
        //
        //        let chromaBaseAddress = CVPixelBufferGetBaseAddressOfPlane(pixelBuffer, 1)
        //        let _ = chromaByteBuffer.withUnsafeReadableBytes(   { chromaByteBufferAddress in
        //            memcpy(chromaBaseAddress, chromaByteBufferAddress.baseAddress, Int(height * chromaStride / 2))
        //        })
        //
        //        CVPixelBufferUnlockBaseAddress(pixelBuffer, CVPixelBufferLockFlags.init(rawValue: 0))
        //
        //        if pthread_mutex_trylock(&self.mutex) == 0{
        //            self.nextPixelBuffer = pixelBuffer
        //            pthread_mutex_unlock(&self.mutex)
        //
        //            self.registry.textureFrameAvailable(self.textureId)
        //        }
        
        self.locker.wait()
        
        if self.pixelBuffer != nil && (self.width != width || self.height != height) {
            self.pixelBuffer!.release()
            self.pixelBuffer = nil
        }
        
        if self.pixelBuffer == nil{
            let attributes:[String:Any] = [
                kCVPixelBufferIOSurfacePropertiesKey as String: [:] ,
                kCVPixelBufferMetalCompatibilityKey as String : true,
                kCVPixelBufferOpenGLCompatibilityKey as String: true,
                kCVPixelBufferPixelFormatTypeKey as String : kCVPixelFormatType_420YpCbCr8BiPlanarFullRange,
            ]
            
            var pixelBuffer:CVPixelBuffer?
            
            let ret = CVPixelBufferCreate(kCFAllocatorDefault, Int(width), Int(height), kCVPixelFormatType_420YpCbCr8BiPlanarFullRange, attributes as NSDictionary, &pixelBuffer)
            
            guard let pixelBuffer = pixelBuffer else {
                print("initialize CVPixelBuffer failed")
                return
            }
            
            self.pixelBuffer = Unmanaged.passRetained(pixelBuffer)
            self.width = width
            self.height = height
        }
        
        guard let pixelBuffer = self.pixelBuffer?.takeUnretainedValue() else {
            print("initialize CVPixelBuffer failed")
            return
        }
        
        CVPixelBufferLockBaseAddress(pixelBuffer, CVPixelBufferLockFlags.init(rawValue: 0))
        
        let luminaBaseAddress = CVPixelBufferGetBaseAddressOfPlane(pixelBuffer, 0)
        let _ = luminaBytebuffer.withUnsafeReadableBytes({luminaByteBufferAddress in
            memcpy(luminaBaseAddress, luminaByteBufferAddress.baseAddress, Int(height * luminaStride))
        })
        
        let chromaBaseAddress = CVPixelBufferGetBaseAddressOfPlane(pixelBuffer, 1)
        let _ = chromaByteBuffer.withUnsafeReadableBytes(   { chromaByteBufferAddress in
            memcpy(chromaBaseAddress, chromaByteBufferAddress.baseAddress, Int(height * chromaStride / 2))
        })
        
        CVPixelBufferUnlockBaseAddress(pixelBuffer, CVPixelBufferLockFlags.init(rawValue: 0))
        
        self.locker.signal()
        
        self.registry.textureFrameAvailable(self.textureId)
    }
    
    func copyPixelBuffer() -> Unmanaged<CVPixelBuffer>? {
        self.locker.wait()
        let pixelBuffer = self.pixelBuffer?.retain()
        self.locker.signal()
        
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

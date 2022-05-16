import CoreVideo
import FlutterMacOS

class VideoTexture: NSObject, FlutterTexture {
    
    var pixelBufferPool: CVPixelBufferPool?
    var pixelBufferQueue: CMSimpleQueue
    var registry: FlutterTextureRegistry
    var dispathQueue: DispatchQueue
    var sem:DispatchSemaphore
    var currentPixelBuffer:CVPixelBuffer?
    
    init(registry: FlutterTextureRegistry) {
        self.registry = registry
        
        var cmSimpleQueue: CMSimpleQueue?
        CMSimpleQueueCreate(allocator: kCFAllocatorDefault, capacity: 600, queueOut: &cmSimpleQueue)
        self.pixelBufferQueue = cmSimpleQueue!
        
        self.dispathQueue = DispatchQueue.init(label: "dispatch_queue.texture")
        self.sem=DispatchSemaphore.init(value: 0)
    }
    
    func notifiyTextureUpdate(textureID: Int64, width:UInt16,height:UInt16,isFullColorRange:CBool,yPlaneBufferAddress:UnsafeMutablePointer<UInt8>,yPlaneStride:UInt32,uvPlaneBufferAddress:UnsafeMutablePointer<UInt8>,uvPlaneStride:UInt32,dts:Int64,pts:Int64) {
        
        setupPixelBufferPool(isFullRange:isFullColorRange, width: width, height: height, stride: yPlaneStride)
        
        guard let pixelBufferPool = self.pixelBufferPool else {
            print("pixel buffer pool is nil")
            return;
        }
        
        var pixelBuffer:CVPixelBuffer?
        let ret = CVPixelBufferPoolCreatePixelBuffer(kCFAllocatorDefault, pixelBufferPool, &pixelBuffer)
        if ret != kCVReturnSuccess{
            print("create pixel buffer failed: \(ret)")
        }
        
        CVPixelBufferLockBaseAddress(pixelBuffer!, CVPixelBufferLockFlags.init(rawValue: 0))
        
        let yPlaneBaseAddress = CVPixelBufferGetBaseAddressOfPlane(pixelBuffer!, 0)
        memcpy(yPlaneBaseAddress, yPlaneBufferAddress, Int(yPlaneStride)*Int(height))
        
        let uvPlaneBaseAddress = CVPixelBufferGetBaseAddressOfPlane(pixelBuffer!, 1)
        memcpy(uvPlaneBaseAddress, uvPlaneBufferAddress, Int(uvPlaneStride)*Int(height)/2)
        
        CVPixelBufferUnlockBaseAddress(pixelBuffer!, CVPixelBufferLockFlags.init(rawValue: 0))
        
        self.currentPixelBuffer = pixelBuffer!
        
        self.registry.textureFrameAvailable(textureID)
        self.sem.wait()
    }
    
    func copyPixelBuffer() -> Unmanaged<CVPixelBuffer>? {
        defer{
            self.sem.signal()
        }
        
        if self.currentPixelBuffer != nil{
            return Unmanaged.passRetained(self.currentPixelBuffer!)
        }else{
            return nil
        }
    }
    
    func setupPixelBufferPool(isFullRange: Bool, width: UInt16, height: UInt16, stride: UInt32) {
        if self.pixelBufferPool != nil {
            return
        }
        
        var pixelBufferAttributes: [String: Any] = [:]
        
        if isFullRange {
            pixelBufferAttributes[kCVPixelBufferPixelFormatTypeKey as String] =
            kCVPixelFormatType_420YpCbCr8BiPlanarFullRange
        } else {
            pixelBufferAttributes[kCVPixelBufferPixelFormatTypeKey as String] =
            kCVPixelFormatType_420YpCbCr8BiPlanarVideoRange
        }
        
        pixelBufferAttributes[kCVPixelBufferMetalCompatibilityKey as String] = true
        pixelBufferAttributes[kCVPixelBufferWidthKey as String] = width
        pixelBufferAttributes[kCVPixelBufferHeightKey as String] = height
        pixelBufferAttributes[kCVPixelBufferBytesPerRowAlignmentKey as String] = stride
        
        var pixelBufferPool:CVPixelBufferPool?
        let returnCode = CVPixelBufferPoolCreate(
            kCFAllocatorDefault, nil, pixelBufferAttributes as CFDictionary, &pixelBufferPool)
        
        if returnCode == kCVReturnSuccess  {
            self.pixelBufferPool = pixelBufferPool!
        }else{
            print("setup CVPixelBufferPool failed, return code: \(returnCode)")
        }
    }
}

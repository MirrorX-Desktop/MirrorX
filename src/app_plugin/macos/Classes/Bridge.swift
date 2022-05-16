import CoreMedia
import CoreVideo
import FlutterMacOS
import Foundation

private var globalVideoTextures: [Int64: VideoTexture] = [:]
//private var globalPixelBufferPool

private var globalVideoTexturesLocker = pthread_rwlock_t()
private var globalVideoTexturesLockerInitialized = false

let ioSurfaceProperties = ["IOSurfaceIsGlobal":true as NSNumber]

private let pixelBufferAttributes =
[kCVPixelBufferIOSurfacePropertiesKey as String:[kCVPixelBufferMetalCompatibilityKey as String: true]] as CFDictionary

@_cdecl("dispatch_frame")
func dispatchFrame(
    textureID: Int64, frameID: UInt64, width:UInt16,height:UInt16,isFullColorRange:CBool,yPlaneBufferAddress:UnsafeMutablePointer<UInt8>,yPlaneStride:UInt32,uvPlaneBufferAddress:UnsafeMutablePointer<UInt8>,uvPlaneStride:UInt32,dts:Int64,pts:Int64
) -> CBool {
    makesureGlobalVideoTexturesLockerInitialized()
    
    //    let video_frame_ptr_for_release_callback = UnsafeMutableRawPointer(videoFrame)
    //
    //    let y_plane_buffer_pointer  = UnsafeMutableRawPointer(videoFrame.pointee.y_plane_buffer)
    //    let uv_plane_buffer_pointer = UnsafeMutableRawPointer(videoFrame.pointee.uv_plane_buffer)
    //
    //    var planeBaseAddresses = [y_plane_buffer_pointer, uv_plane_buffer_pointer]
    //    var planeWidths = [Int(videoFrame.pointee.width), Int(videoFrame.pointee.width)]
    //    var planeHeights = [Int(videoFrame.pointee.height),Int (videoFrame.pointee.height / 2)]
    //    var planeStrides = [Int(videoFrame.pointee.y_plane_stride), Int(videoFrame.pointee.uv_plane_stride)]
    //
    //    CVPixelBufferPoolCreate(kCFAllocatorDefault, <#T##poolAttributes: CFDictionary?##CFDictionary?#>, <#T##pixelBufferAttributes: CFDictionary?##CFDictionary?#>, <#T##poolOut: UnsafeMutablePointer<CVPixelBufferPool?>##UnsafeMutablePointer<CVPixelBufferPool?>#>)
    //    cvpixelbuffercreate
    //    var pixelBuffer: CVPixelBuffer?
    //
    //    let ret = CVPixelBufferCreateWithPlanarBytes(
    //        kCFAllocatorDefault,
    //        Int(videoFrame.pointee.width),
    //        Int(videoFrame.pointee.height),
    //        kCVPixelFormatType_420YpCbCr8BiPlanarFullRange,
    //        video_frame_ptr_for_release_callback,
    //        0, 2,
    //        &planeBaseAddresses,
    //        &planeWidths,
    //        &planeHeights,
    //        &planeStrides,
    //        { _, video_frame_pointer, _, _, _ in
    //            if let pointer = video_frame_pointer{
    //                print("release")
    //
    //                let a = UnsafeMutablePointer<VideoFrame>.init(mutating: pointer.assumingMemoryBound(to: VideoFrame.self))
    //                video_frame_destroy(a)
    //
    //            }
    //        }, nil,
    //        pixelBufferAttributes, &pixelBuffer)
    //
    //
    //
    //    if ret != kCVReturnSuccess {
    //        print("create pixel buffer failed, return code: \(ret)")
    //        return false
    //    }
    //
    //    CVBufferSetAttachment(pixelBuffer!, kCVImageBufferColorPrimariesKey,
    //                          kCVImageBufferColorPrimaries_ITU_R_709_2,
    //                          .shouldPropagate)
    //    CVBufferSetAttachment(pixelBuffer!, kCVImageBufferTransferFunctionKey,
    //                          kCVImageBufferTransferFunction_ITU_R_709_2,
    //                          .shouldPropagate)
    //    CVBufferSetAttachment(pixelBuffer!, kCVImageBufferYCbCrMatrixKey,
    //                          kCVImageBufferYCbCrMatrix_ITU_R_709_2,
    //                          .shouldPropagate)
    
    //    print("type: \(CVPixelBufferGetPixelFormatType(pixelBuffer!).description), width: \(CVPixelBufferGetWidth(pixelBuffer!)), height: \(CVPixelBufferGetHeight(pixelBuffer!)), plane1width: \(CVPixelBufferGetWidthOfPlane(pixelBuffer!, 0)), plane1height: \(CVPixelBufferGetHeightOfPlane(pixelBuffer!, 0)), plane2width: \(CVPixelBufferGetWidthOfPlane(pixelBuffer!, 1)), plane2height: \(CVPixelBufferGetHeightOfPlane(pixelBuffer!, 1))")
    
    pthread_rwlock_rdlock(&globalVideoTexturesLocker)
    defer {
        pthread_rwlock_unlock(&globalVideoTexturesLocker)
    }
    
    if let videoTexture = globalVideoTextures[textureID] {
        
        print("notify")
        videoTexture.notifiyTextureUpdate(textureID: textureID, width:width,height:height,isFullColorRange: isFullColorRange,yPlaneBufferAddress: yPlaneBufferAddress,yPlaneStride: yPlaneStride,uvPlaneBufferAddress: uvPlaneBufferAddress,uvPlaneStride:uvPlaneStride,dts: dts,pts: pts)
    }
    
    
    return true
}

func createVideoTexture(with textureRegistry: FlutterTextureRegistry) -> Int64 {
    makesureGlobalVideoTexturesLockerInitialized()
    
    pthread_rwlock_wrlock(&globalVideoTexturesLocker)
    
    let videoTexture = VideoTexture(registry: textureRegistry)
    let textureID = textureRegistry.register(videoTexture)
    
    globalVideoTextures[textureID] = videoTexture
    
    pthread_rwlock_unlock(&globalVideoTexturesLocker)
    
    return textureID
}

func removeVideoTexture(with textureRegistry: FlutterTextureRegistry, textureID: Int64) {
    makesureGlobalVideoTexturesLockerInitialized()
    
    pthread_rwlock_wrlock(&globalVideoTexturesLocker)
    
    globalVideoTextures.removeValue(forKey: textureID)
    
    textureRegistry.unregisterTexture(textureID)
    
    pthread_rwlock_unlock(&globalVideoTexturesLocker)
}

private func makesureGlobalVideoTexturesLockerInitialized() {
    if !globalVideoTexturesLockerInitialized {
        pthread_rwlock_init(&globalVideoTexturesLocker, nil)
        globalVideoTexturesLockerInitialized = true
    }
}

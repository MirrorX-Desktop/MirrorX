import FlutterMacOS
import simd

let vertices: [Float] = [
    -1, -1, 0, 1, // 左下角
     1, -1, 0, 1, // 右下角
     -1, 1, 0, 1, // 左上角
     1, 1, 0, 1, // 右上角
];

let texCoor: [Float] = [
    0, 0, // 左下角
    1, 0, // 右下角
    0, 1, // 左上角
    1, 1, // 右上角
];


let indices: [UInt32] = [
    0, 1, 2,
    1, 3, 2
]

let  cubeVertexData:[Float] = [
    -1.0, -1.0, 0.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0,
     
     // rotation = 90, offset = 16.
     -1.0, -1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 0.0, -1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0,
     
     // rotation = 180, offset = 32.
     -1.0, -1.0, 1.0, 0.0, 1.0, -1.0, 0.0, 0.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0,
     
     // rotation = 270, offset = 48.
     -1.0, -1.0, 0.0, 0.0, 1.0, -1.0, 0.0, 1.0, -1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0,
]

class VideoTextureController: NSObject, FlutterTexture {
    func copyPixelBuffer() -> Unmanaged<CVPixelBuffer>? {
        //        if let pixelBuffer = target?.takeUnretainedValue() {
        //            print("copyPixelBuffer")
        //            return Unmanaged.passRetained(pixelBuffer)
        //        } else {
        //            return nil
        //        }
        if target != nil {
            return Unmanaged.passRetained(target!)
        } else {
            return nil
        }
    }
    
    var frameAvailableClosure:()->Void
    var target: CVPixelBuffer?
    var flutterCVTexture: CVMetalTexture?
    var metalTexture: MTLTexture?
    
    var textureCache: CVMetalTextureCache?
    var device: MTLDevice!
    var commandQueue: MTLCommandQueue!
    var renderTargetDesc: MTLRenderPassDescriptor!
    var renderPipelineState: MTLRenderPipelineState!
    var lumaTexture: MTLTexture!
    var chromaTexture: MTLTexture!
    var indexBuffer: MTLBuffer!
    var _YUV_To_RGB_Matrix: simd_float3x3!
    var _YUV_Translation: simd_float3!
    
    init(frameAvailableClosure: @escaping () -> Void) {
        self.frameAvailableClosure = frameAvailableClosure
        super.init()
        
        self.setMetal()
        self.setRenderTarget()
        self.setRenderPipeline()
        self.setIndexBuffer()
        self.setTextureCache()
        
        if CVPixelBufferCreate(
            kCFAllocatorDefault, 1920, 1080, kCVPixelFormatType_32BGRA,
            [
                kCVPixelBufferMetalCompatibilityKey: true
            ] as CFDictionary, &target) != kCVReturnSuccess{
            
            fatalError("Failed to create CVPixelBuffer")
        }
        
        if CVMetalTextureCacheCreateTextureFromImage(
            kCFAllocatorDefault,
            textureCache!,
            target!,
            //                target!.takeUnretainedValue(),
            nil,
            .bgra8Unorm,
            1920,
            1080,
            0,
            &flutterCVTexture
        ) != kCVReturnSuccess{
            
            fatalError("Failed to bind CVPixelBuffer to metal texture")
        }
        
        self.metalTexture = CVMetalTextureGetTexture(flutterCVTexture!)
    }
    
    func setMetal() {
        device = MTLCreateSystemDefaultDevice()
        commandQueue = device.makeCommandQueue()
    }
    
    func setRenderTarget() {
        renderTargetDesc = MTLRenderPassDescriptor.init()
        renderTargetDesc.colorAttachments[0].loadAction = .clear
        renderTargetDesc.colorAttachments[0].storeAction = .store
        renderTargetDesc.colorAttachments[0].clearColor = MTLClearColorMake(0, 0, 0, 1)
    }
    
    func setRenderPipeline() {
        //        guard let bundle = Bundle.init(identifier: "org.cocopads.video_texture")else{
        //            fatalError("Init Bundle with 'org.cocopads.video_texture' failed")
        //        }
        
        do{
            let bundle = Bundle(for: type(of: self))
            
            let library = try device.makeDefaultLibrary(bundle:bundle)
            
            
            
            let vertexFunc = library.makeFunction(name: "vertexPassthrough")
            let fragmentFunc = library.makeFunction(name: "fragmentColorConversion")
            
            let pipelineDescriptor = MTLRenderPipelineDescriptor.init()
            pipelineDescriptor.label = "MirrorX Video Texture Render Pipeline"
            pipelineDescriptor.vertexFunction = vertexFunc
            pipelineDescriptor.fragmentFunction = fragmentFunc
            pipelineDescriptor.colorAttachments[0].pixelFormat = .bgra8Unorm
            
            renderPipelineState = try device.makeRenderPipelineState(descriptor: pipelineDescriptor)
        } catch {
            fatalError("set Render Pipeline failed:\(error)")
        }
    }
    
    func setIndexBuffer() {
        print("indexbuffer size \(indices.count) \(MemoryLayout.size(ofValue: indices))")
        indexBuffer = device.makeBuffer(
            bytes: cubeVertexData, length: cubeVertexData.count, options: .cpuCacheModeWriteCombined)
    }
    
    func setTextureCache() {
        let ret = CVMetalTextureCacheCreate(nil, nil, device, nil, &textureCache)
        if ret != kCVReturnSuccess {
            print("CVMetalTextureCacheCreate failed with \(ret)")
        }
    }
    
    func render() {
        renderTargetDesc.colorAttachments[0].texture = metalTexture
        
        let commandBuffer = commandQueue.makeCommandBuffer()!
        commandBuffer.label = "Command Buffer"
        commandBuffer.addCompletedHandler { [weak self] cb in
            self?.frameAvailableClosure()
        }
        
        let encoder = commandBuffer.makeRenderCommandEncoder(descriptor: renderTargetDesc)!
        encoder.label = "Render Command Encoder"
        
        //encoder.setViewport(MTLViewport.init(originX: 0, originY: 0, width: 1920, height: 1080, znear: 0, zfar: 1))
        
        encoder.pushDebugGroup("debug group")
        encoder.setRenderPipelineState(renderPipelineState)
        encoder.setVertexBuffer(indexBuffer, offset: 0, index: 0)
        
        //        encoder.setVertexBytes(vertices, length: vertices.count, index: 0)
        //        encoder.setVertexBytes(texCoor, length: texCoor.count, index: 1)
        
        encoder.setFragmentTexture(lumaTexture, index: 0)
        encoder.setFragmentTexture(chromaTexture, index: 1)
        
        //        encoder.setFragmentBytes(&self._YUV_To_RGB_Matrix, length: MemoryLayout.size(ofValue: _YUV_To_RGB_Matrix), index: 0)
        //        encoder.setFragmentBytes(&self._YUV_Translation, length: MemoryLayout.size(ofValue: _YUV_Translation), index: 1)
        //
        //        encoder.drawIndexedPrimitives(type: .triangleStrip, indexCount: 2, indexType: .uint32, indexBuffer: indexBuffer, indexBufferOffset: 0)
        
        encoder.drawPrimitives(type: .triangleStrip, vertexStart: 0, vertexCount: 4, instanceCount: 1)
        encoder.popDebugGroup()
        
        encoder.endEncoding()
        
        commandBuffer.commit()
    }
}

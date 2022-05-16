#include "DuplicationManager.h"

DuplicationManager::DuplicationManager()
    : m_device(nullptr),
      m_device_context(nullptr),
      m_output_duplication(nullptr),
      m_back_texture(nullptr),
      m_input_layout(nullptr),
      m_vertex_shader(nullptr),
      m_vertex_buffer(nullptr),
      m_y_render_texture(nullptr),
      m_y_staging_texture(nullptr),
      m_y_rtv(nullptr),
      m_y_pixel_shader(nullptr),
      m_uv_render_texture(nullptr),
      m_uv_staging_texture(nullptr),
      m_uv_rtv(nullptr),
      m_uv_pixel_shader(nullptr),
      m_should_reinit_back_texture(true) {
  RtlZeroMemory(&m_output_desc, sizeof(m_output_desc));
  RtlZeroMemory(&m_y_viewport, sizeof(m_y_viewport));
  RtlZeroMemory(&m_uv_viewport, sizeof(m_uv_viewport));
}
DuplicationManager::~DuplicationManager() {
  SAFE_RELEASE(m_device)
  SAFE_RELEASE(m_device_context)
  SAFE_RELEASE(m_output_duplication)
  SAFE_RELEASE(m_back_texture)
  SAFE_RELEASE(m_input_layout)
  SAFE_RELEASE(m_vertex_shader)
  SAFE_RELEASE(m_vertex_buffer)

  SAFE_RELEASE(m_y_render_texture)
  SAFE_RELEASE(m_y_staging_texture)
  SAFE_RELEASE(m_y_rtv)
  SAFE_RELEASE(m_y_pixel_shader)

  SAFE_RELEASE(m_uv_render_texture)
  SAFE_RELEASE(m_uv_staging_texture)
  SAFE_RELEASE(m_uv_rtv)
  SAFE_RELEASE(m_uv_pixel_shader)
}

bool DuplicationManager::Init(int display_index) {
  IDXGIOutput1* dxgi_output = nullptr;
  DisplayOutput* display_output;
  HRESULT hr = -1;
  D3D_FEATURE_LEVEL feature_level;
  D3D_FEATURE_LEVEL feature_levels[] = {
      D3D_FEATURE_LEVEL_11_0,
      D3D_FEATURE_LEVEL_10_1,
      D3D_FEATURE_LEVEL_10_0,
      D3D_FEATURE_LEVEL_9_1,
  };
  UINT feature_levels_size = ARRAYSIZE(feature_levels);

  auto display_manager = new DisplayManager();
  auto display_outputs = display_manager->ListDisplays();
  if (display_index >= display_outputs.size()) {
    goto CLEAN;
  }

  display_output = display_outputs[display_index];

  hr = D3D11CreateDevice(display_output->adapter,
                         D3D_DRIVER_TYPE_UNKNOWN,
                         nullptr,
                         D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                         feature_levels,
                         feature_levels_size,
                         D3D11_SDK_VERSION,
                         &m_device,
                         &feature_level,
                         &m_device_context);

  if (FAILED(hr)) {
    ffi_log(FFI_LOG_ERROR, "D3D11CreateDevice failed: %d", hr);
    goto CLEAN;
  }

  ffi_log(FFI_LOG_INFO,
          "D3D11CreateDevice succeeded, feature level:%s",
          GetD3DFeatureLevelName(feature_level));

  hr = display_output->output->GetDesc(&m_output_desc);
  if (FAILED(hr)) {
    ffi_log(FFI_LOG_ERROR, "GetDesc failed: %d", hr);
    goto CLEAN;
  }

  hr = display_output->output->QueryInterface(
      __uuidof(IDXGIOutput1),
      reinterpret_cast<void**>(&dxgi_output));
  if (FAILED(hr)) {
    ffi_log(FFI_LOG_ERROR, "IDXGIOutput QI failed: %d", hr);
    goto CLEAN;
  }

  hr = dxgi_output->DuplicateOutput(m_device, &m_output_duplication);
  if (FAILED(hr)) {
    ffi_log(FFI_LOG_ERROR, "IDXGIOutput create DuplicateOutput failed: %d", hr);
    goto CLEAN;
  }

  init_shaders();

CLEAN:
  SAFE_RELEASE(dxgi_output)
  delete display_manager;
  return SUCCEEDED(hr);
}

void DuplicationManager::CaptureFrame(void* tx, capture_callback cb) {
  if (m_should_reinit_back_texture) {
    init_back_texture();
    m_should_reinit_back_texture = false;
  }

  if (!capture_raw_frame()) {
    return;
  }

  if (!process_frame()) {
    return;
  }

  m_device_context->CopyResource(m_y_staging_texture, m_y_render_texture);
  m_device_context->CopyResource(m_uv_staging_texture, m_uv_render_texture);

  D3D11_MAPPED_SUBRESOURCE y_mapped_resource;
  D3D11_MAPPED_SUBRESOURCE uv_mapped_resource;

  m_device_context->Map(m_y_staging_texture,
                        0,
                        D3D11_MAP_READ,
                        0,
                        &y_mapped_resource);
  m_device_context->Map(m_uv_staging_texture,
                        0,
                        D3D11_MAP_READ,
                        0,
                        &uv_mapped_resource);

  auto width = (m_output_desc.DesktopCoordinates.right -
                m_output_desc.DesktopCoordinates.left);

  auto height = (m_output_desc.DesktopCoordinates.bottom -
                 m_output_desc.DesktopCoordinates.top);

  cb(tx,
     width,
     height,
     y_mapped_resource.RowPitch,
     (uint8_t*)y_mapped_resource.pData,
     uv_mapped_resource.RowPitch,
     (uint8_t*)uv_mapped_resource.pData);

  m_device_context->Unmap(m_y_staging_texture, 0);
  m_device_context->Unmap(m_uv_staging_texture, 0);
}

bool DuplicationManager::init_shaders() {
  UINT g_VS_SIZE = ARRAYSIZE(g_VS);
  HRESULT hr =
      m_device->CreateVertexShader(g_VS, g_VS_SIZE, nullptr, &m_vertex_shader);
  if (FAILED(hr)) {
    ffi_log(FFI_LOG_ERROR, "Failed to create vertex shader: %d", hr);
    return false;
  }

  m_device_context->VSSetShader(m_vertex_shader, nullptr, 0);

  VERTEX Vertices[NUM_VERTICES] = {
      {DirectX::XMFLOAT3(-1.0f, -1.0f, 0), DirectX::XMFLOAT2(0.0f, 1.0f)},
      {DirectX::XMFLOAT3(-1.0f, 1.0f, 0), DirectX::XMFLOAT2(0.0f, 0.0f)},
      {DirectX::XMFLOAT3(1.0f, -1.0f, 0), DirectX::XMFLOAT2(1.0f, 1.0f)},
      {DirectX::XMFLOAT3(1.0f, -1.0f, 0), DirectX::XMFLOAT2(1.0f, 1.0f)},
      {DirectX::XMFLOAT3(-1.0f, 1.0f, 0), DirectX::XMFLOAT2(0.0f, 0.0f)},
      {DirectX::XMFLOAT3(1.0f, 1.0f, 0), DirectX::XMFLOAT2(1.0f, 0.0f)},
  };

  UINT vertex_stride = sizeof(VERTEX);
  UINT vertex_offset = 0;

  D3D11_BUFFER_DESC buffer_desc;
  RtlZeroMemory(&buffer_desc, sizeof(buffer_desc));
  buffer_desc.Usage = D3D11_USAGE_DEFAULT;
  buffer_desc.ByteWidth = sizeof(VERTEX) * NUM_VERTICES;
  buffer_desc.BindFlags = D3D11_BIND_VERTEX_BUFFER;
  buffer_desc.CPUAccessFlags = 0;

  D3D11_SUBRESOURCE_DATA subresource_data;
  RtlZeroMemory(&subresource_data, sizeof(subresource_data));
  subresource_data.pSysMem = Vertices;

  // Create vertex buffer
  hr =
      m_device->CreateBuffer(&buffer_desc, &subresource_data, &m_vertex_buffer);
  if (FAILED(hr)) {
    ffi_log(FFI_LOG_ERROR, "Failed to create vertex buffer: %d", hr);
    return false;
  }

  m_device_context->IASetVertexBuffers(0,
                                       1,
                                       &m_vertex_buffer,
                                       &vertex_stride,
                                       &vertex_offset);

  m_device_context->IASetPrimitiveTopology(
      D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);

  D3D11_INPUT_ELEMENT_DESC input_element_desc[] = {{"POSITION",
                                                    0,
                                                    DXGI_FORMAT_R32G32B32_FLOAT,
                                                    0,
                                                    0,
                                                    D3D11_INPUT_PER_VERTEX_DATA,
                                                    0},
                                                   {"TEXCOORD",
                                                    0,
                                                    DXGI_FORMAT_R32G32_FLOAT,
                                                    0,
                                                    12,
                                                    D3D11_INPUT_PER_VERTEX_DATA,
                                                    0}};

  UINT input_element_desc_size = ARRAYSIZE(input_element_desc);
  hr = m_device->CreateInputLayout(input_element_desc,
                                   input_element_desc_size,
                                   g_VS,
                                   g_VS_SIZE,
                                   &m_input_layout);
  if (FAILED(hr)) {
    ffi_log(FFI_LOG_ERROR, "Failed to create input layout: %d", hr);
    return false;
  }

  m_device_context->IASetInputLayout(m_input_layout);

  UINT pixel_shader_y_size = ARRAYSIZE(g_PS_Y);
  hr = m_device->CreatePixelShader(g_PS_Y,
                                   pixel_shader_y_size,
                                   nullptr,
                                   &m_y_pixel_shader);
  if (FAILED(hr)) {
    ffi_log(FFI_LOG_ERROR, "Failed to create y pixel shader: %d", hr);
    return false;
  }

  UINT pixel_shader_uv_size = ARRAYSIZE(g_PS_UV);
  hr = m_device->CreatePixelShader(g_PS_UV,
                                   pixel_shader_uv_size,
                                   nullptr,
                                   &m_uv_pixel_shader);
  if (FAILED(hr)) {
    ffi_log(FFI_LOG_ERROR, "Failed to create uv pixel shader: %d", hr);
    return false;
  }

  return true;
}

bool DuplicationManager::init_back_texture() {
  release_textures();

  DXGI_OUTDUPL_DESC out_duplication_desc;
  m_output_duplication->GetDesc(&out_duplication_desc);

  D3D11_TEXTURE2D_DESC back_texture_desc;
  RtlZeroMemory(&back_texture_desc, sizeof(D3D11_TEXTURE2D_DESC));
  back_texture_desc.Width = out_duplication_desc.ModeDesc.Width;
  back_texture_desc.Height = out_duplication_desc.ModeDesc.Height;
  back_texture_desc.MipLevels = 1;
  back_texture_desc.ArraySize = 1;
  back_texture_desc.Format = out_duplication_desc.ModeDesc.Format;
  back_texture_desc.SampleDesc.Count = 1;
  back_texture_desc.Usage = D3D11_USAGE_DEFAULT;
  back_texture_desc.BindFlags = D3D11_BIND_SHADER_RESOURCE;

  HRESULT hr =
      m_device->CreateTexture2D(&back_texture_desc, nullptr, &m_back_texture);
  if (FAILED(hr) || !m_back_texture) {
    ffi_log(FFI_LOG_ERROR, "Failed to create back texture: %d", hr);
    return false;
  }

  // create y render texture
  back_texture_desc.Format = DXGI_FORMAT_R8_UNORM;
  back_texture_desc.BindFlags = D3D11_BIND_RENDER_TARGET;
  hr = m_device->CreateTexture2D(&back_texture_desc,
                                 nullptr,
                                 &m_y_render_texture);
  if (FAILED(hr) || !m_y_render_texture) {
    ffi_log(FFI_LOG_ERROR, "Failed to create y render texture: %d", hr);
    return false;
  }

  // create y staging texture
  back_texture_desc.CPUAccessFlags = D3D11_CPU_ACCESS_READ;
  back_texture_desc.Usage = D3D11_USAGE_STAGING;
  back_texture_desc.BindFlags = 0;
  hr = m_device->CreateTexture2D(&back_texture_desc,
                                 nullptr,
                                 &m_y_staging_texture);
  if (FAILED(hr) || !m_y_staging_texture) {
    ffi_log(FFI_LOG_ERROR, "Failed to create y staging texture: %d", hr);
    return false;
  }

  set_viewport(&m_y_viewport,
               back_texture_desc.Width,
               back_texture_desc.Height);
  if (!make_rtv(&m_y_rtv, m_y_render_texture)) {
    return false;
  }

  // create uv render texture
  back_texture_desc.Width = out_duplication_desc.ModeDesc.Width / 2;
  back_texture_desc.Height = out_duplication_desc.ModeDesc.Height / 2;
  back_texture_desc.Format = DXGI_FORMAT_R8G8_UNORM;
  back_texture_desc.Usage = D3D11_USAGE_DEFAULT;
  back_texture_desc.CPUAccessFlags = 0;
  back_texture_desc.BindFlags = D3D11_BIND_RENDER_TARGET;

  hr = m_device->CreateTexture2D(&back_texture_desc,
                                 nullptr,
                                 &m_uv_render_texture);
  if (FAILED(hr) || !m_uv_render_texture) {
    ffi_log(FFI_LOG_ERROR, "Failed to create uv render texture: %d", hr);
    return false;
  }

  // create uv staging texture
  back_texture_desc.CPUAccessFlags = D3D11_CPU_ACCESS_READ;
  back_texture_desc.Usage = D3D11_USAGE_STAGING;
  back_texture_desc.BindFlags = 0;
  hr = m_device->CreateTexture2D(&back_texture_desc,
                                 nullptr,
                                 &m_uv_staging_texture);
  if (FAILED(hr) || !m_uv_staging_texture) {
    ffi_log(FFI_LOG_ERROR, "Failed to create uv staging texture: %d", hr);
    return false;
  }

  set_viewport(&m_uv_viewport,
               back_texture_desc.Width,
               back_texture_desc.Height);
  return make_rtv(&m_uv_rtv, m_uv_render_texture);
}

void DuplicationManager::set_viewport(D3D11_VIEWPORT* viewport,
                                      size_t width,
                                      size_t height) {
  viewport->TopLeftX = 0;
  viewport->TopLeftY = 0;
  viewport->Width = static_cast<FLOAT>(width);
  viewport->Height = static_cast<FLOAT>(height);
  viewport->MinDepth = 0.0f;
  viewport->MaxDepth = 1.0f;
}

bool DuplicationManager::make_rtv(ID3D11RenderTargetView** render_target_view,
                                  ID3D11Texture2D* texture) {
  SAFE_RELEASE(*render_target_view)
  HRESULT hr =
      m_device->CreateRenderTargetView(texture, nullptr, render_target_view);

  if (FAILED(hr)) {
    ffi_log(FFI_LOG_ERROR, "Failed to create render target view: %d", hr);
  }

  return SUCCEEDED(hr);
}

bool DuplicationManager::capture_raw_frame() {
  IDXGIResource* desktop_resource = nullptr;
  DXGI_OUTDUPL_FRAME_INFO frame_info;
  HRESULT hr =
      m_output_duplication->AcquireNextFrame(0, &frame_info, &desktop_resource);
  if (FAILED(hr)) {
    // if there's no desktop picture changed, it will fail
    return false;
  }

  ID3D11Texture2D* acquired_desktop_image = nullptr;
  hr = desktop_resource->QueryInterface(
      __uuidof(ID3D11Texture2D),
      reinterpret_cast<void**>(&acquired_desktop_image));
  SAFE_RELEASE(desktop_resource)

  if (FAILED(hr)) {
    ffi_log(FFI_LOG_ERROR, "Failed to get desktop image: %d", hr);
    return false;
  }

  m_device_context->CopyResource(m_back_texture, acquired_desktop_image);
  SAFE_RELEASE(acquired_desktop_image)

  m_output_duplication->ReleaseFrame();
  return true;
}

bool DuplicationManager::process_frame() {
  D3D11_TEXTURE2D_DESC shader_texture_desc;
  m_back_texture->GetDesc(&shader_texture_desc);

  D3D11_SHADER_RESOURCE_VIEW_DESC shader_resource_view_desc;
  shader_resource_view_desc.Format = shader_texture_desc.Format;
  shader_resource_view_desc.ViewDimension = D3D11_SRV_DIMENSION_TEXTURE2D;
  shader_resource_view_desc.Texture2D.MostDetailedMip =
      shader_texture_desc.MipLevels - 1;
  shader_resource_view_desc.Texture2D.MipLevels = shader_texture_desc.MipLevels;

  ID3D11ShaderResourceView* shader_resource_view = nullptr;
  HRESULT hr = m_device->CreateShaderResourceView(m_back_texture,
                                                  &shader_resource_view_desc,
                                                  &shader_resource_view);
  if (FAILED(hr)) {
    ffi_log(FFI_LOG_ERROR, "Failed to create shader resource view: %d", hr);
    return false;
  }

  m_device_context->PSSetShaderResources(0, 1, &shader_resource_view);

  m_device_context->OMSetRenderTargets(1, &m_y_rtv, nullptr);
  m_device_context->PSSetShader(m_y_pixel_shader, nullptr, 0);
  m_device_context->RSSetViewports(1, &m_y_viewport);
  m_device_context->Draw(NUM_VERTICES, 0);

  m_device_context->OMSetRenderTargets(1, &m_uv_rtv, nullptr);
  m_device_context->PSSetShader(m_uv_pixel_shader, nullptr, 0);
  m_device_context->RSSetViewports(1, &m_uv_viewport);
  m_device_context->Draw(NUM_VERTICES, 0);

  shader_resource_view->Release();
  shader_resource_view = nullptr;

  SAFE_RELEASE(shader_resource_view)

  return true;
}

void DuplicationManager::release_textures() {
  SAFE_RELEASE(m_back_texture)
  SAFE_RELEASE(m_y_render_texture)
  SAFE_RELEASE(m_y_staging_texture)
  SAFE_RELEASE(m_uv_render_texture)
  SAFE_RELEASE(m_uv_staging_texture)
}
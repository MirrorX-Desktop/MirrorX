#ifndef DUPLICATION_MGR_H
#define DUPLICATION_MGR_H

#include "DisplayManager.h"
#include "common.h"
#include "shaders/pixel_shader_uv.h"
#include "shaders/pixel_shader_y.h"
#include "shaders/vertex_shader.h"

typedef struct VERTEX {
  DirectX::XMFLOAT3 Pos;
  DirectX::XMFLOAT2 TexCoord;
} VERTEX;

class DuplicationManager {
 public:
  DuplicationManager();
  ~DuplicationManager();
  bool Init(int display_index);
  void CaptureFrame(void* tx, capture_callback cb);

 private:
  ID3D11Device* m_device;
  ID3D11DeviceContext* m_device_context;
  IDXGIOutputDuplication* m_output_duplication;
  DXGI_OUTPUT_DESC m_output_desc;
  ID3D11Texture2D* m_back_texture;
  ID3D11InputLayout* m_input_layout;
  ID3D11VertexShader* m_vertex_shader;
  ID3D11Buffer* m_vertex_buffer;

  D3D11_VIEWPORT m_y_viewport;
  ID3D11Texture2D* m_y_render_texture;
  ID3D11Texture2D* m_y_staging_texture;
  ID3D11RenderTargetView* m_y_rtv;
  ID3D11PixelShader* m_y_pixel_shader;

  D3D11_VIEWPORT m_uv_viewport;
  ID3D11Texture2D* m_uv_render_texture;
  ID3D11Texture2D* m_uv_staging_texture;
  ID3D11RenderTargetView* m_uv_rtv;
  ID3D11PixelShader* m_uv_pixel_shader;

  bool m_should_reinit_back_texture;

  bool init_shaders();
  bool init_back_texture();
  static void set_viewport(D3D11_VIEWPORT* viewport,
                           size_t width,
                           size_t height);
  bool make_rtv(ID3D11RenderTargetView** render_target_view,
                ID3D11Texture2D* texture);
  bool capture_raw_frame();
  bool process_frame();
  void release_textures();
};

#endif  // DUPLICATION_MGR_H

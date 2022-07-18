echo "Make sure you are running this script from Visual Studio Developer Command Prompt enviornment"
fxc /E VS /T vs_4_0 /Fo vertex_shader.cso VertexShader.hlsl
fxc /E PS /T ps_4_0 /Fo pixel_shader.cso PixelShader.hlsl
fxc /E PS_Y /T ps_4_0 /Fo pixel_shader_y.cso PixelShaderY.hlsl
fxc /E PS_UV /T ps_4_0 /Fo pixel_shader_uv.cso PixelShaderUV.hlsl
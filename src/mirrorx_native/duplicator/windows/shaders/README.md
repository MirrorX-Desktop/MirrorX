how to compile current directory shader files.

"C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64\fxc.
exe" /E VS /T vs_4_0 /Fh m_vertex_shader.h VertexShader.hlsl

/E VS means shader entry point

/T vs_4_0_level_9_1 means shader profile
(derived from https://docs.microsoft.com/en-us/windows/win32/direct3dtools/dx-graphics-tools-fxc-syntax)


/Fh m_vertex_shader.h means output
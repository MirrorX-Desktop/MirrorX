use windows::{
    core::Interface,
    Win32::Graphics::{Direct3D::*, Direct3D10::*, Direct3D11::*},
};

struct DXContext {
    device: ID3D11Device,
    device_context: ID3D11DeviceContext,
    video_device: ID3D11VideoDevice,
    video_context: ID3D11VideoContext,
}

impl DXContext {
    pub fn new() -> anyhow::Result<Self> {
        unsafe {
            let feature_levels = [
                D3D_FEATURE_LEVEL_12_2,
                D3D_FEATURE_LEVEL_12_1,
                D3D_FEATURE_LEVEL_12_0,
                D3D_FEATURE_LEVEL_11_1,
                D3D_FEATURE_LEVEL_11_0,
                D3D_FEATURE_LEVEL_10_1,
                D3D_FEATURE_LEVEL_10_0,
            ];

            let device = {
                let mut device = None;

                if let Err(err) = D3D11CreateDevice(
                    None,
                    D3D_DRIVER_TYPE_HARDWARE,
                    None,
                    D3D11_CREATE_DEVICE_VIDEO_SUPPORT | D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                    &feature_levels,
                    D3D11_SDK_VERSION,
                    &mut device,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ) {
                    return Err(anyhow::anyhow!(err));
                };

                match device {
                    Some(device) => device,
                    None => return Err(anyhow::anyhow!("create id3d11device failed")),
                }
            };

            let multithread: ID3D10Multithread =
                device.cast().map_err(|err| anyhow::anyhow!(err))?;
            multithread.SetMultithreadProtected(true);

            let device_context = {
                let mut device_context = None;
                device.GetImmediateContext(&mut device_context);

                match device_context {
                    Some(device_context) => device_context,
                    None => return Err(anyhow::anyhow!("create device context failed")),
                }
            };

            let video_device: ID3D11VideoDevice =
                device.cast().map_err(|err| anyhow::anyhow!(err))?;

            let video_context: ID3D11VideoContext =
                device_context.cast().map_err(|err| anyhow::anyhow!(err))?;

            Ok(DXContext {
                device,
                device_context,
                video_device,
                video_context,
            })
        }
    }
}

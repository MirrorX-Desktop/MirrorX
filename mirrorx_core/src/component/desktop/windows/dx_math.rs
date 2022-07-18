#[repr(C)]
#[derive(Clone)]
pub struct XMFLOAT2 {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Clone)]
pub struct XMFLOAT3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Clone)]
pub struct VERTEX {
    pub pos: XMFLOAT3,
    pub tex_coord: XMFLOAT2,
}

pub static VERTEX_STRIDES: u32 = std::mem::size_of::<VERTEX>() as u32;

pub static VERTICES: [VERTEX; 6] = [
    VERTEX {
        pos: XMFLOAT3 {
            x: -1.0,
            y: -1.0,
            z: 0.0,
        },
        tex_coord: XMFLOAT2 { x: 0.0, y: 1.0 },
    },
    VERTEX {
        pos: XMFLOAT3 {
            x: -1.0,
            y: 1.0,
            z: 0.0,
        },
        tex_coord: XMFLOAT2 { x: 0.0, y: 0.0 },
    },
    VERTEX {
        pos: XMFLOAT3 {
            x: 1.0,
            y: -1.0,
            z: 0.0,
        },
        tex_coord: XMFLOAT2 { x: 1.0, y: 1.0 },
    },
    VERTEX {
        pos: XMFLOAT3 {
            x: 1.0,
            y: -1.0,
            z: 0.0,
        },
        tex_coord: XMFLOAT2 { x: 1.0, y: 1.0 },
    },
    VERTEX {
        pos: XMFLOAT3 {
            x: -1.0,
            y: 1.0,
            z: 0.0,
        },
        tex_coord: XMFLOAT2 { x: 0.0, y: 0.0 },
    },
    VERTEX {
        pos: XMFLOAT3 {
            x: 1.0,
            y: 1.0,
            z: 0.0,
        },
        tex_coord: XMFLOAT2 { x: 1.0, y: 0.0 },
    },
];

pub const BPP: u32 = 4;

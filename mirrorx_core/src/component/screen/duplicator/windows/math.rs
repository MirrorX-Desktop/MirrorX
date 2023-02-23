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
pub struct Vertex {
    pub pos: XMFLOAT3,
    pub tex_coord: XMFLOAT2,
}

pub static VERTEX_STRIDES: u32 = std::mem::size_of::<Vertex>() as u32;

pub static VERTICES: [Vertex; 6] = [
    Vertex {
        pos: XMFLOAT3 {
            x: -1.0,
            y: -1.0,
            z: 0.0,
        },
        tex_coord: XMFLOAT2 { x: 0.0, y: 1.0 },
    },
    Vertex {
        pos: XMFLOAT3 {
            x: -1.0,
            y: 1.0,
            z: 0.0,
        },
        tex_coord: XMFLOAT2 { x: 0.0, y: 0.0 },
    },
    Vertex {
        pos: XMFLOAT3 {
            x: 1.0,
            y: -1.0,
            z: 0.0,
        },
        tex_coord: XMFLOAT2 { x: 1.0, y: 1.0 },
    },
    Vertex {
        pos: XMFLOAT3 {
            x: 1.0,
            y: -1.0,
            z: 0.0,
        },
        tex_coord: XMFLOAT2 { x: 1.0, y: 1.0 },
    },
    Vertex {
        pos: XMFLOAT3 {
            x: -1.0,
            y: 1.0,
            z: 0.0,
        },
        tex_coord: XMFLOAT2 { x: 0.0, y: 0.0 },
    },
    Vertex {
        pos: XMFLOAT3 {
            x: 1.0,
            y: 1.0,
            z: 0.0,
        },
        tex_coord: XMFLOAT2 { x: 1.0, y: 0.0 },
    },
];

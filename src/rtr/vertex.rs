use super::*;

#[derive(Debug, Clone)]
#[repr(C, packed)]
pub struct Vertex {
    pub xyz: Vec3,
    pub rgba: Vec4,
    pub uv: Vec2,
}

#[derive(Default, Debug)]
pub struct VertexBufCPU {
    pub verts: Vec<Vertex>,
    pub inds: Vec<u32>,
}

impl VertexBufCPU {
    pub fn extend(&mut self, verts: impl Iterator<Item = Vertex>, inds: impl Iterator<Item = u32>) {
        let offset = self.verts.len() as u32;
        self.verts.extend(verts);
        self.inds.extend(inds.map(|ind| ind + offset))
    }
}
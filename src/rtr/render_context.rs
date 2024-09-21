use super::*;
use glow::Texture as GPUTexture;
use glow::Context as GLContext;
use glow::*;
use std::collections::HashMap;

use super::*;

pub struct RenderContext {
    pub gl: GLContext,
    pub wh: IVec2,
}

impl RenderContext {
    pub fn new(gl: GLContext) -> Self {
        unsafe {

        }
    }

    pub fn draw_frame(&self) {
        
    }
}
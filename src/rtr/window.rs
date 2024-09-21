use super::*;

use glutin::{
    config::{ConfigTemplateBuilder, GlConfig},
    context::{ContextApi, ContextAttributesBuilder, NotCurrentGlContext, PossiblyCurrentContext},
    display::{GetGlDisplay, GlDisplay},
    surface::{GlSurface, SwapInterval},
};
use winit::window::Window as WindowInner;
use glutin_winit::{DisplayBuilder, GlWindow};
use raw_window_handle::HasRawWindowHandle;
use std::num::NonZeroU32;

pub struct Window {
    pub window: WindowInner,
    pub gl: PossiblyCurrentContext,
}

impl Window {
    pub fn new(event_loop: &winit::event_loop::EventLoop<()>) -> Self {
        unsafe {
            let window_builder = winit::window::WindowBuilder::new()
            .with_title("Hello triangle!")
            .with_inner_size(winit::dpi::LogicalSize::new(1024.0, 768.0));
    
            let template = ConfigTemplateBuilder::new();
    
            let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));
    
            let (window, gl_config) = display_builder
                .build(&event_loop, template, |configs| {
                    configs
                        .reduce(|accum, config| {
                            if config.num_samples() > accum.num_samples() {
                                config
                            } else {
                                accum
                            }
                        })
                        .unwrap()
                })
                .unwrap();
    
            let raw_window_handle = window.as_ref().map(|window| window.raw_window_handle());
    
            let gl_display = gl_config.display();
            let context_attributes = ContextAttributesBuilder::new()
                .with_context_api(ContextApi::OpenGl(Some(glutin::context::Version {
                    major: 4,
                    minor: 1,
                })))
                .build(raw_window_handle);
    
            let not_current_gl_context = gl_display
                .create_context(&gl_config, &context_attributes)
                .unwrap();
    
            let window = window.unwrap();
    
            let attrs = window.build_surface_attributes(Default::default());
            let gl_surface = gl_display
                .create_window_surface(&gl_config, &attrs)
                .unwrap();
    
            let gl = not_current_gl_context.make_current(&gl_surface).unwrap();

            gl_surface
                .set_swap_interval(&gl, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
                .unwrap();

            Self {
                window,
                gl,
            }

        }
    }
}